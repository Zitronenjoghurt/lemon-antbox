use crate::simulation::ant::{Ant, AntSenses};

#[derive(Clone)]
pub struct AntBuffer {
    pub ant: Ant,
    pub senses: AntSenses,
}
