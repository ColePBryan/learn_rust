use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your weight(kg):");
    io::stdin().read_line(&mut input).unwrap();
    let weight_kg :f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight_kg);

    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    (weight_on_earth/9.81) * 3.711
}