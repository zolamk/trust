extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn secure_token(size: usize) -> String {
    return thread_rng().sample_iter(&Alphanumeric).take(size).collect();
}
