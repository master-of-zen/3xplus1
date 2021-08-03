use rayon::prelude::*;
use std::convert::TryInto;
fn main() {
    println!("Let's gooooo");

    par_calculation();
}

fn par_calculation() {
    let mut biggest: (u128, u128) = (0, 0);

    for i in (1..u64::MAX).step_by(1000) {
        let arr: [u128; 1000] = (i as u128..(i as u128 + 1000u128))
            .collect::<Vec<u128>>()
            .try_into()
            .expect("wrong size iterator");
        let res = process_nums(&arr);
        if res.1 > biggest.1 {
            println!("{}:{}", res.0, res.1);
            biggest = res;
        }
    }
}

fn process_u128(start_number: u128) -> u128 {
    let mut i = start_number;
    let mut iterations = 0u128;

    loop {
        iterations += 1;
        if i % 2 == 0 {
            i /= 2
        } else {
            i = 3 * i + 1;
        }

        if i == 1 {
            return iterations;
        }
    }
}

fn process_nums(input: &[u128]) -> (u128, u128) {
    let mut comb: Vec<(u128, u128)> = input
        .clone()
        .par_iter()
        .map(|&i| (i, process_u128(i)))
        .collect();

    comb.sort_by(|x, y| x.1.cmp(&y.1));

    comb[0]
}
