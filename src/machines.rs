use std::marker::PhantomData;

// This module models the 3 available machines
// They are represented as one struct that
// implements the correct behaviour, which is
// parametrized through the MachineTag trait and
// its implementors' provided constants.
// The Machine struct can instantiate itself,
// this checks for the capacity being in the
// correct range, but requires generic parameters
// or variable type to be supplied. It also
// computes its prices.

pub trait MachineTag {
    const COST_STEP: usize;
    const COST_BASELINE: usize;
    const CAPACITY_MAX: usize;
    const CAPACITY_MIN: usize;
}
#[derive(Debug)]
pub enum MachineError {
    LTMinCap,
    GTMaxCap,
}

// A Machine's implementation

#[derive(Debug)]
pub struct Machine<T: MachineTag> {
    capacity: usize,
    tag: PhantomData<T>,
}
impl<T: MachineTag> Machine<T> {
    pub fn price(&self) -> usize {
        T::COST_BASELINE + T::COST_STEP * (self.capacity - T::CAPACITY_MIN)
    }
    pub fn new(capacity: usize) -> Result<Machine<T>, MachineError> {
        match capacity {
            c if c > T::CAPACITY_MAX => Err(MachineError::GTMaxCap),
            c if c < T::CAPACITY_MIN => Err(MachineError::LTMinCap),
            c => Ok(Machine {
                capacity: c,
                tag: PhantomData,
            }),
        }
    }
}

// Constants, which will get inlined
// during monomorphization

#[derive(Debug)]
pub struct Assembly;
impl MachineTag for Assembly {
    const COST_STEP: usize = 200;
    const COST_BASELINE: usize = 5000;
    const CAPACITY_MAX: usize = 15;
    const CAPACITY_MIN: usize = 10;
}
#[derive(Debug)]
pub struct Soldering;
impl MachineTag for Soldering {
    const COST_STEP: usize = 50;
    const COST_BASELINE: usize = 2000;
    const CAPACITY_MAX: usize = 30;
    const CAPACITY_MIN: usize = 20;
}
#[derive(Debug)]
pub struct QualityChecking;
impl MachineTag for QualityChecking {
    const COST_STEP: usize = 2000;
    const COST_BASELINE: usize = 8000;
    const CAPACITY_MAX: usize = 10;
    const CAPACITY_MIN: usize = 8;
}
