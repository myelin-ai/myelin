use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{
    Client, ClientFactoryFn, ThreadSpawnFn, WebsocketConnectionAcceptor,
};
use crate::constant::*;
use crate::controller::{
    ConnectionAcceptor, ConnectionAcceptorFactoryFn, Controller, ControllerImpl, Presenter,
};
use crate::fixed_interval_sleeper::{FixedIntervalSleeper, FixedIntervalSleeperImpl};
use crate::presenter::DeltaPresenter;
use myelin_engine::{prelude::*, simulation::SimulationBuilder};
use myelin_genetics::{
    genome::Genome,
    genome_generator_impl::{
        CorpusCallosumClusterGeneGenerator, CorpusCallosumClusterGeneGeneratorImpl,
        GenomeGeneratorImpl, IoClusterGeneGenerator, IoClusterGeneGeneratorImpl,
    },
    neural_network_development_orchestrator_impl::{
        ChromosomalCrossoverGenomeDeriver, GeneticNeuralNetworkDeveloper, GenomeDeriver,
        GenomeMutator, GenomeMutatorStub, InputNeuronHandles, NeuralNetworkConfigurator,
        NeuralNetworkConfiguratorFactory, NeuralNetworkConfiguratorImpl, NeuralNetworkDeveloper,
        NeuralNetworkDeveloperFactory, NeuralNetworkDevelopmentOrchestratorImpl,
        OutputNeuronHandles,
    },
    GenomeGenerator, NeuralNetworkDevelopmentConfiguration, NeuralNetworkDevelopmentOrchestrator,
};
use myelin_neural_network::{spiking_neural_network::DefaultSpikingNeuralNetwork, NeuralNetwork};
use myelin_object_behavior::{
    organism::OrganismBehavior, stochastic_spreading::StochasticSpreading, Static,
};
use myelin_object_data::{AdditionalObjectDescription, Kind};
use myelin_random::{Random, RandomImpl};
use myelin_visualization_core::serialization::{BincodeSerializer, ViewModelSerializer};
use myelin_worldgen::{
    HardcodedGenerator, NameProvider, NameProviderBuilder, NameProviderFactory,
    ShuffledNameProviderFactory, WorldGenerator,
};
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use wonderbox::Container;

/// Starts the simulation and a websocket server, that broadcasts
/// `ViewModel`s on each step to all clients.
pub fn start_server<A>(addr: A)
where
    A: Into<SocketAddr> + Send,
{
    let container = create_composition_root(addr.into());
    let mut controller = container.resolve::<Box<dyn Controller>>();
    controller.run();
}

struct ServerAddress(SocketAddr);

fn create_composition_root(addr: SocketAddr) -> Container {
    let mut container = Container::new();

    container
        .register(move |_| ServerAddress(addr))
        .extend(utility_container())
        .extend(server_container())
        .extend(client_container())
        .extend(neural_network_container())
        .extend(genetics_container())
        .extend(worldgen_container())
        .extend(object_behavior_container());

    container
}

fn utility_container() -> Container {
    let mut container = Container::new();

    container
        .register(|_| box RandomImpl::new() as Box<dyn Random>)
        .register(|_| box FixedIntervalSleeperImpl::new() as Box<dyn FixedIntervalSleeper>)
        .register(|_| {
            box (|function| {
                thread::spawn(function);
            }) as Box<ThreadSpawnFn>
        });

    container
}

fn server_container() -> Container {
    let mut container = Container::new();

    container
        .register(|_| box BincodeSerializer::new() as Box<dyn ViewModelSerializer>)
        .register(|container| {
            let expected_delta = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);

            let mut world_generator = container.resolve::<Box<dyn WorldGenerator<'_>>>();
            let connection_acceptor_factory_fn =
                container.resolve::<Arc<ConnectionAcceptorFactoryFn>>();
            let thread_spawn_fn = container.resolve::<Box<ThreadSpawnFn>>();
            box ControllerImpl::new(
                world_generator.generate(),
                connection_acceptor_factory_fn,
                thread_spawn_fn,
                expected_delta,
            ) as Box<dyn Controller>
        });

    container
}

fn client_container() -> Container {
    let mut container = Container::new();

    container
        .register(|_| box DeltaPresenter::new() as Box<dyn Presenter>)
        .register(|container| {
            let container = container.clone();
            Arc::new(move |websocket_client, current_snapshot_fn| {
                let interval = Duration::from_secs_f64(SIMULATED_TIMESTEP_IN_SI_UNITS);
                let fixed_interval_sleeper = container.resolve::<Box<dyn FixedIntervalSleeper>>();
                let presenter = container.resolve::<Box<dyn Presenter>>();
                let view_model_serializer = container.resolve::<Box<dyn ViewModelSerializer>>();

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
                let client_factory_fn = container.resolve::<Arc<ClientFactoryFn>>();
                let addr = container.resolve::<ServerAddress>().0;
                let thread_spawn_fn = container.resolve::<Box<ThreadSpawnFn>>();

                box WebsocketConnectionAcceptor::try_new(
                    addr,
                    client_factory_fn,
                    thread_spawn_fn,
                    current_snapshot_fn,
                )
                .expect("Failed to create websocket connection acceptor")
                    as Box<dyn ConnectionAcceptor>
            }) as Arc<ConnectionAcceptorFactoryFn>
        });

    container
}

fn genetics_container() -> Container {
    let mut container = Container::new();

    container.register(|container| {
        let random = container.resolve::<Box<dyn Random>>();
        let io_cluster_gene_generator = IoClusterGeneGeneratorImpl::new(random);
        box io_cluster_gene_generator as Box<dyn IoClusterGeneGenerator>
    });

    container.register(|container| {
        let random = container.resolve::<Box<dyn Random>>();
        let corpus_callosum_cluster_gene_generator =
            CorpusCallosumClusterGeneGeneratorImpl::new(random);
        box corpus_callosum_cluster_gene_generator as Box<dyn CorpusCallosumClusterGeneGenerator>
    });

    container.register(|container| {
        let random = container.resolve::<Box<dyn Random>>();
        let io_cluster_gene_generator = container.resolve::<Box<dyn IoClusterGeneGenerator>>();
        let corpus_callosum_cluster_gene_generator =
            container.resolve::<Box<dyn CorpusCallosumClusterGeneGenerator>>();
        let genome_generator = GenomeGeneratorImpl::new(
            io_cluster_gene_generator,
            corpus_callosum_cluster_gene_generator,
            random,
        );
        box genome_generator as Box<dyn GenomeGenerator>
    });

    container
}

fn neural_network_container() -> Container {
    let mut container = Container::new();
    container
        .register(|_| {
            Rc::new(|| box DefaultSpikingNeuralNetwork::new() as Box<dyn NeuralNetwork>)
                as Rc<dyn Fn() -> Box<dyn NeuralNetwork>>
        })
        .register(|_| {
            fn neural_network_developer_factory_factory() -> impl for<'b> Fn(
                &'b NeuralNetworkDevelopmentConfiguration,
                &'b Genome,
            ) -> Box<
                dyn NeuralNetworkDeveloper + 'b,
            > {
                move |configuration, genome| {
                    box GeneticNeuralNetworkDeveloper::new(configuration.clone(), genome.clone())
                        as Box<dyn NeuralNetworkDeveloper>
                }
            }
            Rc::new(neural_network_developer_factory_factory()) as Rc<NeuralNetworkDeveloperFactory>
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
        });

    container
}

fn worldgen_container() -> Container {
    let mut container = Container::new();

    container
        .register(|_| myelin_worldgen::SimulationFactory(box || SimulationBuilder::new().build()))
        .register(|_| {
            myelin_worldgen::TerrainFactory(
                box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                    box Static::default()
                },
            )
        })
        .register(|_| {
            myelin_worldgen::WaterFactory(
                box || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
                    box Static::default()
                },
            )
        })
        .register(|container| {
            let plant_factory = container.resolve::<myelin_worldgen::PlantFactory>();

            let organism_factory = container.resolve::<myelin_worldgen::OrganismFactory>();

            let terrain_factory = container.resolve::<myelin_worldgen::TerrainFactory>();

            let water_factory = container.resolve::<myelin_worldgen::WaterFactory>();

            let simulation_factory = container.resolve::<myelin_worldgen::SimulationFactory<'_>>();

            let name_provider = container.resolve::<Box<dyn NameProvider>>();

            box HardcodedGenerator::new(
                simulation_factory,
                plant_factory,
                organism_factory,
                terrain_factory,
                water_factory,
                name_provider,
            ) as Box<dyn WorldGenerator<'_>>
        });
    container
}

fn object_behavior_container() -> Container {
    let mut container = Container::new();

    container
        .register(create_name_provider)
        .register(create_plant_factory)
        .register(create_organism_factory)
        .register(|_| box ShuffledNameProviderFactory::new() as Box<dyn NameProviderFactory>)
        .register(|c| {
            box ChromosomalCrossoverGenomeDeriver::new(c.resolve()) as Box<dyn GenomeDeriver>
        })
        .register(|_| box GenomeMutatorStub::new() as Box<dyn GenomeMutator>)
        .register(|c| {
            box NeuralNetworkDevelopmentOrchestratorImpl::new(
                c.resolve(),
                c.resolve(),
                c.resolve(),
                c.resolve(),
                c.resolve(),
            ) as Box<dyn NeuralNetworkDevelopmentOrchestrator>
        });

    container
}

fn create_name_provider(container: &Container) -> Box<dyn NameProvider> {
    let name_provider_factory = container.resolve::<Box<dyn NameProviderFactory>>();
    let mut name_provider_builder = NameProviderBuilder::new(name_provider_factory);
    let organism_names = load_names_from_file(Path::new("./object-names/organisms.txt"));
    name_provider_builder.add_names(&organism_names, Kind::Organism);
    name_provider_builder.build()
}

fn create_plant_factory(container: &Container) -> myelin_worldgen::PlantFactory {
    let container = container.clone();
    myelin_worldgen::PlantFactory(
        box move || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
            let random = container.resolve::<Box<dyn Random>>();
            box StochasticSpreading::new(1.0 / 5_000.0, random)
        },
    )
}

fn create_organism_factory(container: &Container) -> myelin_worldgen::OrganismFactory {
    let container = container.clone();
    myelin_worldgen::OrganismFactory(
        box move || -> Box<dyn ObjectBehavior<AdditionalObjectDescription>> {
            let neural_network_development_orchestrator =
                container.resolve::<Box<dyn NeuralNetworkDevelopmentOrchestrator>>();
            let genome_generator = container.resolve::<Box<dyn GenomeGenerator>>();
            box OrganismBehavior::from_genome_generator(
                genome_generator,
                neural_network_development_orchestrator,
            )
        },
    )
}

fn load_names_from_file(path: &Path) -> Vec<String> {
    read_to_string(path)
        .expect("Error while reading file")
        .lines()
        .map(String::from)
        .collect()
}
