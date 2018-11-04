use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{Client, ThreadSpawnFn, WebsocketConnectionAcceptor};
use crate::constant::*;
use crate::controller::{ConnectionAcceptor, Controller, ControllerImpl};
use crate::fixed_interval_sleeper::FixedIntervalSleeperImpl;
use crate::presenter::DeltaPresenter;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
use myelin_environment::simulation_impl::world::NphysicsWorld;
use myelin_environment::{simulation_impl::SimulationImpl, Simulation};
use myelin_object_behavior::Static;
use myelin_visualization_core::serialization::JsonSerializer;
use myelin_visualization_core::transmission::ViewModelTransmitter;
use myelin_worldgen::generator::HardcodedGenerator;
use myelin_worldgen::WorldGenerator;
use spmc::{channel, Sender};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, thread};
use threadpool::ThreadPool;
use uuid::Uuid;
use websocket::sync::Server;
use websocket::OwnedMessage;

///
/// Starts the simulation and a websocket server, that broadcasts
/// `ViewModel`s on each step to all clients.
///
pub fn start_server<A>(addr: A)
where
    A: Into<SocketAddr> + Send,
{
    let addr = addr.into();

    let simulation_factory = box || -> Box<dyn Simulation> {
        let rotation_translator = NphysicsRotationTranslatorImpl::default();
        let force_applier = SingleTimeForceApplierImpl::default();
        let world = NphysicsWorld::with_timestep(
            SIMULATED_TIMESTEP_IN_SI_UNITS,
            box rotation_translator,
            box force_applier,
        );
        box SimulationImpl::new(box world)
    };
    let object_factory = box |_: Kind| -> Box<dyn ObjectBehavior> { box Static::new() };
    let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);

    let conection_acceptor_factory_fn = Arc::new(move |current_snapshot_fn| {
        let client_factory_fn = Arc::new(|websocket_client, current_snapshot_fn| {
            let interval = Duration::from_float_secs(SIMULATED_TIMESTEP_IN_SI_UNITS);
            let fixed_interval_sleeper = FixedIntervalSleeperImpl::default();
            let presenter = DeltaPresenter::default();
            let view_model_serializer = JsonSerializer::new();

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
