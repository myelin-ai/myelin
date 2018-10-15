use myelin_environment::object::ObjectDescription;
use myelin_environment::Id;
use std::collections::HashMap;

pub(crate) type Snapshot = HashMap<Id, ObjectDescription>;
