fn main() {
    println!("Let's gooooo");

    let mut i = 1usize;
    let mut start = 1usize;
    let mut iterations = 0usize;
    loop {
        iterations += 1;
        if i % 2 == 0 {
            i /= 2
        } else {
            i = 3 * i + 1;
        }

        if i == 1 {
            println!("{} reached in {} iterations", start, iterations);
            iterations = 0;
            start += 1;
            i = start;
        }
    }
}
