# Rust Snake Game

A simple Snake game implemented in Rust with a graphical user interface.

## Game

<p align="center">
  <img alt="Snake beginning" src="assets/snake beginning.png"/>
  <img alt="Snake during" src="assets/snake during.png"/>
  <img alt="Snake final" src="assets/snake final.png"/>
  <img alt="Snake gameplay" src="assets/snake gameplay.gif"/>
</p>

## Dependencies

This project uses the following third-party crates:

- [rand](https://crates.io/crates/rand)
- [piston_window](https://crates.io/crates/piston_window)
- [piston2d-opengl_graphics](https://crates.io/crates/piston2d-opengl_graphics)

## Installation

To install Rust, follow the instructions provided on the [official website](https://www.rust-lang.org/tools/install).

To build and run the Snake game, clone this repository and run the following command in the root directory:

```bash
cargo run
```

## Gameplay

Once you run the program, the game will start automatically. The objective of the game is to control the snake by moving it around the screen and collecting food items. As the snake eats more food, it grows in length, making the game more challenging. If the snake collides with itself or the edges of the screen, the game ends.

## Assets

The only asset used in this project is the font "FiraSans-Regular.ttf", which is available for free on FFonts.net.

## Contributions

Contributions to this project are welcome. Please feel free to create a pull request or submit an issue if you have any suggestions or encounter any problems.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
