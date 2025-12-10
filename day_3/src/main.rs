use day_3::libs::find_battery_power::find_battery_power;

fn main() {
    let labels = include_str!("../bank_labels.txt");

    let labels = labels.lines();

    let mut total_power: usize = 0;

    for label in labels {
        let power = find_battery_power(label.to_string(), 12);
        total_power += power;
    }

    println!("Battery power: {}", total_power);
}
