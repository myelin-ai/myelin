use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{Client, ThreadSpawnFn, WebsocketConnectionAcceptor};
use crate::constant::*;
use crate::controller::{ConnectionAcceptor, Controller, ControllerImpl};
use crate::fixed_interval_sleeper::FixedIntervalSleeperImpl;
use crate::presenter::DeltaPresenter;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation_impl::world::{
    IgnoringCollisionFilterImpl, NphysicsRotationTranslatorImpl, NphysicsWorld,
    SingleTimeForceApplierImpl,
};
use myelin_environment::simulation_impl::{ObjectEnvironmentImpl, SimulationImpl};
use myelin_environment::Simulation;
use myelin_object_behavior::stochastic_spreading::{RandomChanceCheckerImpl, StochasticSpreading};
use myelin_object_behavior::Static;
use myelin_visualization_core::serialization::BincodeSerializer;
use myelin_worldgen::FileSystemNameProviderBuilder;
use myelin_worldgen::{HardcodedGenerator, WorldGenerator};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, RwLock};
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

    let simulation_factory = box || -> Box<dyn Simulation> {
        let rotation_translator = NphysicsRotationTranslatorImpl::default();
        let force_applier = SingleTimeForceApplierImpl::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));
        let world = NphysicsWorld::with_timestep(
            SIMULATED_TIMESTEP_IN_SI_UNITS,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        box SimulationImpl::new(box world, box |simulation| {
            box ObjectEnvironmentImpl::new(simulation)
        })
    };
    let plant_factory = box || -> Box<dyn ObjectBehavior> {
        box StochasticSpreading::new(1.0 / 5_000.0, box RandomChanceCheckerImpl::new())
    };
    let organism_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
    let terrain_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };
    let water_factory = box || -> Box<dyn ObjectBehavior> { box Static::default() };

    let mut name_provider_builder = FileSystemNameProviderBuilder::default();
    name_provider_builder
        .add_file_for_kind(Path::new("./object_names/organisms.txt"), Kind::Organism)
        .expect("Error while loading the file");

    let name_provider = name_provider_builder.build_randomized();

    let mut worldgen = HardcodedGenerator::new(
        simulation_factory,
        plant_factory,
        organism_factory,
        terrain_factory,
        water_factory,
        name_provider,
    );

    let conection_acceptor_factory_fn = Arc::new(move |current_snapshot_fn| {
        let client_factory_fn = Arc::new(|websocket_client, current_snapshot_fn| {
            let interval = Duration::from_float_secs(SIMULATED_TIMESTEP_IN_SI_UNITS);
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
            // To do: How do we get addr?
            addr,
            client_factory_fn,
            spawn_thread_factory(),
            current_snapshot_fn,
        )
        .expect("Failed to create websocket connection acceptor")
            as Box<dyn ConnectionAcceptor>
    });

    let expected_delta = Duration::from_float_secs(SIMULATED_TIMESTEP_IN_SI_UNITS);

    let mut controller = ControllerImpl::new(
        worldgen.generate(),
        conection_acceptor_factory_fn,
        spawn_thread_factory(),
        expected_delta,
    );

    controller.run();
}

fn spawn_thread_factory() -> Box<ThreadSpawnFn> {
    box move |function| {
        thread::spawn(function);
    }
}
