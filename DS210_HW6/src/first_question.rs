fn test() {
    use std::time::Instant;
    fn fib(x: u32) -> u128 {
        if x == 0 {
            0
        }
        else if x == 1 {
            1
        }
        // base case for the function 
        else {
            fib(x - 2) + fib(x - 1)
        // recursive case for function
        }
        }
    for number in 0..=50 {
        println!("K equals {}", number);
        // variable below tracks current time
        let start = Instant::now();
        let result = fib(number);
        let duration: std::time::Duration = start.elapsed();
        // variable above tracks elapsed time of recursion
        println!("Fibonacci of {} is: {}", number, result);
        println!("Calculation took: {:?}", duration);
        // print commands for Fib number and Calculation time

    }
    }

