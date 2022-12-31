This is part of a series of fun little programs I write to learn rust:

- [Alder](https://github.com/coijanovic/alder): Conway's Game of Life
- [Balsa](https://github.com/coijanovic/balsa): Image to Emoji
- [Cedar](https://github.com/coijanovic/cedar): Auto snake game

# Cedar

Have you ever wanted to watch your computer play snake in a somewhat clumsy fashion?
Here you go!

`cargo run` and enjoy! 🐍

## Usage

You can choose the decision algorithm the snake uses via commandline flags.
The available options are:

- `--random` (default): The snake chooses a random (possible) direction for every step
- `--greedy`: The snake selects the (possible) direction which brings her closest to the food in the next step 
- `--angle`: If the food is in the same column as the snake, she goes down. 
  Otherwise, she goes right.

Use `--help` for more information.
