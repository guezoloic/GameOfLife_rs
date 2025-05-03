use std::{time::Duration, thread};

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

struct Cell {
    name: &'static str,
    array: Vec<(usize, usize)>
}

impl Cell {
    pub fn new(x: usize, y: usize, name: &'static str, directions_array: &[(i8, i8)]) -> Self {
        let mut cells_array: Vec<(usize, usize)> = Vec::new();

        for i in 0..directions_array.len() {
            let dx = x as i8 + directions_array[i].0;
            let dy = y as i8 + directions_array[i].1;

            if dx > 0 && dy > 0 {
                cells_array.push(( 
                    dx as usize,
                    dy as usize
                ));
            }
        }

        return Cell {name, array: cells_array};
    }

    pub fn get_array(&self) -> &Vec<(usize, usize)> {
        return &self.array;
    }

    pub fn get_name(&self) -> &'static str {
        return self.name;
    }

    pub fn add_grid(&self, grid: &mut [[bool; WIDTH]; HEIGHT]) {
        for (x, y) in &self.array {
            if *x < WIDTH && *y < HEIGHT {
                grid[*y][*x] = true;
            }
        }
    }
}

fn display(grid: &[[bool; WIDTH]; HEIGHT], buffer: &mut String) {
    buffer.clear();

    for row in grid {
        for &cell in row {
            buffer.push(if cell { 'X' } else { '.' });
            buffer.push(' ');
        }
        buffer.push('\n');
    }

    println!("{}", buffer);
}

fn count(grid: &[[bool; WIDTH]; HEIGHT], x: usize, y: usize) -> u8 { 

    let near:[(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
         (0, -1),           (0, 1),
        ( 1, -1),  (1, 0),  (1, 1)
    ];

    let mut count = 0;

    for (i, j) in near {
        let nx: isize = x as isize + j;
        let ny: isize = y as isize + i;

        if nx >= 0 && (WIDTH as isize) > nx && 
            ny >= 0 && (HEIGHT as isize) > ny {
            
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }

    return count;
}

fn generation(grid: &[[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_grid = [[false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let count = count(grid, x, y);
            new_grid[y][x] = match (grid[y][x], count) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    return new_grid;
}

fn main() {
    let mut grid: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    let mut buffer: String = String::new();

    let glider: Cell = Cell::new(14, 14, "glider", &[(1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]);
    glider.add_grid(&mut grid);

    loop {
        display(&grid, &mut buffer);
        grid = generation(&grid);
        thread::sleep(Duration::from_millis(500));
    }
}
