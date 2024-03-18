fn summation(k:u32) -> u128 {
    let mut result: u128 = 0;
    // Establishes a result mutable for the function to return
    for i in 1..=k {
        result += (i as u128) * (i as u128);
        // Converts i to u64 and adds the square of i to result
    }
    // Loops k times 
    result
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    let input = input.trim();
    let k: u32 = input.parse().expect("Not a good number!");
    let sum = summation(k);
    // Calls the summation function with K = 5
    println!("The sum of squares up to {} is {}", k, sum);
    // Prints the value of k and returns summation
    }