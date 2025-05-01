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

fn main() {
    let grid: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    let mut buffer: String = String::new();
    
    display(&grid, &mut buffer);
}
