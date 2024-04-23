use std::fs::File; // For Reading the File
use std::io::prelude::*; // For Vectors
use rand::prelude::*; // For Random Distributions
use std::io::{self, BufRead}; // For Writing on the File



fn write(filepath: &str) {
    let mut file = File::create(filepath).expect("File Error");
    let mut rng = thread_rng(); // Initialize Random for Use of Coordinates

    for _ in 0..500 {
        let coordinates: [i32; 2] = [rng.gen_range(0..=100_000_000), rng.gen_range(0..=1)]; // [Random Value Inclusive of 100000, then a Binary Value]
        writeln!(file, "{}, {}", coordinates[0], coordinates[1]).expect("File Error"); // Write These Coordinates
    }
}
fn read_data(filepath: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = File::open(filepath)?; // The File
    let reader = io::BufReader::new(file); // The File Reader
    let mut data = Vec::new(); // A New Vector Titled Data

    for line_result in reader.lines() { // Iterate Through Lines in Txt File @ Path
        let line = line_result?; // The Line Result is then Added to a Value
        let parts: Vec<_> = line.split(", ").collect(); // Split the Txt File Where it Has a Comma ,
        if parts.len() == 2 { // Default Length of Each Line is 2 (x and z)
            let x = parts[0].parse::<i32>().unwrap_or_default();
            let z = parts[1].parse::<i32>().unwrap_or_default();
            data.push((x, z)); // Push the Coordinates it to the Data Vector
        }
    }
    Ok(data) // Return the Vector Form of the Text File
}
fn new_threshold(data: &[(i32, i32)]) -> (i32, f32) {
    let mut best_accuracy = 0.0;
    let mut best_threshold = 0;
    
    
    let min_x = data.iter().map(|(x, _)| *x).min().unwrap_or(0); // Minimum Point in Data
    let max_x = data.iter().map(|(x, _)| *x).max().unwrap_or(0); // Maximum Point in Data

    // Evaluate each possible threshold from min_x to max_x
    for threshold in min_x..=max_x { // Iterate Through a Range Minimum of Data and Maximum in Data (Only the X Value is Considered)
        let mut hits = 0; // How Many the Threshold Correctly Guesses

        for &(x, z) in data {
            // Apply the Rule: If x >= Threshold, Predict 1; Else Predict 0
            let pred = if x >= threshold { 1 } else { 0 };
            if pred == z {
                hits += 1; // Increment Hits for Correct Predictions
            }
        }

        let accuracy = hits as f32 / data.len() as f32; // Calculate Accuracy for This Threshold as Float
        if accuracy > best_accuracy {
            best_accuracy = accuracy;
            best_threshold = threshold; // Update best_threshold if this Accuracy is the Best so Far
        }
    }
    (best_threshold, best_accuracy)
}


fn main() -> io::Result<()> {
    write("data.txt"); // My Data
    let filepath = "data.txt"; // The Final Path
    let test_path = "test.txt"; // Test Path
    let data = read_data(filepath)?; // Main Path --> Data
    let data_test = read_data(test_path)?; // Test Path --> Data_test

    let (threshold, accuracy) = new_threshold(&data); // Main Data Threshold
    let (test_threshold, test_accuracy) = new_threshold(&data_test); // Test Data Threshold (Test Function)
    
    println!("Decision Tree: Data.txt");
    println!("if x >= {}", threshold);
    println!("    Predict label: 1");
    println!("else");
    println!("    Predict label: 0");
    println!("Accuracy: {:.2}%", accuracy * 100.0);
    println!("=============================================");

    println!("Decision Tree: Test.txt");
    println!("Best threshold: {}, Accuracy: {:.2}%", threshold, accuracy * 100.0);
    println!("Best threshold: {}, Accuracy: {}", test_threshold, test_accuracy); //
    println!("Decision Tree:");
    println!("if x >= {}", test_threshold);
    println!("    Predict label: 1");
    println!("else");
    println!("    Predict label: 0");
    println!("Accuracy: {:.2}%", test_accuracy * 100.0);

    Ok(())
}

