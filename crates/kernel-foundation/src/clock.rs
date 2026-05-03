use crate::time::IsoDateTime;

pub trait Clock: Send + Sync {
    fn now(&self) -> IsoDateTime;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> IsoDateTime {
        IsoDateTime::now()
    }
}