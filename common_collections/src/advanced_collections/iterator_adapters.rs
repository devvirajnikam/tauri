// Scenario:
// A collection needs filtering, transforming, numbering, combining, or reducing.
//
// Thinking:
// Iterator adapters let you describe collection processing as a pipeline without
// manually managing indexes.

pub fn run() {
    println!("\n23. Iterator adapters");

    let numbers = vec![1, 2, 3, 4, 5];

    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|number| **number % 2 == 0)
        .map(|number| number * number)
        .collect();

    let total = numbers.iter().fold(0, |sum, number| sum + number);
    let labelled: Vec<String> = numbers
        .iter()
        .enumerate()
        .map(|(index, number)| format!("{}:{}", index, number))
        .collect();

    println!("Even squares: {:?}", even_squares);
    println!("Total using fold: {}", total);
    println!("Labelled numbers: {:?}", labelled);
}
