use sfml::graphics::{
    Color, RenderTarget, RenderWindow, Transformable, CircleShape, Shape
};
use sfml::system::{Vector2i, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};
use rand::Rng;

struct Board {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}

impl Board {
    fn new(col: usize, row: usize) -> Board {
        let vec = vec![false; col * row];
        Board{ rows: row, cols: col, data: vec }
    }

    fn get(&self, col: usize, row: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            panic!("Out of bounds");
        }
        let offset = row * self.cols + col;
        return self.data[offset];
    }

    fn set(&mut self, col: usize, row: usize, value: bool) {
        if row >= self.rows || col >= self.cols {
            panic!("Out of bounds");
        }
        let offset = row * self.cols + col;
        self.data[offset] = value;
    }

    fn clear(&mut self) {
        for i in &mut self.data {
            *i = false;
        }
    }

    fn randomize(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            self.data[rng.gen_range(0..self.cols * self.rows)] = true;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut board = Board::new(10, 3);
        board.set(8, 1, true);
        assert!(board.get(8, 1));
        assert!(board.get(0, 0) == false);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let board = Board::new(10, 3);
        board.get(1, 10);
    }

    #[test]
    fn test_count_neighbours() {
        let mut board = Board::new(10, 5);
        // 8, 2 is empty but has three neighbours below it
        board.set(7, 3, true);
        board.set(8, 3, true);
        board.set(9, 3, true);
        let nc = count_neighbours(&board, 8, 2);
        assert!(nc == 3);
    }

    #[test]
    fn test_count_neighbours_edge() {
        // literal edge case :)
        let mut board = Board::new(3, 3);
        board.set(0, 1, true);
        board.set(1, 1, true);
        let nc = count_neighbours(&board, 0, 0);
        assert!(nc == 2);
    }

    #[test]
    fn test_generation_spawn() {
        // an empty cell with three neighbours should spawn
        let mut board = Board::new(10, 5);
        // 8, 2 is empty but has three neighbours below it
        board.set(7, 3, true);
        board.set(8, 3, true);
        board.set(9, 3, true);
        assert!(board.get(8, 2) == false);
        next_generation(&mut board);
        assert!(board.get(8, 2) == true);
    }

    #[test]
    fn test_generation_die() {
        // a cell with no neighbours should die
        let mut board = Board::new(10, 5);
        board.set(7, 3, true);
        next_generation(&mut board);
        assert!(board.get(7, 3) == false);
    }

    #[test]
    fn test_generation_survive() {
        // an existing cell with two or three neighbours should survive
        let mut board = Board::new(10, 5);
        board.set(8, 2, true);
        board.set(7, 3, true);
        board.set(8, 3, true);
        board.set(9, 3, true);
        next_generation(&mut board);
        assert!(board.get(8, 2) == true);
    }

    #[test]
    fn test_generation_overpopulation() {
        // an existing cell with more than three neighbours should die
        let mut board = Board::new(10, 5);
        board.set(8, 2, true);
        board.set(9, 2, true);
        board.set(7, 3, true);
        board.set(8, 3, true);
        board.set(9, 3, true);
        next_generation(&mut board);
        assert!(board.get(8, 2) == false);
    }

}

fn display_board( window : &mut RenderWindow, board : &Board, cell_size: u32 ) {
    let cols = board.cols;
    let rows = board.rows;
    let mut rng = rand::thread_rng();
    for row in 0..rows {
        for col in 0..cols {
            if board.get(col, row) == true {
                let mut circ = CircleShape::new((cell_size as f32 / 2.0) as f32, 30);
                let green = rng.gen_range(96..=150);
                circ.set_fill_color(Color::rgb(0, green, 0));
                circ.set_position(Vector2f::new(
                    (col * cell_size as usize) as f32,
                    (row * cell_size as usize) as f32));
                window.draw(&circ);
            }
        }
    }
}

fn count_neighbours( board : &Board, col: usize, row: usize ) -> i32 {
    let mut count = 0;
    let min_col = if col > 0 { col - 1 } else { 0 };
    let max_col = if col < board.cols - 1 { col + 1 } else { board.cols - 1 };
    let min_row = if row > 0 { row - 1 } else { 0 };
    let max_row = if row < board.rows - 1 { row + 1 } else { board.rows - 1 };
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if (c == col) && (r == row) {
                continue;
            }
            if board.get(c, r) {
                count += 1;
            }
        }
    }
    count
}

fn next_generation( board : &mut Board ) {
    let cols = board.cols;
    let rows = board.rows;
    let mut new_board = Board::new(cols, rows);
    for row in 0..rows {
        for col in 0..cols {
            let c = count_neighbours(&board, col, row);
            if board.get(col, row) {
                // occupied slot
                if c == 2 || c == 3{
                    // an existing cell with 2-3 neighbours
                    // will just continue to live
                    new_board.set(col, row, true);
                } else {
                    new_board.set(col, row, false);
                }
            } else {
                // non-occupied slot
                if c == 3 {
                    new_board.set(col, row, true);
                }
            }
        }
    }
    *board = new_board;
}

fn main() {
    let screen_width  = VideoMode::desktop_mode().width;
    let screen_height = VideoMode::desktop_mode().height;
    let ratio: f32 = screen_width as f32 / screen_height as f32;

    let window_width = 1920;
    let window_height = ( window_width as f32 /ratio ) as u32;
    let cell_size = 16;

    let mut window = RenderWindow::new(
        (window_width, window_height),
        "Conway's Life",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(16);
    window.set_position(Vector2i::new(50, 50));

    let rows = (window_height / cell_size) as usize;
    let cols = (window_width / cell_size) as usize;
    let mut board = Board::new(cols, rows);

    board.randomize(2000);

    // Main Loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyReleased { code, .. } => {
                    match code {
                        Key::Escape => {
                            window.close();
                        },
                        Key::Q => {
                            window.close();
                        },
                        Key::R => {
                            board.clear();
                            board.randomize(2000);
                        },
                        _ => {}
                    }
                },
                _ => {} // ignore other events
            }
        }
        window.clear(Color::BLACK);
        display_board(&mut window, &board, cell_size);
        next_generation(&mut board);
        window.display();
    }
}
