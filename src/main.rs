use rayon::prelude::*;
use std::time::Instant;
fn main() {
    println!("Let's gooooo");
    let now = Instant::now();
    functional();
    println!("Done: {}", now.elapsed().as_millis());
}

fn functional() {
    let mut biggest: (u32, u32) = (0, 0);
    for i in (1..1000000000u32).step_by(1000000) {
        let vc: Vec<(u32, u32)> = (i as u32..(i as u32 + 1000000u32))
            .into_par_iter()
            .map(|x| (x, process_u32(x)))
            .filter(|x| x.1 > biggest.1)
            .collect::<Vec<(u32, u32)>>();

        for i in vc.iter() {
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
        if i & 1 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }

        if i == 1 {
            return iterations;
        }
    }
}
