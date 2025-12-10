use rust_cli_app::SecretDial;

const DIAL_SIZE: u32 = 100;
const DIAL_STARTING_NUMBER: u32 = 50;

fn main() {
    // Read secret_document and get secret door password
    let rotation_file = include_str!("../secret_document.txt");
    let rotations: Vec<String> = rotation_file.lines().map(|s| s.to_string()).collect();
    println!("Rotations: {:?}", rotations);

    let mut dial = SecretDial::new(DIAL_SIZE, DIAL_STARTING_NUMBER);
    dial.rotate_many(rotations.iter().map(|s| s.to_string()).collect());
    let current = dial.get_current_number();
    println!("Current number: {current}");
    let zeroes = dial.get_zeroes();
    println!("Number of zeroes: {zeroes}");
}
