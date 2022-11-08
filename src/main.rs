use std::io;

fn main() -> io::Result<()> {
    println!("Temperature transformer!");

    let mut mode = String::new();
    let mut input = String::new();

    println!("What do you want to transform?\nType f for F° to C°, type c for C° to F°, type quit for exiting");

    let stdin = io::stdin();

    stdin.read_line(&mut mode)?;

    
    println!("Input the value");
    stdin.read_line(&mut input)?;

    let mode = mode.trim();
    let input = input.trim();



    println!("{mode} {input}");

    Ok(())
}

/* fn fahrenheit_to_celsius(x: f32) -> f32 {
    (x - 32.0) * (5.0/9.0)
} */

/* fn celsius_to_fahrenheit(x: f32) -> f32 {
    (x * (9.0/5.0)) + 32.0
} */