extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Piston
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

// Rest
use serde::Deserialize;
use core::time;
use std::{fs, thread::sleep};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
struct Config {
    initial_cells: Vec<Vec<i32>>,
    grid_size: GridSize,
}

#[derive(Debug, serde::Deserialize)]
struct GridSize {
    width: usize,
    height: usize,
}

lazy_static! {
    static ref CONFIG: Config = load_config();
}

fn load_config() -> Config {
    let toml_str = fs::read_to_string("config.toml").expect("Failed to read config.toml file");
    toml::from_str(&toml_str).expect("Failed to deserialize config.toml")
}

fn init_field() -> Vec<Vec<bool> > {
    // Config
    let config = &*CONFIG;

    // game_grid aus grid_size initialisieren, bool 2D array mit toten Zellen
    let mut game_grid: Vec<Vec<bool>> = vec![vec![false; config.grid_size.width]; config.grid_size.height];

    // Lebendige Zellen in game_grid als true repräsentieren
    for cell_coordinates in &config.initial_cells {
        if cell_coordinates.len() >= 2 {
            let x = cell_coordinates[0] as usize;
            let y = cell_coordinates[1] as usize;

            if x < game_grid.len() && y < game_grid[x].len() {
                game_grid[x][y] = true;
            }
        }
    }

    game_grid
}

fn next_gen(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // Config
    let config = &*CONFIG;

    let mut next: Vec<Vec<bool>> = vec![vec![false; config.grid_size.width]; config.grid_size.height];
    let mut y: usize = 0;
    let mut x: usize;
    let mut alive: u8;

    for row in grid {
        x = 0;
        y+=1;
        for cell in row {
            x+=1;
            alive = 0;
            if check(x-1, y-1, &grid) {alive+=1;}
            if check(x-1, y, &grid) {alive+=1;}
            if check(x-1, y+1, &grid) {alive+=1;}
            if check(x, y-1, &grid) {alive+=1;}
            if check(x, y+1, &grid) {alive+=1;}
            if check(x+1, y-1, &grid) {alive+=1;}
            if check(x+1, y, &grid) {alive+=1;}
            if check(x+1, y+1, &grid) {alive+=1;}

            if !*cell && alive==3 {next[y-1][x-1]=true;}
            if *cell && alive>1 && alive<4 {next[y-1][x-1]=true;}
        }
    }

  return next;
}

fn check(mut x: usize, mut y: usize, grid: &Vec<Vec<bool>>) -> bool {
    // Config
    let config = &*CONFIG;

    if y<1 {y=config.grid_size.height;}
    if y>config.grid_size.height {y=1;}
    if x<1 {x=config.grid_size.width;}
    if x>config.grid_size.width {x=1;}

    return grid[y-1][x-1];
}

pub struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, args: &RenderArgs, game_grid: &Vec<Vec<bool>>) {
        use graphics::*;

        const ALIVE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; // Grün
        const DEAD_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // Grau
        const GRID_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0]; // Gitterfarbe
        const CELL_SIZE: f64 = 25.0;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(DEAD_COLOR, gl);

            for (i, row) in game_grid.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let color = if cell { ALIVE_COLOR } else { DEAD_COLOR };

                    let x = (j as f64) * CELL_SIZE;
                    let y = (i as f64) * CELL_SIZE;

                    let square = rectangle::square(x, y, CELL_SIZE);
                    let transform = c.transform;

                    rectangle(color, square, transform, gl);
                }
            }

            let transform = c.transform;

            for i in 0..game_grid.len() + 1 {
                let y = (i as f64) * CELL_SIZE;
                line(GRID_COLOR, 0.5, [0.0, y, args.window_size[0], y], transform, gl);
            }

            for j in 0..game_grid[0].len() + 1 {
                let x = (j as f64) * CELL_SIZE;
                line(GRID_COLOR, 0.5, [x, 0.0, x, args.window_size[1]], transform, gl);
            }
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Gamegrid init
    let mut game_grid = init_field();
    
    // Create a Glutin window.
    let config = &*CONFIG;

    let mut window: Window = WindowSettings::new::<&str, (u32, u32)>("Game of Life", (config.grid_size.width as u32 * 25, config.grid_size.height as u32 * 25).into())
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game{
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &game_grid);
        }

        if let Some(args) = e.update_args() {
            game_grid = next_gen(&game_grid);
            sleep(time::Duration::from_millis(150));
        }
    }
}