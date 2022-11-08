use std::io;

fn main() -> io::Result<()> {
    println!("Temperature converter!");
    let stdin = io::stdin();
    
    loop {
        println!("What do you want to convert?\nf for F° to C°, c for C° to F°, quit for exiting");
        let mut mode = String::new();
        let mut input = String::new();
    
    
        stdin.read_line(&mut mode)?;
        let mode: &str = mode.trim();

        if mode.eq("quit") {
            break;
        } else if mode.eq("f") {
            println!("Input a Fahrenheit value.");
        } else if mode.eq("c") {
            println!("Input a Celsius value");
        } else {
            continue;
        }

        stdin.read_line(&mut input)?;

        let input: f32 = input.trim().parse().expect("Please input a valid number.");

        let result: f32 = if mode.eq("f") {fahrenheit_to_celsius(input)} else {celsius_to_fahrenheit(input)};
        let unit = if mode.eq("f") {"C"} else {"F"};

        println!("{:.2} {unit}°", result);
    }

    Ok(())
}

fn fahrenheit_to_celsius(x: f32) -> f32 {
    (x - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(x: f32) -> f32 {
    (x * (9.0/5.0)) + 32.0
}