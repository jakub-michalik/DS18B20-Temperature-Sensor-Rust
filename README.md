# DS18B20-Temperature-Sensor-Rust
Rust Embedded application for reading temperature from a DS18B20 sensor, designed to run on both host systems and Cortex-M microcontrollers.

This repository is a hands-on invitation to the brave new world of Embedded Systems development leveraging the rustacean landscape. 

**'Fearless Concurrency' in Embedded Systems**: Rust's emphasis on zero-cost abstractions, safety, and fearless concurrency makes it an enticing alternative to traditional embedded languages like C and C++. This repository provides fledgling rustaceans a launchpad into the embedded system's milieu.

**Hardware Abstraction Layer (HAL) Deep Dive**: The repository masterfully exemplifies the design and implementation of a HAL, which is an essential piece of the embedded systems puzzle. The HAL interface helps in writing a reusable and hardware-agnostic higher-level application code.

**Cross-compilation Flex**: In the world of embedded systems, cross-compilation is the name of the game. The example in the repository provides a lucid illustration of setting up a rust project encompassing the intricacies of cross-compilation.

**Conditional Compilation Magic**: The repository also gives a sneak peek into the awesomeness of conditional compilation in Rust. Using Rust's cfg attribute and Cargo features, it shows how to switch between different features at compile time - truly a boon when you're targeting different environments!

**Sensor Interaction Basics**: Though the code mimics a hypothetical interaction with a real-world DS18B20 sensor, it opens up a cosmos of possibilities for real sensor interaction in myriad embedded applications.

In a nutshell, dig into this repository, decode the enigma of embedded systems through the rustacean lens and expand your understanding with each line of code!

The "sensor.rs" file in this example defines a DS18B20 temperature sensor in Rust.

```rust
pub struct DS18B20Sensor {
    temperature: f32,
}
```
This part is the declaration of a public data structure `DS18B20Sensor` that represents a sensor object in our code. It contains temperature as a field of type `f32` (32-bit floating point number) to store the temperature readings from the sensor.

As the struct is declared as `pub`, it means it can be accessed from other modules in the project.

```rust
impl DS18B20Sensor {
    pub fn new() -> DS18B20Sensor {
        DS18B20Sensor {
            temperature: 0.0,
        }
    }
```
This is the implementation block for the DS18B20Sensor struct. We define a method `new()`, which is a common convention in Rust for functions that create and return a new instance of a struct. In this `new()` function, an instance of `DS18B20Sensor` is created with the temperature field initially set to `0.0`.

This setup provides the basic architecture for representing a DS18B20 sensor and creating new sensor instances in your Rust code.

Please note: This is a very basic representation and in real application, you would have more complex code dealing with direct hardware communication to be able to interact with the actual DS18B20 sensor, fetch the temperature etc.
