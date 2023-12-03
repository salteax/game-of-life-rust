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
    interface: InterfaceType,
}

#[derive(Debug, serde::Deserialize)]
struct GridSize {
    width: usize,
    height: usize,
}

#[derive(Debug, Deserialize)]
enum InterfaceType {
    Gui,
    Console,
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

fn print_game_grid(game_grid: &Vec<Vec<bool>>) {
    // Screen leeren und Cursor auf Position 1 1
    print!("\x1B[2J\x1b[1;1H");

    // Farben
    let alive_color = "\x1B[32m";
    let dead_color = "\x1B[31m";
    let reset_color = "\x1B[0m";

    // Zeichne oberer Rahmen
    let width = game_grid[0].len() * 2;
    println!("+{}+", "-".repeat(width));

    // Grid ausgeben
    for row in game_grid {
        print!("|");
        for &cell in row {
            if cell {
                print!("{}x{} ", alive_color, reset_color);
            } else {
                print!("{}.{} ", dead_color, reset_color);
            }
        }
        println!("|");
    }

    // Zeichne unterer Rahmen
    println!("+{}+", "-".repeat(width));
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

fn init_gui(config: &Config) -> (Window, Game) {
    let opengl = OpenGL::V3_2;

    let window: Window = WindowSettings::new::<&str, (u32, u32)>("Game of Life", (config.grid_size.width as u32 * 25, config.grid_size.height as u32 * 25).into())
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let app = Game {
        gl: GlGraphics::new(opengl),
    };

    (window, app)
}

fn run_gol_gui(window: &mut Window, app: &mut Game, game_grid: Vec<Vec<bool>>) {
    let event_settings = EventSettings {
        max_fps: 60,
        ups: 1,
        ups_reset: 100,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };

    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &game_grid);
        }

        if let Some(args) = e.update_args() {
            break;
        }
    }
}

fn run_gol_console(game_grid: Vec<Vec<bool>>, mut i: i32) -> i32 {
    print_game_grid(&game_grid);
    print!("Generation: ");
    print!("{}", i);
    println!();
    
    i + 1
}

fn main() {
    // Gamegrid init
    let mut game_grid = init_field();
    
    // Create a Glutin window.
    let config = &*CONFIG;

    let (mut window, mut app) = init_gui(config);

    let mut i = 1;
    loop {
        let updated_game_grid = next_gen(&game_grid);
        match config.interface {
            InterfaceType::Gui => {
                run_gol_gui(&mut window, &mut app, updated_game_grid.clone());
            }
            InterfaceType::Console => {
                i = run_gol_console(updated_game_grid.clone(), i);
            }
        }
        game_grid = updated_game_grid;
        sleep(time::Duration::from_millis(300));
    }
}