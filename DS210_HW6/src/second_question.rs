fn main() {
    let mut F: [u128; 101] = [0; 101];
    F[0] = 0;
    F[1] = 1;
    // initialize 0 and 1 which would be out of index otherwise 
    for i in 2..=100 {
        // for loop which creates i32 values in range of 0-100 inclusive
        F[i] = F[i - 1] + F[i - 2];
        // This is the arthimetic operations for the fibonnaci sequence
        println!("{:?}", F[i]);
        // Prints each value of F with the value of F[i] replaced with Fi
        }
    }
