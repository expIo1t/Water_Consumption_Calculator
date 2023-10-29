use dialoguer::Input;

fn main() {
    // Prompt the user to enter their weight (from 20 to 150 kg):
    let weight: f32 = Input::new()
        .with_prompt("Enter your weight (from 20 to 150 kg)")
        .interact_text()
        .unwrap();

    // Prompt the user to enter their physical activity (in hours, from 0 to 8):
    let activity: f32 = loop {
        let input: String = Input::new()
            .with_prompt("Enter your physical activity (in hours, from 0 to 8)")
            .interact_text()
            .unwrap();

        // Parse the input into a floating-point number and check if it's within the valid range:
        match input.parse() {
            Ok(num) if (0.0..=8.0).contains(&num) => break num,
            _ => println!("Invalid input. Please enter a number from 0 to 8."),
        }
    };

    // Calculate the daily water consumption norm based on weight and activity:
    let water_norm = calculate_water_norm(weight, activity);

    // Display the calculated water consumption norm:
    println!(
        "Your daily water consumption norm is: {} liters",
        water_norm
    );
}

// Algorithm:
fn calculate_water_norm(weight: f32, activity: f32) -> f32 {
    // Calculate the water consumption based on weight and whether the weight is within the normal range:
    if weight >= 20.0 && weight <= 150.0 {
        // Calculate the recommended water consumption based on the weight:
        let base_water_norm = if weight <= 30.0 {
            weight * 0.03 // 30 ml per 1 kg of ideal body weight:
        } else {
            30.0 * 0.03 * weight / 30.0 // 30 ml per 1 kg of recommended weight for overweight individuals
        };

        // Adjust the water consumption based on the activity level:
        let activity_ratio = activity / 8.0;
        let water_norm = base_water_norm * (1.0 + activity_ratio);
        water_norm
    } else {
        // If the weight is outside the valid range, return a default value:
        0.0
    }
}
