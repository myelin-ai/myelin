use crate::client::ClientHandler;
use crate::connection::{Connection, WebsocketClient};
use crate::connection_acceptor::{Client, WebsocketConnectionAcceptor};
use crate::constant::*;
use crate::controller::{Controller, ControllerImpl};
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

struct ChannelTransmitter(Sender<Vec<u8>>);

impl ViewModelTransmitter for ChannelTransmitter {
    fn send_view_model(&self, view_model: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.0.send(view_model)?;

        Ok(())
    }
}

impl fmt::Debug for ChannelTransmitter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChannelTransmitter").finish()
    }
}

fn run_simulation(tx: Sender<Vec<u8>>) {
    thread::spawn(move || {
        let transmitter = Box::new(ChannelTransmitter(tx));
        let serializer = Box::new(JsonSerializer::new());
        let presenter = Box::new(DeltaPresenter::new());
        let simulation_factory = Box::new(|| -> Box<dyn Simulation> {
            let rotation_translator = NphysicsRotationTranslatorImpl::default();
            let force_applier = SingleTimeForceApplierImpl::default();
            let world = Box::new(NphysicsWorld::with_timestep(
                SIMULATED_TIMESTEP_IN_SI_UNITS,
                Box::new(rotation_translator),
                Box::new(force_applier),
            ));
            Box::new(SimulationImpl::new(world))
        });
        let object_factory =
            Box::new(|_: Kind| -> Box<dyn ObjectBehavior> { Box::new(Static::new()) });
        let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);

        /*
        let mut controller = ControllerImpl::new(
            presenter,
            &worldgen,
            Duration::from_float_secs(SIMULATED_TIMESTEP),
        );
        
        controller.run();
        */
        unimplemented!()
    });
}

///
/// Starts the simulation and a websocket server, that broadcasts
/// `ViewModel`s on each step to all clients.
///
pub fn start_server<A>(addr: A)
where
    A: Into<SocketAddr>,
{
    let rotation_translator = NphysicsRotationTranslatorImpl::default();
    let force_applier = SingleTimeForceApplierImpl::default();
    let world = NphysicsWorld::with_timestep(
        SIMULATED_TIMESTEP_IN_SI_UNITS,
        box rotation_translator,
        box force_applier,
    );
    let simulation = SimulationImpl::new(box world);

    let interval = Duration::from_millis(SIMULATED_TIMESTEP_IN_MILLIS as u64);
    let fixed_interval_sleeper = FixedIntervalSleeperImpl::default();
    let presenter = DeltaPresenter::default();
    let view_model_serializer = JsonSerializer::new();
    let client_factory_fn = Arc::new(move |websocket_client, current_snapshot_fn| {
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
    let conection_acceptor_factory_fn = Arc::new(move |current_snapshot_fn| {
        WebsocketConnectionAcceptor::try_new(
            addr.into(),
            client_factory_fn,
            thread_spawn,
            current_snapshot_fn,
        )
    });
    let controller = ControllerImpl::new(box simulation, conection_acceptor_factory_fn);

    const MAX_CONNECTIONS: usize = 255;
    let thread_pool = ThreadPool::new(MAX_CONNECTIONS);

    run_simulation(tx);

    for request in server.filter_map(Result::ok) {
        let rx = rx.clone();

        thread_pool.execute(move || {
            let mut client = request.accept().unwrap();

            loop {
                let view_model = rx.recv().expect("sending end of channel is gone");

                client
                    .send_message(&OwnedMessage::Binary(view_model))
                    .expect("unable to send message");
            }
        });
    }
}
