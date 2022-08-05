// My first nasty rust program!! :DDD
// It looks like C!! NEED FIX!!
use std::io;

fn main() {
    let mut line_buffer = String::new();
    let mut inputs: Vec<i32> = Vec::new();
    let mut counts = 0;

    while !line_buffer.ends_with("\n\n") {
        io::stdin()
            .read_line(&mut line_buffer)
            .expect("failed to read line.");
    }

    println!("------");
    let iter = line_buffer.split_whitespace();
    iter.for_each(|x| inputs.push(x.trim().parse().expect("failed to parse.")));

    for i in 1..inputs.len() {
        if inputs[i] > inputs[i - 1] {
            counts += 1;
        }
    }
    println!("{counts}");
}

// A great discussion on reading input as integer
// https://users.rust-lang.org/t/how-to-read-an-integer-from-stdin/57538
