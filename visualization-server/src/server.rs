use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{
    Client, ClientFactoryFn, ThreadSpawnFn, WebsocketConnectionAcceptor,
};
use crate::constant::*;
use crate::controller::{
    ConnectionAcceptor, ConnectionAcceptorFactoryFn, Controller, ControllerImpl, CurrentSnapshotFn,
    Presenter,
};
use crate::fixed_interval_sleeper::{FixedIntervalSleeper, FixedIntervalSleeperImpl};
use crate::presenter::DeltaPresenter;
use myelin_engine::prelude::*;
use myelin_engine::simulation::SimulationBuilder;
use myelin_genetics::genome::Genome;
use myelin_genetics::neural_network_development_orchestrator_impl::{
    ChromosomalCrossoverGenomeDeriver, FlatNeuralNetworkDeveloper, GenomeDeriver, GenomeMutator,
    GenomeMutatorStub, InputNeuronHandles, NeuralNetworkConfigurator,
    NeuralNetworkConfiguratorFactory, NeuralNetworkConfiguratorImpl, NeuralNetworkDeveloper,
    NeuralNetworkDeveloperFactory, NeuralNetworkDevelopmentOrchestratorImpl, NeuralNetworkFactory,
    OutputNeuronHandles,
};
use myelin_genetics::{
    NeuralNetworkDevelopmentConfiguration, NeuralNetworkDevelopmentOrchestrator,
};
use myelin_neural_network::spiking_neural_network::DefaultSpikingNeuralNetwork;
use myelin_neural_network::NeuralNetwork;
use myelin_object_behavior::organism::OrganismBehavior;
use myelin_object_behavior::stochastic_spreading::StochasticSpreading;
use myelin_object_behavior::Static;
use myelin_object_data::Kind;
use myelin_object_data::{
    AdditionalObjectDescriptionBincodeDeserializer, AdditionalObjectDescriptionBincodeSerializer,
    AdditionalObjectDescriptionSerializer,
};
use myelin_random::{Random, RandomImpl};
use myelin_visualization_core::serialization::{BincodeSerializer, ViewModelSerializer};
use myelin_worldgen::{HardcodedGenerator, NameProvider, NameProviderFactory, WorldGenerator};
use myelin_worldgen::{NameProviderBuilder, ShuffledNameProviderFactory};
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use wonderbox::{register_autoresolvable, Container};

/// Starts the simulation and a websocket server, that broadcasts
/// `ViewModel`s on each step to all clients.
pub fn start_server<A>(addr: A)
where
    A: Into<SocketAddr> + Send,
{
    let container = create_composition_root(addr.into());
    let mut controller = container.resolve::<Box<dyn Controller>>().unwrap();
    println!("running");

    controller.run();
}

fn create_composition_root(addr: SocketAddr) -> Container {
    let mut container = Container::new();

    register_autoresolvable!(container, RandomImpl as Box<dyn Random>);

    register_autoresolvable!(
        container,
        ShuffledNameProviderFactory as Box<dyn NameProviderFactory>
    );

    register_autoresolvable!(
        container,
        AdditionalObjectDescriptionBincodeSerializer
            as Box<dyn AdditionalObjectDescriptionSerializer>
    );

    register_autoresolvable!(
        container,
        ChromosomalCrossoverGenomeDeriver as Box<dyn GenomeDeriver>
    );

    register_autoresolvable!(container, GenomeMutatorStub as Box<dyn GenomeMutator>);

    register_autoresolvable!(
        container,
        NeuralNetworkDevelopmentOrchestratorImpl as Box<dyn NeuralNetworkDevelopmentOrchestrator>
    );

    container
        .register(move |_| addr.clone())
        .register(|_| SimulationBuilder::new().build())
        .register(|_| {
            Rc::new(|| box DefaultSpikingNeuralNetwork::new() as Box<dyn NeuralNetwork>)
                as Rc<dyn Fn() -> Box<dyn NeuralNetwork>>
        })
        .register(|_| {
            box (|function| {
                thread::spawn(function);
            }) as Box<ThreadSpawnFn>
        })
        .register(|container| {
            fn neural_network_developer_factory_factory<'a>(
                container: &'a Container,
            ) -> impl for<'b> Fn(
                &'b NeuralNetworkDevelopmentConfiguration,
                &'b Genome,
            ) -> Box<dyn NeuralNetworkDeveloper + 'b> {
                let container = container.clone();
                move |configuration, _| {
                    let random = container.resolve::<Box<dyn Random>>().unwrap();
                    box FlatNeuralNetworkDeveloper::new(configuration, random)
                        as Box<dyn NeuralNetworkDeveloper>
                }
            }
            Rc::new(neural_network_developer_factory_factory(container))
                as Rc<NeuralNetworkDeveloperFactory>
        })
        .register(|container| {
            let name_provider_factory =
                container.resolve::<Box<dyn NameProviderFactory>>().unwrap();
            let mut name_provider_builder = NameProviderBuilder::new(name_provider_factory);
            let organism_names = load_names_from_file(Path::new("./object-names/organisms.txt"));
            name_provider_builder.add_names(&organism_names, Kind::Organism);
            name_provider_builder.build()
        })
        .register(|_| {
            fn neural_network_configurator_factory<'a>(
                neural_network: &'a mut dyn NeuralNetwork,
                input_neural_handles: &'a mut InputNeuronHandles,
                output_neuron_handles: &'a mut OutputNeuronHandles,
            ) -> Box<dyn NeuralNetworkConfigurator + 'a> {
                box NeuralNetworkConfiguratorImpl::new(
                    neural_network,
                    input_neural_handles,
                    output_neuron_handles,
                )
            }
            Rc::new(neural_network_configurator_factory) as Rc<NeuralNetworkConfiguratorFactory>
        })
        .register(|container| {
            let plant_factory = {
                let container = container.clone();
                box move || -> Box<dyn ObjectBehavior> {
                    let random = container.resolve::<Box<dyn Random>>().unwrap();
                    box StochasticSpreading::new(1.0 / 5_000.0, random)
                }
            };
            let organism_factory = {
                let container = container.clone();
                box move || -> Box<dyn ObjectBehavior> {
                    let neural_network_development_orchestrator = container
                        .resolve::<Box<dyn NeuralNetworkDevelopmentOrchestrator>>()
                        .unwrap();
                    box OrganismBehavior::new(
                        (Genome::default(), Genome::default()),
                        neural_network_development_orchestrator,
                        box AdditionalObjectDescriptionBincodeDeserializer::default(),
                    )
                }
            };
            let terrain_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
            let water_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };

            let simulation_factory = container
                .resolve::<Box<dyn Fn() -> Box<dyn Simulation>>>()
                .unwrap();
            let name_provider = container.resolve::<Box<dyn NameProvider>>().unwrap();

            let additional_object_description_serializer = container
                .resolve::<Box<dyn AdditionalObjectDescriptionSerializer>>()
                .unwrap();

            box HardcodedGenerator::new(
                simulation_factory,
                plant_factory,
                organism_factory,
                terrain_factory,
                water_factory,
                name_provider,
                additional_object_description_serializer,
            ) as Box<dyn WorldGenerator<'_>>
        })
        .register(|_| box FixedIntervalSleeperImpl::default() as Box<dyn FixedIntervalSleeper>)
        .register(|_| box BincodeSerializer::default() as Box<dyn ViewModelSerializer>)
        .register(|container| {
            let container = container.clone();
            Arc::new(move |websocket_client, current_snapshot_fn| {
                let interval = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);
                let fixed_interval_sleeper = container
                    .resolve::<Box<dyn FixedIntervalSleeper>>()
                    .unwrap();
                let presenter = container.resolve::<Box<dyn Presenter>>().unwrap();
                let view_model_serializer =
                    container.resolve::<Box<dyn ViewModelSerializer>>().unwrap();

                let connection = Connection {
                    id: Uuid::new_v4(),
                    socket: box WebsocketClient::new(websocket_client),
                };

                box ClientHandler::new(
                    interval,
                    fixed_interval_sleeper,
                    presenter,
                    view_model_serializer,
                    connection,
                    current_snapshot_fn,
                ) as Box<dyn Client>
            }) as Arc<ClientFactoryFn>
        })
        .register(move |container| {
            let container = container.clone();
            Arc::new(move |current_snapshot_fn| {
                let client_factory_fn = container.resolve::<Arc<ClientFactoryFn>>().unwrap();
                let addr = container.resolve::<SocketAddr>().unwrap();
                let thread_spawn_fn = container.resolve::<Box<ThreadSpawnFn>>().unwrap();

                box WebsocketConnectionAcceptor::try_new(
                    addr,
                    client_factory_fn,
                    thread_spawn_fn,
                    current_snapshot_fn,
                )
                .expect("Failed to create websocket connection acceptor")
                    as Box<dyn ConnectionAcceptor>
            }) as Arc<ConnectionAcceptorFactoryFn>
        })
        .register(|container| {
            let expected_delta = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);

            let mut world_generator = container.resolve::<Box<dyn WorldGenerator<'_>>>().unwrap();
            let connection_acceptor_factory_fn = container
                .resolve::<Arc<ConnectionAcceptorFactoryFn>>()
                .unwrap();
            let thread_spawn_fn = container.resolve::<Box<ThreadSpawnFn>>().unwrap();
            box ControllerImpl::new(
                world_generator.generate(),
                connection_acceptor_factory_fn,
                thread_spawn_fn,
                expected_delta,
            ) as Box<dyn Controller>
        });

    container
}

fn load_names_from_file(path: &Path) -> Vec<String> {
    read_to_string(path)
        .expect("Error while reading file")
        .lines()
        .map(String::from)
        .collect()
}
