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
    for row in game_grid {
        for &cell in row {
            if cell {
                print!("X ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}


fn main() {
    let game_grid = init_field();

    print_game_grid(&game_grid);
}
