use std::marker::PhantomData;

use self::driver::StreamDriver;

use super::topology::Topology;

pub struct Idle;
pub struct Started;

mod driver;

enum ApplicationState {
    Running,
    Rebalancing,
    Paused,
}

pub struct Application<State, Driver> {
    driver: Driver,
    state: PhantomData<State>,
}

impl<Driver> Application<Idle, Driver> {
    fn new(driver: Driver) -> Application<Idle, Driver> {
        Application {
            driver,
            state: PhantomData,
        }
    }

    fn start(self, topology: Topology) -> Application<Started, Driver> {
        Application {
            driver: self.driver,
            state: PhantomData,
        }
    }
}

impl<Driver> Application<Started, Driver> {
    fn get_state() -> ApplicationState {
        unimplemented!()
    }
}
