<h1 align="center">Game of Life</h1>

<p align="center">
<strong> A simple implementation of Conway's Game of Life in Rust.</strong> <br> <em>Experience the magic of cellular automata directly in your terminal.</em>
</p>

<p align="center">
  <a href="https://www.rust-lang.org"><img alt="Rust" src="https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust&logoColor=white" /></a>
  <a href="https://github.com/guezoloic/GameOfLife"><img alt="GitHub Repo" src="https://img.shields.io/badge/Source-GitHub-181717?logo=github" /></a>
</p>

![glider](/media/glider.gif)

## Prerequisites

Before you begin, make sure [**Rust**](https://www.rust-lang.org) is installed on your machine.
```bash
rustc --version
cargo --version
```


## Installation
Clone and run the project locally:
```bash
git clone https://github.com/guezoloic/GameOfLife.git
cd GameOfLife
cargo build --release
cargo run
```

## Features
- Simple and efficient 2D grid-based simulation.
- Add your own patterns (glider, beacon, etc.)
- Console-based visualization.

## Usage

You can easily insert patterns in `main.rs` like: 

- **Glider:** 
    ```rust
    //  X   .   .
    //  .   X   X
    //  X   X   .

    Cell::new(14, 14, "glider", &[
        (-1, 1), (0, 0), (1, 0),
        (-1, -1), (0, -1),
    ]);
    ```

- **Blinker:**
    ```rust
    //  X
    //  X
    //  X

    // Blinker (horizontal):
    //  X   X   X

    Cell::new(10, 10, "blinker", &[
        (1, 0), (0, 0), (-1, 0)
    ]);
    ```

- **Beacon:**
    ```rust
    //  X   X   .   .
    //  X   X   .   .
    //  .   .   X   X
    //  .   .   X   X

    Cell::new(6, 6, "beacon", &[
        (-1, 1), (0, 1), (-1, 0), (0, 0),
        (-3, 3), (-2, 3), (-3, 2), (-2, 2)
    ]);
    ```

- **Toad:**
    ```rust
    //  .   X   
    //  X   X   
    //  X   X
    //  X   .
    Cell::new(24, 24, "toad", &[
        (-1, 1), (-1, 0), (-1, -1), 
        (0, 0), (0, -1), (0, -2)
    ]);
    ```

You can create additional patterns by modifying the positions or creating new shapes. The Cell::new(x, y, name, array) constructor accepts a starting position (x, y) and a list of directions (i8, i8) that represent the neighbors of the cell.
If you need inspiration or want to explore existing patterns, the [ConwayLife](https://conwaylife.com) website offers a comprehensive library of well-documented examples.

## History

[The Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) was created by mathematician John Conway in 1970 as a simple simulation of how life might evolve. It follows a few basic rules where cells live, die, or are born depending on their neighbors. Despite its simplicity, the system can lead to incredibly complex and fascinating patterns. It’s a classic example of how simple rules can create unexpected and beautiful behaviors.

## Contributing 
Feel free to fork the repository, submit issues or pull requests.
