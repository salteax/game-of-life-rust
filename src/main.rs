extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Piston
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;

// Rest
use serde::Deserialize;
use core::time;
use std::time::Instant;
use std::{fs, thread::sleep};
use lazy_static::lazy_static;

mod backend;

#[derive(Debug, Deserialize)]
struct Config {
    interface: InterfaceType,
    max_fps: u64,
    ups: u64,
    initial_cells: Vec<Vec<i32>>,
    grid_size: GridSize,
}

#[derive(Debug, serde::Deserialize)]
pub struct GridSize {
    width: usize,
    height: usize,
}

#[derive(Debug, Deserialize)]
enum InterfaceType {
    Gui,
    Console,
    Speed,
    SuperSpeed
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

pub struct Game {
    gl: GlGraphics,
    is_paused: bool,
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

            // Zellen zeichnen
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

            // Grid zeichnen
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

fn init_gui(config: &Config) -> (PistonWindow, Game) {
    let opengl = OpenGL::V3_2;

    let window: PistonWindow = WindowSettings::new::<&str, (u32, u32)>("Game of Life", (config.grid_size.width as u32 * 25, config.grid_size.height as u32 * 25).into())
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let app = Game {
        gl: GlGraphics::new(opengl),
        is_paused: false,
    };

    (window, app)
}

async fn run_gol_gui(window: &mut PistonWindow, app: &mut Game, game_grid: &mut Vec<Vec<bool>>, alive_list: &mut Vec<(usize, usize)>) {
    let config = &*CONFIG;
    
    let event_settings = EventSettings {
        max_fps: config.max_fps,
        ups: config.ups,
        ups_reset: 100,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };

    let mut events = Events::new(event_settings);
    let mut i = 0;
    let title = format!("Game of Life | fps: {}, ups: {}", config.max_fps, config.ups);

    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &game_grid);
        }

        if !app.is_paused {
            // Updaten
            if let Some(_args) = e.update_args() {
                window.events.set_ups(1000);
                i += 1;
                (*game_grid, *alive_list) = backend::next_gen(&game_grid, &alive_list, &config.grid_size).await;

                // Status als window title
                window.set_title(format!("{} | generation: {}, alive: {}, dead: {}", title, i, alive_list.len(), ((config.grid_size.width * config.grid_size.height)-alive_list.len())));
            }
        }

        // Spiel pausieren
        if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
            app.is_paused = !app.is_paused;

            // Status als window title
            window.set_title(format!("{} | generation: {}, alive: {}, dead: {} [PAUSED]", title, i, alive_list.len(), ((config.grid_size.width * config.grid_size.height)-alive_list.len())));
        }
    }
}

fn run_gol_console(game_grid: &Vec<Vec<bool>>, i: i32) {
    print_game_grid(&game_grid);
    print!("Generation: ");
    print!("{}", i);
    println!();
}

#[tokio::main]
async fn main() {
    // Gamegrid init
    let mut game_grid = init_field();
    let mut alive_list: Vec<(usize, usize)> = vec![(0,0); 0];

    let config = &*CONFIG;

    let mut i = 1;
    let start = Instant::now();

    match config.interface {
        InterfaceType::Gui => {
            // Create a window.
            let config = &*CONFIG;
            let (mut window, mut app) = init_gui(config);
            
            // GUI zeichnen und updaten
            run_gol_gui(&mut window, &mut app, &mut game_grid, &mut alive_list).await;
        }
        InterfaceType::Console => {
            loop {
                (game_grid, alive_list) = backend::next_gen(&game_grid, &alive_list, &config.grid_size).await;
                run_gol_console(&game_grid, i);
                if i==100000 {break;}
                i += 1;
                sleep(time::Duration::from_millis(300));
            }
        }
        InterfaceType::Speed => {
            loop {
                (game_grid, alive_list) = backend::next_gen(&game_grid, &alive_list, &config.grid_size).await;
                if i==100000 {break;}
                i += 1;
                print!("Generation: ");
                print!("{}", i);
                println!();
            }
        }
        InterfaceType::SuperSpeed => {
            loop {
                (game_grid, alive_list) = backend::next_gen(&game_grid, &alive_list, &config.grid_size).await;
                if i==100000 {break;}
                i += 1;
            }
            //run_gol_console(&game_grid, i);
        }
    }

    let duration = start.elapsed();
    println!("Time for 100.000 Generations: {:?}", duration);
}