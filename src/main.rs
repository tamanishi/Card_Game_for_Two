use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let mut line2 = String::new();
    let _ = io::stdin().read_line(&mut line2);
    let mut values = line2
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut alice: Vec<u32> = vec![];
    let mut bob: Vec<u32> = vec![];

    values.sort();
    values.reverse();

    for i in 0..values.len() {
        match i % 2 {
            0 => alice.push(values[i]),
            1 => bob.push(values[i]),
            _ => (),
        }
    }

    println!("{}", alice.iter().sum::<u32>() - bob.iter().sum::<u32>());
}
