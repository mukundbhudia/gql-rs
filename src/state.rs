use crate::Config;

#[derive(Clone)]
pub struct State {
    pub config: Config,
}

impl State {
    pub fn new(config: Config) -> tide::Result<Self> {
        Ok(State { config })
    }
}
