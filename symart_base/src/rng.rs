use rand::distributions::Distribution;
use rand::rngs::{OsRng, SmallRng};
use rand::SeedableRng;
use std::cell::RefCell;

fn make_rng() -> SmallRng {
    SmallRng::from_rng(OsRng).unwrap()
}

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(make_rng());
}

pub fn sample_fn<F, T>(f: F) -> T
where
    F: FnOnce(&mut SmallRng) -> T,
{
    RNG.with(|r| f(&mut r.borrow_mut()))
}

pub fn sample<D, T>(dist: D) -> T
where
    D: Distribution<T>,
{
    sample_fn(|r| dist.sample(r))
}
