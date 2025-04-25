use std::hash::{DefaultHasher, Hash, Hasher};

use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::seq::IndexedRandom;
use rand::{SeedableRng, rng};

const ADJECTIVES: &str = include_str!("../../data/words/adjectives.txt");
const ANIMALS: &str = include_str!("../../data/words/animals.txt");
const DEFAULT_SEED: u64 = 123456789;

lazy_static! {
    static ref ADJECTIVE_LIST: Vec<&'static str> =
        ADJECTIVES.split('\n').map(|n| n.trim()).collect();
    static ref ANIMALS_LIST: Vec<&'static str> = ANIMALS.split('\n').map(|n| n.trim()).collect();
    static ref ALL_WORDS: Vec<&'static str> = {
        let mut words = vec![];
        words.extend(ADJECTIVE_LIST.iter());
        words.extend(ANIMALS_LIST.iter());
        words
    };
}

pub fn generate_id() -> String {
    let mut rng = rng();
    let adjective1 = ADJECTIVE_LIST.choose(&mut rng).unwrap();
    let adjective2 = ADJECTIVE_LIST.choose(&mut rng).unwrap();
    let animal = ANIMALS_LIST.choose(&mut rng).unwrap();

    format!("{adjective1}-{adjective2}-{animal}")
}

pub fn get_random_word() -> String {
    let mut rng = rng();
    ALL_WORDS.choose(&mut rng).unwrap().to_string()
}

pub fn create_seeded_rng(seed: Option<&str>) -> StdRng {
    let seed = match seed {
        Some(string) => {
            let mut hasher = DefaultHasher::new();
            string.hash(&mut hasher);
            hasher.finish()
        }
        None => DEFAULT_SEED,
    };
    StdRng::seed_from_u64(seed)
}
