use macroquad::prelude::*;
use std::thread::sleep;
use std::time::Duration;

const MATRIX_SIZE: usize = 64;
const MATRIX_GUI_UPSCALE_FACTOR: f32 = 8.;

fn calc_neighbour_cells(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbouring_cells = Vec::new();
    let (x, y) = (pos.0 as i32, pos.1 as i32);

    for xoff in -1..=1 {
        for yoff in -1..=1 {
            let (new_x, new_y) = (x + xoff, y + yoff);

            let is_new_x_in_bounds = new_x >= 0 && new_x < MATRIX_SIZE as i32;
            let is_new_y_in_bounds = new_y >= 0 && new_y < MATRIX_SIZE as i32;
            let is_new_pos_self = xoff == 0 && yoff == 0;

            if is_new_x_in_bounds && is_new_y_in_bounds && !is_new_pos_self {
                neighbouring_cells.push((new_x as usize, new_y as usize));
            }
        }
    }

    neighbouring_cells
}

fn apply_conway_rules(matrix: &Matrix, pos: (usize, usize)) -> bool {
    // RULES
    //Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    //Any live cell with two or three live neighbours lives on to the next generation.
    //Any live cell with more than three live neighbours dies, as if by overpopulation.
    //Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    
    let cell_state = matrix.data[pos.0][pos.1];

    let neighbours = calc_neighbour_cells(pos);
    let living_neighbours_count = neighbours.iter().filter_map(
        |(x, y)|
        match matrix.data[*x][*y] {
            true => Some(true),
            false => None
        }
    ).collect::<Vec<bool>>().len();

    match (cell_state, living_neighbours_count) {
        (true, 0 | 1 | 4..) => false, // RULE 1 and 3
        (true, 2 | 3) => true, // RULE 2
        (false, 3) => true, // RULE 4
        (_, _) => false
    }
}

struct Matrix {
    data: [[bool; 64]; 64]
}

impl Matrix {
    pub fn new() -> Self {
        let mut data = [[false; 64]; 64];

        //CREATE INITIAL DATA
        data[10][11] = true;
        data[10][12] = true;
        data[10][13] = true;

        Matrix {
            data
        }
    }

    pub fn update(&mut self) {
        let mut new_matrix = self.data;
        for x in 0..self.data.len() {
            for y in 0..self.data[x].len() {
                new_matrix[x][y] = apply_conway_rules(self, (x, y));
            }
        }
        self.data = new_matrix;
    }

    pub fn draw(&self) {
        for x in 0..self.data.len() {
            for y in 0..self.data[0].len() {
                if self.data[x][y] {
                    let x = x as f32 * MATRIX_GUI_UPSCALE_FACTOR;
                    let y = y as f32 * MATRIX_GUI_UPSCALE_FACTOR;
                    let w = MATRIX_GUI_UPSCALE_FACTOR;
                    let h = MATRIX_GUI_UPSCALE_FACTOR;
                    draw_rectangle(x, y, w, h, WHITE);
                }
            }
        }
    }
}

#[macroquad::main("GameOfLife")]
async fn main() {
    let mut matrix = Matrix::new();

    let size = MATRIX_SIZE as f32 * MATRIX_GUI_UPSCALE_FACTOR;
    request_new_screen_size(size, size);
    next_frame().await;

    loop {
        sleep(Duration::from_secs_f32(1./2.));
        matrix.update();

        // drawing
        clear_background(BLACK);

        matrix.draw();

        next_frame().await;
    }
}
