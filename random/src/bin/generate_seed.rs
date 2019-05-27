use myelin_random::{Random, RandomImpl};

fn main() {
    let random = RandomImpl::new();

    const SEED_LENGTH: usize = 32;
    for _ in 0..SEED_LENGTH {
        print!("{:#x}, ", random_byte(&random));
    }

    println!();
}

fn random_byte(random: &dyn Random) -> u8 {
    const MAX_VALUE: usize = std::u8::MAX as usize;
    random.usize_in_range(0, MAX_VALUE) as u8
}
