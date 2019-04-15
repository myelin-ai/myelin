use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{Client, ThreadSpawnFn, WebsocketConnectionAcceptor};
use crate::constant::*;
use crate::controller::{ConnectionAcceptor, Controller, ControllerImpl};
use crate::fixed_interval_sleeper::FixedIntervalSleeperImpl;
use crate::presenter::DeltaPresenter;
use myelin_engine::prelude::*;
use myelin_engine::simulation::SimulationBuilder;
use myelin_genetics::deriver::ChromosomalCrossoverGenomeDeriver;
use myelin_genetics::developer::FlatNeuralNetworkDeveloper;
use myelin_genetics::genome::Genome;
use myelin_genetics::mutator::GenomeMutatorStub;
use myelin_genetics::orchestrator_impl::{
    NeuralNetworkConfiguratorFactory, NeuralNetworkConfiguratorImpl, NeuralNetworkDeveloperFactory,
    NeuralNetworkDevelopmentOrchestratorImpl, NeuralNetworkFactory,
};
use myelin_neural_network::spiking_neural_network::DefaultSpikingNeuralNetwork;
use myelin_object_behavior::organism::OrganismBehavior;
use myelin_object_behavior::stochastic_spreading::StochasticSpreading;
use myelin_object_behavior::Static;
use myelin_object_data::Kind;
use myelin_object_data::{
    AdditionalObjectDescriptionBincodeDeserializer, AdditionalObjectDescriptionBincodeSerializer,
};
use myelin_random::RandomImpl;
use myelin_visualization_core::serialization::BincodeSerializer;
use myelin_worldgen::NameProviderBuilder;
use myelin_worldgen::{HardcodedGenerator, WorldGenerator};
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

/// Starts the simulation and a websocket server, that broadcasts
/// `ViewModel`s on each step to all clients.
pub fn start_server<A>(addr: A)
where
    A: Into<SocketAddr> + Send,
{
    let addr = addr.into();

    let simulation_factory = box || -> Box<dyn Simulation> { SimulationBuilder::new().build() };
    let plant_factory = box || -> Box<dyn ObjectBehavior> {
        box StochasticSpreading::new(1.0 / 5_000.0, box RandomImpl::new())
    };
    let organism_factory = box || -> Box<dyn ObjectBehavior> {
        let neural_network_factory: Rc<NeuralNetworkFactory> =
            Rc::new(|| box DefaultSpikingNeuralNetwork::new());
        let neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory> =
            Rc::new(|configuration, _| {
                box FlatNeuralNetworkDeveloper::new(configuration, box RandomImpl::new())
            });
        let neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory> = Rc::new(
            |neural_network, input_neural_handles, output_neuron_handles| {
                box NeuralNetworkConfiguratorImpl::new(
                    neural_network,
                    input_neural_handles,
                    output_neuron_handles,
                )
            },
        );
        box OrganismBehavior::new(
            (Genome::default(), Genome::default()),
            box NeuralNetworkDevelopmentOrchestratorImpl::new(
                neural_network_factory,
                neural_network_developer_factory,
                neural_network_configurator_factory,
                box ChromosomalCrossoverGenomeDeriver::new(box RandomImpl::new()),
                box GenomeMutatorStub::new(),
            ),
            box AdditionalObjectDescriptionBincodeDeserializer::new(),
        )
    };
    let terrain_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
    let water_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };

    let mut name_provider_builder = NameProviderBuilder::default();

    let organism_names = load_names_from_file(Path::new("./object-names/organisms.txt"));
    name_provider_builder.add_names(&organism_names, Kind::Organism);

    let name_provider = name_provider_builder.build_randomized();

    let additional_object_description_serializer =
        box AdditionalObjectDescriptionBincodeSerializer::default();

    let mut worldgen = HardcodedGenerator::new(
        simulation_factory,
        plant_factory,
        organism_factory,
        terrain_factory,
        water_factory,
        name_provider,
        additional_object_description_serializer,
    );

    let conection_acceptor_factory_fn = Arc::new(move |current_snapshot_fn| {
        let client_factory_fn = Arc::new(|websocket_client, current_snapshot_fn| {
            let interval = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);
            let fixed_interval_sleeper = FixedIntervalSleeperImpl::default();
            let presenter = DeltaPresenter::default();
            let view_model_serializer = BincodeSerializer::default();

            let connection = Connection {
                id: Uuid::new_v4(),
                socket: box WebsocketClient::new(websocket_client),
            };

            box ClientHandler::new(
                interval,
                box fixed_interval_sleeper,
                box presenter,
                box view_model_serializer,
                connection,
                current_snapshot_fn,
            ) as Box<dyn Client>
        });

        box WebsocketConnectionAcceptor::try_new(
            addr,
            client_factory_fn,
            spawn_thread_factory(),
            current_snapshot_fn,
        )
        .expect("Failed to create websocket connection acceptor")
            as Box<dyn ConnectionAcceptor>
    });

    let expected_delta = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);

    let mut controller = ControllerImpl::new(
        worldgen.generate(),
        conection_acceptor_factory_fn,
        spawn_thread_factory(),
        expected_delta,
    );

    controller.run();
}

fn load_names_from_file(path: &Path) -> Vec<String> {
    read_to_string(path)
        .expect("Error while reading file")
        .lines()
        .map(String::from)
        .collect()
}

fn spawn_thread_factory() -> Box<ThreadSpawnFn> {
    box move |function| {
        thread::spawn(function);
    }
}
