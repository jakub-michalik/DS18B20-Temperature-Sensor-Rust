use crate::hal::sensor::DS18B20Sensor;

fn main() {
    // Initialize DS18B20 sensor
    let mut sensor = DS18B20Sensor::new();

    // Read temperature from DS18B20
    sensor.read();

    // Get the temperature
    let the_temperature = sensor.get_temperature();

    // Print the temperature
    println!("Temperature: {}", the_temperature);
}
