use converter::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() -> std::io::Result<()> {
    let items = vec!["Celsius", "Fahrenheit", "Kelvin"];

    let from = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What unit do you want to convert from?")
        .clear(true)
        .report(true)
        .items(&items)
        .default(0)
        .interact_opt()?;

    let to = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What unit do you want to convert it into?")
        .clear(false)
        .report(false)
        .items(&items)
        .default(0)
        .interact_opt()?;

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Please write the temperature")
        .report(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            if let Ok(_) = input.parse::<f32>() {
                Ok(())
            } else {
                Err("Please input a valid number")
            }
        })
        .interact_text()?;

    let input_number: f32 = input.parse().unwrap();

    match from {
        Some(input_index) => match to {
            Some(output_index) => {
                let mut result: f32 = 0_f32;

                if input_index == output_index {
                    result = input_number;
                } else {
                    match input_index {
                        0 => {
                            if output_index == 1 {
                                result = celsius_to_fahrenheit(input_number);
                            } else {
                                result = celsius_to_kelvin(input_number);
                            }
                        }
                        1 => {
                            if output_index == 0 {
                                result = fahrenheit_to_celsius(input_number);
                            } else {
                                result = fahrenheit_to_kelvin(input_number);
                            }
                        }
                        2 => {
                            if output_index == 0 {
                                result = kelvin_to_celsius(input_number);
                            } else {
                                result = kelvin_to_fahrenheit(input_number);
                            }
                        }
                        _ => {}
                    }
                }

                if items[output_index] == "Kelvin" {
                    println!("\nResult: {} Kelvin", result);
                } else {
                    println!("\nResult: {}° {}", result, items[output_index]);
                }
            }
            None => {}
        },
        None => {}
    }

    Ok(())
}
