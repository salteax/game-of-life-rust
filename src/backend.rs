use crate::GridSize;

pub fn next_gen(grid: &Vec<Vec<bool>>, alive_list: &Vec<(usize, usize)>, grid_size: &GridSize)   -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    
    let mut next: Vec<Vec<bool>> = vec![vec![false; grid_size.width]; grid_size.height];
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
                if check(x-1, y-1, &grid, &grid_size) {alive_count+=1;}
                if check(x-1, y, &grid, &grid_size) {alive_count+=1;}
                if check(x-1, y+1, &grid, &grid_size) {alive_count+=1;}
                if check(x, y-1, &grid, &grid_size) {alive_count+=1;}
                if check(x, y+1, &grid, &grid_size) {alive_count+=1;}
                if check(x+1, y-1, &grid, &grid_size) {alive_count+=1;}
                if check(x+1, y, &grid, &grid_size) {alive_count+=1;}
                if check(x+1, y+1, &grid, &grid_size) {alive_count+=1;}
    
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

            let r = check_area(x, y, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y-1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            let r = check_area(x-1, y+1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x, y-1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x, y+1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y-1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}
            
            let r = check_area(x+1, y+1, &grid, &grid_size);
            if r.0 {next[r.1-1][r.2-1]=true; alive_list_new.push((r.1-1, r.2-1));}

            alive_list_new.sort();
            alive_list_new.dedup();
        }
    }
    

  return (next, alive_list_new);
}

fn check_area (mut x: usize, mut y: usize, grid: &Vec<Vec<bool>>, grid_size: &GridSize) -> (bool, usize, usize) {

    if y<1 {y=grid_size.height;}
    if y>grid_size.height {y=1;}
    if x<1 {x=grid_size.width;}
    if x>grid_size.width {x=1;}

    let cell = grid[y-1][x-1];
    let mut alive_count = 0;

    if check(x-1, y-1, &grid, &grid_size) {alive_count+=1;}
    if check(x-1, y, &grid, &grid_size) {alive_count+=1;}
    if check(x-1, y+1, &grid, &grid_size) {alive_count+=1;}
    if check(x, y-1, &grid, &grid_size) {alive_count+=1;}
    if check(x, y+1, &grid, &grid_size) {alive_count+=1;}
    if check(x+1, y-1, &grid, &grid_size) {alive_count+=1;}
    if check(x+1, y, &grid, &grid_size) {alive_count+=1;}
    if check(x+1, y+1, &grid, &grid_size) {alive_count+=1;}

    if !cell && alive_count==3 {return (true, y, x);}
    if cell && alive_count>1 && alive_count<4 {return (true, y, x);}       
    return (false, 0, 0);
}

fn check(mut x: usize, mut y: usize, grid: &Vec<Vec<bool>>, grid_size: &GridSize) -> bool {

    if y<1 {y=grid_size.height;}
    if y>grid_size.height {y=1;}
    if x<1 {x=grid_size.width;}
    if x>grid_size.width {x=1;}

    return grid[y-1][x-1];
}