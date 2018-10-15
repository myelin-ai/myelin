use crate::connection::Connection;
use crate::controller::{Client, CurrentSnapshotFn, Presenter};
use myelin_visualization_core::serialization::ViewModelSerializer;
use std::fmt::{self, Debug};

struct ClientImpl {
    presenter: Box<dyn Presenter>,
    serializer: Box<dyn ViewModelSerializer>,
    connection: Connection,
    current_snapshot_fn: Box<CurrentSnapshotFn>,
}

impl ClientImpl {
    pub(crate) fn new(
        presenter: Box<dyn Presenter>,
        serializer: Box<dyn ViewModelSerializer>,
        connection: Connection,
        current_snapshot_fn: Box<CurrentSnapshotFn>,
    ) -> Self {
        Self {
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        }
    }
}

impl Client for ClientImpl {
    fn run(&mut self) {
        unimplemented!()
    }
}

impl Debug for ClientImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientImpl")
            .field("presenter", &self.presenter)
            .field("serializer", &self.serializer)
            .field("connection", &self.connection)
            .finish()
    }
}
