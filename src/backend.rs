use futures::join;

use crate::GridSize;

pub async fn next_gen(grid: &Vec<Vec<bool>>, alive_list: &Vec<(usize, usize)>, grid_size: &GridSize)   -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    
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
            }
        }
    } else {
        for cell in alive_list {
            y = cell.0 +1;
            x = cell.1 +1;

            let r1 = check_area(x, y, &grid, &grid_size);
            let r2 = check_area(x-1, y-1, &grid, &grid_size);
            let r3 = check_area(x-1, y, &grid, &grid_size);
            let r4 = check_area(x-1, y+1, &grid, &grid_size);
            let r5 = check_area(x, y-1, &grid, &grid_size);      
            let r6 = check_area(x, y+1, &grid, &grid_size);
            let r7 = check_area(x+1, y-1, &grid, &grid_size);
            let r8 = check_area(x+1, y, &grid, &grid_size);
            let r9 = check_area(x+1, y+1, &grid, &grid_size);

            let (r1,r2,r3,r4,r5,r6,r7,r8,r9) 
                = join!(r1,r2,r3,r4,r5,r6,r7,r8,r9);

            if r1.0 {next[r1.1-1][r1.2-1]=true; alive_list_new.push((r1.1-1, r1.2-1));}
            if r2.0 {next[r2.1-1][r2.2-1]=true; alive_list_new.push((r2.1-1, r2.2-1));}
            if r3.0 {next[r3.1-1][r3.2-1]=true; alive_list_new.push((r3.1-1, r3.2-1));}
            if r3.0 {next[r3.1-1][r3.2-1]=true; alive_list_new.push((r3.1-1, r3.2-1));}
            if r4.0 {next[r4.1-1][r4.2-1]=true; alive_list_new.push((r4.1-1, r4.2-1));}
            if r5.0 {next[r5.1-1][r5.2-1]=true; alive_list_new.push((r5.1-1, r5.2-1));}
            if r6.0 {next[r6.1-1][r6.2-1]=true; alive_list_new.push((r6.1-1, r6.2-1));}
            if r7.0 {next[r7.1-1][r7.2-1]=true; alive_list_new.push((r7.1-1, r7.2-1));}
            if r8.0 {next[r8.1-1][r8.2-1]=true; alive_list_new.push((r8.1-1, r8.2-1));}
            if r9.0 {next[r9.1-1][r9.2-1]=true; alive_list_new.push((r9.1-1, r9.2-1));}
        }
        
        alive_list_new.sort();
        alive_list_new.dedup();
    }
    

  return (next, alive_list_new);
}

async fn check_area (mut x: usize, mut y: usize, grid: &Vec<Vec<bool>>, grid_size: &GridSize) -> (bool, usize, usize) {

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