fn main() {
    println!("Let's gooooo");

    calc_loop();
}

fn calc_loop() {
    let mut biggest: (u64, u64) = (0, 0);
    for i in 1..u64::MAX {
        let res = iter_loop(i);
        if res > biggest.1 {
            println!("{}:{}", i, res);
            biggest = (i, res);
        }
    }
}

fn iter_loop(start_number: u64) -> u64 {
    let mut i = start_number as u128;
    let mut iterations = 0u64;

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
