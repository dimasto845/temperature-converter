pub fn fahrenheit_to_celsius(x: f32) -> f32 {
    ((x - 32_f32) * (5_f32 / 9_f32) * 10000_f32).round() / 10000_f32
}

pub fn celsius_to_fahrenheit(x: f32) -> f32 {
    (x * (9_f32 / 5_f32)) + 32_f32
}

pub fn celsius_to_kelvin(x: f32) -> f32 {
    x + 273.15
}

pub fn kelvin_to_celsius(x: f32) -> f32 {
    x - 273.15
}

pub fn fahrenheit_to_kelvin(x: f32) -> f32 {
    (((x - 32_f32) / 1.8 + 273.15) * 1000_f32).round() / 1000_f32
}

pub fn kelvin_to_fahrenheit(x: f32) -> f32 {
    (((x - 273.15) * 1.8 + 32_f32) * 100_f32).round() / 100_f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_to_c_works() {
        assert_eq!(fahrenheit_to_celsius(0_f32), -17.7778);
        assert_eq!(fahrenheit_to_celsius(100_f32), 37.7778);
        assert_eq!(fahrenheit_to_celsius(1000_f32), 537.7779);
        assert_eq!(fahrenheit_to_celsius(5000_f32), 2760.0002);
        assert_eq!(fahrenheit_to_celsius(69420_f32), 38548.889);
    }

    #[test]
    fn c_to_f_works() {
        assert_eq!(celsius_to_fahrenheit(0_f32), 32_f32);
        assert_eq!(celsius_to_fahrenheit(100_f32), 212_f32);
        assert_eq!(celsius_to_fahrenheit(1000_f32), 1832_f32);
        assert_eq!(celsius_to_fahrenheit(5000_f32), 9032_f32);
        assert_eq!(celsius_to_fahrenheit(69420_f32), 124988_f32);
    }

    #[test]
    fn c_to_k_works() {
        assert_eq!(celsius_to_kelvin(0_f32), 273.15);
        assert_eq!(celsius_to_kelvin(100_f32), 373.15);
        assert_eq!(celsius_to_kelvin(1000_f32), 1273.15);
        assert_eq!(celsius_to_kelvin(5000_f32), 5273.15);
        assert_eq!(celsius_to_kelvin(69420_f32), 69693.15);
    }

    #[test]
    fn k_to_c_works() {
        assert_eq!(kelvin_to_celsius(0_f32), -273.15);
        assert_eq!(kelvin_to_celsius(100_f32), -173.15);
        assert_eq!(kelvin_to_celsius(1000_f32), 726.85);
        assert_eq!(kelvin_to_celsius(5000_f32), 4726.85);
        assert_eq!(kelvin_to_celsius(69420_f32), 69146.85);
    }

    #[test]
    fn f_to_k_works() {
        assert_eq!(fahrenheit_to_kelvin(0_f32), 255.372);
        assert_eq!(fahrenheit_to_kelvin(100_f32), 310.928);
        assert_eq!(fahrenheit_to_kelvin(1000_f32), 810.928);
        assert_eq!(fahrenheit_to_kelvin(5000_f32), 3033.15);
        assert_eq!(fahrenheit_to_kelvin(69420_f32), 38822.039);
    }

    #[test]
    fn k_to_f_works() {
        assert_eq!(kelvin_to_fahrenheit(0_f32), -459.67);
        assert_eq!(kelvin_to_fahrenheit(100_f32), -279.67);
        assert_eq!(kelvin_to_fahrenheit(1000_f32), 1340.33);
        assert_eq!(kelvin_to_fahrenheit(5000_f32), 8540.33);
        assert_eq!(kelvin_to_fahrenheit(69420_f32), 124496.33);
    }
}
