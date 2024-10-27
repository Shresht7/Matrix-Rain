# Matrix-Rain
-------------

The matrix-rain effect in the terminal.

![demo](./demo.gif)

> [!NOTE]
>
> Just a simple program I made while learning Rust!

## Features

- Multiple character symbol sets, including original `Katakana` symbols, `binary`, `decimal`, `mathematical` symbols, `ASCII` characters, `Braille` patterns, and `emojis`.
- Customizable stream colors and gradients.
- Adjustable frame rate, stream count, and spacing.
- Option to leave a trail of characters as the streams pass by.
- Randomized symbol switching for added visual interest.

---

## Installation

To install and run Matrix-Rain, you will need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can clone the repository and build the project:

```sh
git clone https://github.com/Shresht7/matrix-rain.git
cd matrix-rain
cargo build --release
```

To run the project, use the following command:

```sh
cargo run --release
```

---

## Usage Instructions

To use Matrix-Rain, you can run the project with the following command:

```sh
cargo run --release -- [OPTIONS]
```

Here are some examples of how to use the different configuration options:

- To use the original Katakana symbols with the default settings:
  ```sh
  cargo run --release
  ```

- To use binary symbols with a custom stream color and gradient:
  ```sh
  cargo run --release -- --mode Binary --stream-color 0,255,70 --stream-color-gradient-factor 0.5
  ```

- To use ASCII characters with a higher frame rate and leave a trail:
  ```sh
  cargo run --release -- --mode ASCII --fps 120 --leave-trail
  ```

You can combine multiple options to customize the matrix-rain effect to your liking.

### Configuration Options

The different configuration options available in this project are:

* `mode`: The character symbol set to use. Valid options include "Original", "Binary", "Decimal", "Math", "ASCII", "Braille", "Emoji", and custom sets like "abc123".
* `stream_color`: The color of the streaming entities, specified as an RGB value (e.g., "0,255,70").
* `stream_color_gradient_factor`: The multiplier that describes the extent of the gradient in the stream color.
* `leading_entity_color`: The color of the leading entity in a stream, specified as an RGB value (e.g., "200,255,200").
* `leave_trail`: A boolean option to leave the trail intact as the streams pass by.
* `fps`: The frame rate to run at, specified as the number of frames per second.
* `stream_min_count`: The minimum number of entities per stream.
* `stream_max_count`: The maximum number of entities per stream.
* `stream_spacing`: The spacing between the streams, specified as the number of columns between each stream.
* `switch_interval`: The maximum number of seconds within which an entity randomly switches its symbol.

These options are defined in the `src/config.rs` file.

---

## Contributing

Contributions are welcome! If you would like to contribute to this project, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bugfix.
2. Write tests for your changes, if applicable.
3. Ensure that all tests pass and the code is properly formatted.
4. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the [MIT License](./LICENSE). See the [`LICENSE`](./LICENSE) file for more information.
