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
pub enum Error {
    LTMinCap,
    GTMaxCap,
}

// A Machine's implementation

#[derive(Debug, PartialEq, Eq)]
pub struct Machine<T: MachineTag> {
    pub capacity: usize,
    tag: PhantomData<T>,
}
impl<T: MachineTag> Machine<T> {
    fn downgrade(&mut self, count: usize) -> usize {
        let max_count = (self.capacity - T::CAPACITY_MIN).min(count);
        self.capacity -= max_count;
        return max_count;
    }
    pub fn price(&self) -> usize {
        T::COST_BASELINE + T::COST_STEP * (self.capacity - T::CAPACITY_MIN)
    }
    pub fn new(capacity: usize) -> Result<Machine<T>, Error> {
        match capacity {
            c if c > T::CAPACITY_MAX => Err(Error::GTMaxCap),
            c if c < T::CAPACITY_MIN => Err(Error::LTMinCap),
            c => Ok(Machine {
                capacity: c,
                tag: PhantomData,
            }),
        }
    }
    pub fn match_capacity(mut goal: usize) -> Vec<Self> {
        if goal < T::CAPACITY_MIN {
            goal = T::CAPACITY_MIN;
        }
        // n = ceil(goal / cap_max)
        let mut n = goal / T::CAPACITY_MAX;
        if goal % T::CAPACITY_MAX != 0 {
            n += 1;
        }
        let l = n * T::CAPACITY_MAX - goal;
        let mut out = Vec::new();
        for _ in 0..n {
            out.push(Self::new(T::CAPACITY_MAX).unwrap());
        }
        out.iter_mut()
            .scan(l, |well, m| {
                let u = m.downgrade((T::CAPACITY_MAX - T::CAPACITY_MIN).min(*well));
                *well -= u;
                if *well > 0 {
                    Some(())
                } else {
                    None
                }
            })
            .for_each(drop);
        return out;
    }
}

// Constants, which will get inlined
// during monomorphization

#[derive(Debug, PartialEq, Eq)]
pub struct Assembly;
impl MachineTag for Assembly {
    const COST_STEP: usize = 200;
    const COST_BASELINE: usize = 5000;
    const CAPACITY_MAX: usize = 15;
    const CAPACITY_MIN: usize = 10;
}
#[derive(Debug, PartialEq, Eq)]
pub struct Soldering;
impl MachineTag for Soldering {
    const COST_STEP: usize = 50;
    const COST_BASELINE: usize = 2000;
    const CAPACITY_MAX: usize = 30;
    const CAPACITY_MIN: usize = 20;
}
#[derive(Debug, PartialEq, Eq)]
pub struct QualityChecking;
impl MachineTag for QualityChecking {
    const COST_STEP: usize = 2000;
    const COST_BASELINE: usize = 8000;
    const CAPACITY_MAX: usize = 10;
    const CAPACITY_MIN: usize = 8;
}
