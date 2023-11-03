# DS18B20-Temperature-Sensor-Rust
The DS18B20-Temperature-Sensor-Rust is an embedded application built with safety in mind, designed to read data from a DS18B20 sensor and run on both host systems and Cortex-M microcontrollers. This project uses Rust, a language that emphasizes zero-cost abstractions and fearless concurrency, providing an enticing alternative to traditional embedded languages such as C and C++.

**Fearless Concurrency in Embedded Systems**: The project showcases the distinct advantages offered by Rust in developing embedded systems. With its focus on safety and zero-cost abstractions, budding Rustaceans are provided a unique insight into the world of embedded systems.

**Hardware Abstraction Layer (HAL) Deep Dive**: The project serves as an excellent reference for the design and implementation of HAL, a critical component in embedded systems. The HAL interface facilitates the development of reusable and hardware-agnostic high-level application code.

**Cross-compilation Flex**: This project emphasizes the crucial aspect of cross-compilation in embedded systems development. It provides a clear demonstration of setting up a Rust project, that is focused on the complexities of cross-compilation.

**Conditional Compilation Magic**: The project also showcases the power of conditional compilation in Rust, using its attribute features to switch between different elements during compile time. This versatility offers an advantage when targeting various environments.

**Sensor Interaction Fundamentals**: While the code simulates an interaction with a DS18B20 sensor, it opens up a plethora of possibilities for tangible sensor interaction in a vast range of embedded applications.

In summary, the DS18B20-Temperature-Sensor-Rust project invites you to explore the captivating world of embedded systems through the Rust perspective. Go ahead and delve into the code, expand your knowledge, and uncover the mysteries of embedded systems!

The "sensor.rs" file in this example outlines a DS18B20 temperature sensor using Rust.

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
