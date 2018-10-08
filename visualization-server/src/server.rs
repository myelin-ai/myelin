use crate::constant::SIMULATED_TIMESTEP;
use crate::controller::{Controller, ControllerImpl};
use crate::presenter::DeltaPresenter;
use crate::serialize::JsonSerializer;
use crate::transmitter::ViewModelTransmitter;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
use myelin_environment::simulation_impl::world::NphysicsWorld;
use myelin_environment::{simulation_impl::SimulationImpl, Simulation};
use myelin_object_behavior::Static;
use myelin_worldgen::generator::HardcodedGenerator;
use spmc::{channel, Sender};
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
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
        let presenter = Box::new(DeltaPresenter::new(serializer, transmitter));
        let simulation_factory = Box::new(|| -> Box<dyn Simulation> {
            let rotation_translator = NphysicsRotationTranslatorImpl::default();
            let world = Box::new(NphysicsWorld::with_timestep(
                SIMULATED_TIMESTEP,
                Box::new(rotation_translator),
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
    let addr: SocketAddr = addr.into();
    let (tx, rx) = channel();
    let server = Server::bind(addr).expect("unable to create server");

    info!("Server is listening on {}", addr);

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
