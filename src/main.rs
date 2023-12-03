use serde::Deserialize;
use core::time;
use std::{fs, thread::sleep, time::Instant};
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

fn next_gen(grid: &Vec<Vec<bool>>, alive_list: &Vec<(usize, usize)>) -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    // Config
    let config = &*CONFIG;

    let mut next: Vec<Vec<bool>> = vec![vec![false; config.grid_size.width]; config.grid_size.height];
    let mut alive_list_new: Vec<(usize, usize)> = vec![(0,0); 0];
    let mut y: usize = 0;
    let mut x: usize;
    let mut alive_count: u8;

    if alive_list.len()==0 {
        for row in grid {
            x = 0;
            y+=1;
            for cell in row {
                x+=1;
                alive_count = 0;
                if check(x-1, y-1, &grid) {alive_count+=1;}
                if check(x-1, y, &grid) {alive_count+=1;}
                if check(x-1, y+1, &grid) {alive_count+=1;}
                if check(x, y-1, &grid) {alive_count+=1;}
                if check(x, y+1, &grid) {alive_count+=1;}
                if check(x+1, y-1, &grid) {alive_count+=1;}
                if check(x+1, y, &grid) {alive_count+=1;}
                if check(x+1, y+1, &grid) {alive_count+=1;}
    
                if !*cell && alive_count==3 {next[y-1][x-1]=true; alive_list_new.push((y-1,x-1));}
                if *cell && alive_count>1 && alive_count<4 {next[y-1][x-1]=true; alive_list_new.push((y-1,x-1));}
    
                
                /*print!("{}",x);
                print!(" ");
                print!("{}",y);
                print!(" ");
                print!("{}",alive);
                println!();*/
            }
        }
    } else {
        for cell in alive_list {
            y = cell.0 +1;
            x = cell.1 +1;

            let r = check_area(x, y, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y-1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y+1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x, y-1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x, y+1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y-1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y+1, &grid);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            alive_list_new.sort();
            alive_list_new.dedup();
        }
    }
    

  return (next, alive_list_new);
}

fn check_area (mut x: usize, mut y: usize, grid: &Vec<Vec<bool>>) -> (bool, usize, usize) {

    let config = &*CONFIG;

    if y<1 {y=config.grid_size.height;}
    if y>config.grid_size.height {y=1;}
    if x<1 {x=config.grid_size.width;}
    if x>config.grid_size.width {x=1;}

    let cell = grid[y-1][x-1];
    let mut alive_count = 0;

    if check(x-1, y-1, &grid) {alive_count+=1;}
    if check(x-1, y, &grid) {alive_count+=1;}
    if check(x-1, y+1, &grid) {alive_count+=1;}
    if check(x, y-1, &grid) {alive_count+=1;}
    if check(x, y+1, &grid) {alive_count+=1;}
    if check(x+1, y-1, &grid) {alive_count+=1;}
    if check(x+1, y, &grid) {alive_count+=1;}
    if check(x+1, y+1, &grid) {alive_count+=1;}

    if !cell && alive_count==3 {return (true, y, x);}
    if cell && alive_count>1 && alive_count<4 {return (true, y, x);}       
    return (false, 0, 0);
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

fn main() {
    let mut game_grid = init_field();
    let mut alive_list: Vec<(usize, usize)> = vec![(0,0); 0];
    let mut i = 1;

    let start = Instant::now();
    loop {
        print_game_grid(&game_grid);
        print!("Generation: ");
        print!("{}",i);
        println!();
        if i==100000 {break;}

        //for cell in &alive_list {println!("{}, {}",cell.0,cell.1);}

        let result = next_gen(&game_grid, &alive_list);
        game_grid = result.0;
        alive_list = result.1;
        i+=1;
        sleep(time::Duration::from_millis(300));
    }
    let duration = start.elapsed();
    println!("Time for 100.000 Generations: {:?}", duration);
}
