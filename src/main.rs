use rayon::prelude::*;
use std::convert::TryInto;
use std::time::{Duration, Instant};
fn main() {
    println!("Let's gooooo");
    let now = Instant::now();
    par_calculation();
    println!("Done: {}", now.elapsed().as_millis());
}

fn par_calculation() {
    let mut biggest: (u32, u32) = (0, 0);

    for i in (1..1000000000).step_by(100000) {
        let arr: [u32; 100000] = (i as u32..(i as u32 + 100000u32))
            .collect::<Vec<u32>>()
            .try_into()
            .expect("wrong size iterator");
        let res = process_nums(&arr, biggest.1);
        for i in res.iter() {
            if i.1 > biggest.1 {
                println!("{}:{}", i.0, i.1);
                biggest = (i.0, i.1);
            }
        }
    }
}

fn process_u32(start_number: u32) -> u32 {
    let mut i = start_number;
    let mut iterations = 0u32;

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

fn process_nums(input: &[u32], biggest: u32) -> Vec<(u32, u32)> {
    let comb: Vec<(u32, u32)> = input
        .par_iter()
        .map(|&i| (i, process_u32(i)))
        .filter(|x| x.1 > biggest)
        .collect();
    comb
}
