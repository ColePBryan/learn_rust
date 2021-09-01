fn main() {
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    return (weight_on_earth/9.81) * 3.711;
}