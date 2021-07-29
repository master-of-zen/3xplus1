use rayon::prelude::*;
fn main() {
    println!("Let's gooooo");

    let p: Vec<(u64, u64)> = (1..600000000u64)
        .into_par_iter()
        .map(|x| calc_loop(x))
        .collect();

    println!("{:#?}", &p.len());
}

fn calc_loop(start_number: u64) -> (u64, u64) {
    let mut i = start_number;
    let mut iterations = 0u64;

    loop {
        iterations += 1;
        if i % 2 == 0 {
            i /= 2
        } else {
            i = 3 * i + 1;
        }

        if i == 1 {
            return (start_number, iterations);
        }
    }
}
