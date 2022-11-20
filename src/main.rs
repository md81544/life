use sfml::graphics::{
    Color, RenderTarget, RenderWindow, Transformable, CircleShape, Shape
};
use sfml::system::{Vector2i, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};
use rand::Rng;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   fullscreen: bool,
}

struct Board {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
    colours: Vec<u8>,
}

impl Board {
    fn new(col: usize, row: usize) -> Board {
        let vec = vec![false; col * row];
        let mut colours = vec![0; col * row];
        let mut rng = rand::thread_rng();
        for i in &mut colours {
            *i = rng.gen_range(128..=255);
        }
        Board{ rows: row, cols: col, data: vec, colours: colours }
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

    fn get_colour(&self, col: usize, row: usize) -> u8 {
        if row >= self.rows || col >= self.cols {
            panic!("Out of bounds");
        }
        let offset = row * self.cols + col;
        return self.colours[offset];
    }

    fn set_colour(&mut self, col: usize, row: usize, value: u8) {
        if row >= self.rows || col >= self.cols {
            panic!("Out of bounds");
        }
        let offset = row * self.cols + col;
        self.colours[offset] = value;
    }

    fn clear(&mut self) {
        for i in &mut self.data {
            *i = false;
        }
    }

    fn randomise(&mut self, count: usize) {
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
    fn test_colours() {
        let mut board = Board::new(3, 3);
        board.set_colour(0, 0, 64);
        board.set_colour(2, 2, 128);
        assert!(board.get_colour(0, 0) == 64);
        // 1, 1 is unset; it should have been initialised to a
        // random number in the range 128..=255
        assert!(board.get_colour(1, 1) >= 128);
        assert!(board.get_colour(2, 2) == 128);
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

fn display_board( window : &mut RenderWindow, board : &mut Board, cell_size: u32 ) {
    let cols = board.cols;
    let rows = board.rows;
    for row in 0..rows {
        for col in 0..cols {
            let cell_present = board.get(col, row);
            let radius = cell_size as f32 / if cell_present { 2.0 } else { 3.0 };
            let mut circ = CircleShape::new(radius, 30);
            circ.set_origin((radius, radius));
            circ.set_position(Vector2f::new(
                (col * cell_size as usize + (cell_size / 2) as usize) as f32,
                (row * cell_size as usize + (cell_size / 2) as usize) as f32));
            if cell_present {
                circ.set_fill_color(Color::rgb(0, board.get_colour(col, row), 0));
            } else {
                circ.set_fill_color(Color::rgb(32, 64, 32));
            }
            window.draw(&circ);
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
            let colour = board.get_colour(col, row);
            new_board.set_colour(col, row, colour);
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

    let args = Args::parse();

    let screen_width  = VideoMode::desktop_mode().width;
    let screen_height = VideoMode::desktop_mode().height;
    let ratio: f32 = screen_width as f32 / screen_height as f32;

    // There's a bug in SFML on Mac which returns a zero-element
    // array of fullscreen modes, which causes a segfault when
    // using fullscreen. So we count the modes and only use
    // fullscreen if the count is > 0
    let fs_count = VideoMode::fullscreen_modes().into_iter().count();

    let window_width = if screen_width >= 1920 { 1920 } else { screen_width };
    let window_height = ( window_width as f32 /ratio ) as u32;
    let cell_size = 16;

    let mut window = RenderWindow::new(
        (window_width, window_height),
        "Conway's Life",
        if fs_count > 0 && args.fullscreen {Style::FULLSCREEN} else {Style::DEFAULT},
        &ContextSettings::default(),
    );
    window.set_framerate_limit(16);
    window.set_position(Vector2i::new(50, 50));
    window.set_mouse_cursor_visible(false);

    let rows = (window_height / cell_size) as usize;
    let cols = (window_width / cell_size) as usize;
    let mut board = Board::new(cols, rows);

    board.randomise(2000);

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
                            board.randomise(2000);
                        },
                        _ => {}
                    }
                },
                _ => {} // ignore other events
            }
        }
        window.clear(Color::BLACK);
        display_board(&mut window, &mut board, cell_size);
        next_generation(&mut board);
        window.display();
    }
}
