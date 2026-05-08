use std::sync::Mutex;

static SEED: Mutex<u32> = Mutex::new(1804289383);

//XOR Shift Pseudo-Random Number Generator
pub fn get_random_num() -> u32 {
    let mut number = SEED.lock().unwrap();

    *number ^= *number << 13;
    *number ^= *number >> 17;
    *number ^= *number << 5;

    *number
}

pub fn get_random_u64_num() -> u64 {
    let nums: [u64; 4] = std::array::from_fn(|_e| {
        (get_random_num() as u64) & 0xFFFF
    });

    nums[0] | (nums[1] << 16) | (nums[2] << 32) | (nums[3] << 48)
}

pub fn generate_magic_number() -> u64 {
    get_random_u64_num() & get_random_u64_num() & get_random_u64_num()
}
    
#[cfg(test)]
mod tests {
    use crate::board::print_board;
    use super::*;

    #[test]
    fn magic_test() {
        print_board(&(get_random_num() as u64));
        print_board(&(get_random_num() as u64));

        print_board(&get_random_u64_num());
        print_board(&generate_magic_number());
    }
}