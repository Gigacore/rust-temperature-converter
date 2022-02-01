use dialoguer::Select;
use std::io;

fn main() -> std::io::Result<()> {
    let selection = Select::new()
        .with_prompt("\nHow do you like to convert? Use arrow keys to select")
        .item("Celsius to Fahrenheit")
        .item("Fahrenheit to Celsius")
        .interact()?;

    println!("\nEnter the temperature value to convert:");

    let mut temperature = String::new();

    io::stdin()
      .read_line(&mut temperature)
      .expect("Failed to read line");

    let temperature: f32 = temperature.trim().parse().expect("Please type a number!");

    println!("{}", temperature);

    if selection == 0 {
      println!("{}°C in Fahrenheit is {}°F", temperature, (temperature * 1.8) + 32.0);
    };

    if selection == 1 {
      println!("{}°F in Celsius is {}°C", temperature, (temperature - 32.0) * 0.555555555555556);
    };

    Ok(())
}