use rug::Integer;
use rand::prelude::*;
use primal::*;

fn generate_prime(n: u32) -> Integer {
    let mut rng = thread_rng();
    loop {
        let mut candidate = Integer::from(rng.gen_range(0..n));
        candidate.set_bit(0, true);
        if is_prime(candidate.to_u64().unwrap()) {
            return candidate;
        }
    }
}

fn find_longest_prime_sum(primes: &[usize], sieve: &Sieve) -> (usize, usize, Vec<usize>) {
    let mut longest_length = 0;
    let mut longest_sum = 0;
    let mut longest_sequence = Vec::new();

    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            sum += primes[j];
            if sum > 1000 {
                break;
            }

            if sieve.is_prime(sum) && (j - i + 1) > longest_length {
                longest_length = j - i + 1;
                longest_sum = sum;
                longest_sequence = primes[i..=j].to_vec();
            }
        }
    }

    (longest_length, longest_sum, longest_sequence)
}

fn main() {
    let upper_limit = 100;
    let prime_number = generate_prime(upper_limit);
    println!("Question 2.\nGenerated prime number: {}\n", prime_number);

    let sieve = Sieve::new(1000);
    let primes = sieve.primes_from(0).take_while(|&p| p < 1000).collect::<Vec<_>>();
    let (length, sum, sequence) = find_longest_prime_sum(&primes, &sieve);
    println!("Question 5.");
    println!("X: {}", length);
    println!("Y: {}", sum);
    println!("Sequence: {:?}", sequence);
}
