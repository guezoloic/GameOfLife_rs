const WIDTH: usize = 30;
const HEIGHT: usize = 30;

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
    
    // (i, j) coordinate
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
    
    // grid[14][13] = true;
    // grid[15][14] = true;
    // grid[13][15] = true;
    // grid[14][15] = true;
    // grid[15][15] = true;

    loop {
        display(&grid, &mut buffer);
        grid = generation(&grid);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
