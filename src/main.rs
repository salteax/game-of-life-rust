use serde::Deserialize;
use std::fs;

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

fn init_field() -> Vec<Vec<bool> > {
    // Config-Datei einlesen
    let toml_str = fs::read_to_string("config.toml").expect("Failed to read config.toml file");
    let config: Config = toml::from_str(&toml_str).expect("Failed to deserialize config.toml");
    
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


fn main() {
    let game_grid = init_field();

    print_game_grid(&game_grid);
}
