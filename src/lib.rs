use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::{fmt, vec};
use std::rc::Rc;

#[wasm_bindgen]
pub enum Direction{
    Up = 0,
    Down = 2,
    Left = 3,
    Right = 1
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Color{
    r: u8,
    b: u8,
    g: u8,
    a: u8
}

const fish_tank_size: Pos = Pos{x:255,y:255};
const canvas_size: usize = (fish_tank_size.x * fish_tank_size.y * 4) as usize;
const water_blue: Color = Color{r:0, g:100, b:255, a: 200};
const dir_modifier_step: u8 = 9;
const dir_modifier_start: u8 = 91;

pub fn check_wall(pos: (i32,i32), size: &Pos) -> bool{
    return (pos.0 < 0) || (pos.1 < 0) || (pos.0 > size.x - 1) || (pos.1 > size.y - 1)
}

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn my_init_function() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn tick(fish_tank: &mut FishTank){
    fish_tank.tick();
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cell{
    color: Color,
    fisk: Option<Rc<RefCell<Fishy>>>
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Pos{
    x: i32,
    y: i32
}

pub struct Body{
    colors: Vec<Color>,
    matrix: Vec<Vec<Option<Color>>>
}

#[wasm_bindgen]
pub struct Fishy {
    position: Pos,
    // name: String,
    body: Body,
    swim_speed: u8,
    direction: Direction,
    dir_count: u8,
    color: Color,
    // health: u8
}

#[wasm_bindgen]
pub struct FishTank {
    size: Pos,
    fishys: Vec<Rc<RefCell<Fishy>>>,
    grid: Vec<Vec<Cell>>,
    canvas: Vec<u8>
    // OxygenMap intended a feature to be added
}


impl Color {
    pub fn new_rand() -> Color{
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen_range(0..=255);
        let green: u8 = rng.gen_range(0..=255);
        let blue: u8 = rng.gen_range(0..=255);
        let alpha: u8 = rng.gen_range(0..=100);
        return Color { r: red, b: blue, g: green, a: alpha } 
    }

    pub fn get_color_values(&self) -> (u8,u8,u8,u8){
        return (self.r, self.g, self.b, self.a)
    }

    pub fn gen_colors() -> Vec<Color>{
        let mut colors = Vec::new();
        let mut rng = rand::thread_rng();
        let color_count = rng.gen_range(1..=5);
        for i in 0..color_count{
            colors.push(Color::new_rand());
        }
        return colors
    }
}

impl Pos{
    pub fn new_rand(size: Pos) -> Pos{
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..size.x);
        let y = rng.gen_range(0..size.y);
        return Pos{x,y}
    }
}

#[wasm_bindgen]
impl Pos{
    pub fn new(x:i32, y:i32) -> Pos{
        return Pos{x,y}
    }
}

impl Direction{
    pub fn new_rand() -> Direction {
        let mut rng = rand::thread_rng();
        let dir:Direction;
        match rng.gen_range(0..=3) {
            0 => dir = Direction::Up,
            1 => dir = Direction::Right,
            2 => dir = Direction::Down,
            _ => dir = Direction::Left,
        }
        return dir
    }

    pub fn invert(&self) -> Direction {
        match self{
            Direction::Up => return Direction::Down,
            Direction::Down => return Direction::Up,
            Direction::Left => return Direction::Right,
            Direction::Right => return Direction::Left,
        }
    }

}

impl Cell{
    pub fn new(color: Color) -> Cell{
        return Cell{color, fisk: None}
    }

    pub fn reset(& mut self){
        self.fisk = None;
        self.color = water_blue;
    }
}

impl Body{
    fn gen_matrix(fish_height: u8) -> Vec<Vec<Option<Color>>>{
        let place_holder_color: Color = Color{r:200, g:0, b:0, a:255};
        // let place_holder_cell: Cell = Cell::new(place_holder_color);
        //gen center line length
        let mut matrix: Vec<Vec<Option<Color>>> = Vec::new();
        let mut rng = rand::thread_rng();
        let length_modifier = rng.gen_range(0..=100);
        let length = (fish_height as f64 * ((length_modifier / 100 + 2)) as f64 ).ceil() as u8;
        //determine how long each of the scale lines will be

        //gens dimensions
        let above_count = (((fish_height - 1) / 2) as f64).ceil();
        let below_count: f64 = fish_height as f64 - 1.0 - above_count;
        let center_line_length = length;
        let above_center_length = center_line_length - 1;
        println!("Above: {}, Below: {}", above_count, below_count);
        //pushes the center line and the intial above and below at 1 less length then center
        matrix.push(vec![Some(place_holder_color.clone());center_line_length.into()]);
        matrix.insert(0, vec![Some(place_holder_color.clone());(center_line_length - 1).into()]);
        matrix.push(vec![Some(place_holder_color.clone());above_center_length.into()]);
        //fills in the rest of the scales
        for line in 1..(above_count) as u8{
            let mut temp_matrix = vec![Some(place_holder_color.clone());(above_center_length - (2 * line)) as usize];
            for i in 1..line + 1{
                temp_matrix.insert(0, None);
                println!("test");
            }
            // temp_matrix.insert(0, )
            matrix.insert(0, temp_matrix);
        }
        for line in 1..(below_count) as u8{
            let mut temp_matrix = vec![Some(place_holder_color.clone());(above_center_length - (2 * line)) as usize];
            for i in 1..line + 1{
                temp_matrix.insert(0, None);
                println!("test");
            }
            matrix.push(temp_matrix);
        }

        //gen the fins based off of scales



        return matrix
        //gen the fins based off of that
        //place eye
        //place rear fin
        //insert blanks
    }

    pub fn new(fish_height: u8) -> Body{
        let colors = Color::gen_colors();
        let matrix = Body::gen_matrix(fish_height);
        Body{colors: colors, matrix: matrix}
    }

    pub fn get_middle(&self) -> Pos{
        let x = ((self.matrix.len() / 2) as f64).ceil() - 1.0;
        let y = ((self.matrix[0].len() / 2) as f64).ceil() - 1.0;
        return Pos{x: x as i32,y: y as i32}
    }

    pub fn transpose(&self, grid: & mut Vec<Vec<Cell>>, x: i32, y: i32, fishy: &Rc<RefCell<Fishy>>){
        let mid = self.get_middle();
        for (x_index, row) in self.matrix.iter().enumerate(){
            for (y_index, color) in row.iter().enumerate(){
                let temp_x = x + x_index as i32 - mid.x;
                let temp_y = y + y_index  as i32 - mid.y;
                let mut temp_cell = & mut grid[temp_y as usize][temp_x as usize];
                temp_cell.color = color.clone().unwrap_or(water_blue.clone());
                temp_cell.fisk = Some(Rc::clone(fishy));
            }
        }
    }
}

impl Fishy {
    pub fn new(size: Pos) -> Fishy{
        let color = Color::new_rand();
        let pos = Pos::new_rand(size);
        let dir = Direction::new_rand();
        let dir_count = 0;
        let swim_speed = 1;
        let body = Body::new(6);
        return Fishy{color: color, position: pos, swim_speed: swim_speed, direction: dir, dir_count: dir_count, body: body}
    }


    pub fn new_dir(& mut self){
        let cur_dir = &self.direction;
        let dir_count = self.dir_count;
        let rng: u8 = rand::thread_rng().gen_range(0..=100);
        let mut dirs = vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left];
        match cur_dir {
            Direction::Up => dirs.remove(0),
            Direction::Right => dirs.remove(1),
            Direction::Down => dirs.remove(2),
            Direction::Left => dirs.remove(3),
        };
        let dir_modifier = dir_count * dir_modifier_step;
        let mut same = dir_modifier_start - dir_modifier;
        if 100 - same <= 0{
            same = 10;
        }
        let rest = (100 - same) / 3;
        let rest1 = same + rest;
        let rest2 = rest1 + rest;
        let rest3 = rest2 + rest;
        if (0..same).contains(&rng){
        }
        else if (same..rest1).contains(&rng){
            self.direction = dirs.remove(0);
        }
        else if (rest1..rest2).contains(&rng){
            self.direction = dirs.remove(1);
        }
        else if (rest2..rest3).contains(&rng){
            self.direction = dirs.remove(2);
        }

    }

    fn swim_dir(&self) -> (i32,i32) {
        let mut x: i32 = self.position.x;
        let mut y: i32 = self.position.y;
        match self.direction{
            Direction::Up => y = self.position.y as i32 - self.swim_speed as i32,
            Direction::Down => y = self.position.y as i32 + self.swim_speed as i32,
            Direction::Left => x = self.position.x as i32 - self.swim_speed as i32,
            Direction::Right => x = self.position.x as i32 + self.swim_speed as i32,
        }
        return (x,y)
    }

    pub fn swim(& mut self, size: &Pos, grid: & mut Vec<Vec<Cell>>) -> (i32, i32){
        grid[self.position.y as usize][self.position.x as usize].reset();
        self.new_dir();
        let (mut x,mut y) = self.swim_dir();
        let mut hit_wall = check_wall((x,y), size);
        if hit_wall{
            println!("Hit Wall");
            self.direction = self.direction.invert();
            hit_wall = true;
            (x,y) = self.swim_dir();
        }
        let mut cell = &grid[y as usize][x as usize];
        if cell.fisk.is_some(){
            println!("Bonk");
            if hit_wall{
                println!("Stickin it out");
                (x,y) = (self.position.x, self.position.y)
            }
            else{
                self.direction = self.direction.invert();
                (x,y) = self.swim_dir();
                if check_wall((x,y), size){
                    (x,y) = (self.position.x, self.position.y);
                }
            }
        }
        self.position.x = x;
        self.position.y = y;
        // let mut cell = & mut grid[self.position.y as usize][self.position.x as usize];
        // cell.color = self.color.clone();

        return (x,y)
    }

    pub fn swim_test(& mut self, size: &Pos, grid: & mut Vec<Vec<Cell>>, dir: Direction) -> (i32, i32){
        grid[self.position.y as usize][self.position.x as usize].reset();
        self.direction = dir;
        let (mut x,mut y) = self.swim_dir();
        let mut hit_wall = false;
        if (x < 0) || (y < 0) || (x > size.x - 1) || (y > size.y - 1){
            println!("Hit Wall");
            self.direction = self.direction.invert();
            hit_wall = true;
            (x,y) = self.swim_dir();
        }
        let mut cell = &grid[y as usize][x as usize];
        if cell.fisk.is_some(){
            println!("True");
        }
        else if cell.fisk.is_none(){
            println!("FALSE");
        }

        if cell.fisk.is_some(){
            println!("There is kinda a fish in the way!");
            if hit_wall{
                (x,y) = (self.position.x, self.position.y)
            }
            else{
                self.direction = self.direction.invert();
                (x,y) = self.swim_dir();
            }
        }
        self.position.x = x;
        self.position.y = y;
        let mut cell = & mut grid[self.position.y as usize][self.position.x as usize];
        cell.color = self.color.clone();
        return (x,y)
    }
}

impl FishTank{
    pub fn fish_collider(& mut self){
        let size = self.size.clone();
        let mut counter = 0;
        let mut dir: Direction;
        for fishy in &self.fishys{
            println!("{}, {}", self, fishy.borrow().color);
            if counter % 2 == 0{
                dir = Direction::Left;
            }
            else {
                dir = Direction::Right;
            }
            counter += 1;
            let (x,y) = fishy.borrow_mut().swim_test(&size, & mut self.grid, dir);
            // self.grid[y as usize][x as usize].color = fishy.borrow().color.clone();
            self.grid[y as usize][x as usize].fisk = Some(Rc::clone(fishy));
        }
    }
}

#[wasm_bindgen]
impl Cell{
    pub fn color(&self) -> String{
        return self.color.to_string();
    }

    pub fn insert_color(&self){

    }
}

#[wasm_bindgen]
impl FishTank{
    // #[wasm_bindgen(constructor)]
    pub fn new(fish_count: u8, size: Pos) -> FishTank{
        let mut fishys = Vec::new();
        let mut grid = Vec::new();
        for i in 0..size.y{
            let mut temp_vec = Vec::new();
            for ii in 0..size.x{
                temp_vec.push(Cell::new(water_blue))
            }
            grid.push(temp_vec);
        }
        for i in 0..fish_count{
            let size_c = size.clone();
            let fish_temp = Fishy::new(size_c);
            let x = fish_temp.position.x;
            let y = fish_temp.position.y;
            let fish_color = fish_temp.color.clone();
            let fish = Rc::new(RefCell::new(fish_temp));
            let fish_c = Rc::clone(&fish);
            fishys.push(fish);
            grid[y as usize][x as usize].fisk = Some(fish_c);
            grid[y as usize][x as usize].color = fish_color;
        }
        return FishTank { size: size, fishys: fishys, grid: grid, canvas: vec![0;canvas_size]}
    }

    pub fn push_canvas(&mut self){
        let mut red: u8;
        let mut blue: u8;
        let mut green: u8;
        let mut alpha: u8;
        for (x, row) in self.grid.iter().enumerate(){
            for (y, cell) in row.iter().enumerate(){
                (red, green, blue, alpha) = cell.color.get_color_values();
                let index: usize = ((y as i32 * self.size.x + x as i32) * 4) as usize;
                    self.canvas[index] = red;
                    self.canvas[index + 1] = green;
                    self.canvas[index + 2] = blue;
                    self.canvas[index + 3] = alpha;
            }
        }
    }

    pub fn get_canvas(&self) -> *const u8{
        return self.canvas.as_ptr()
    }

    pub fn tick(& mut self){
        let size = self.size.clone();
        for fishy in &self.fishys{
            let (x,y) = fishy.borrow_mut().swim(&size, & mut self.grid);
            fishy.borrow().body.transpose(& mut self.grid, x, y, fishy);
            // self.grid[y as usize][x as usize].fisk = Some(Rc::clone(fishy));

        }
        self.push_canvas();
    }

    pub fn print(&self) -> String{
        return self.to_string();
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string;
        match self{
            Direction::Up => string = "Up",
            Direction::Right => string = "Right",
            Direction::Down => string = "Down",
            Direction::Left => string = "Left"
        };
        write!(f, "{}", string)
    }
}

impl fmt::Display for Body{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.matrix{
            for col in i{
                write!(f, "{}", col.as_ref().unwrap_or(&Color{r:0,b:0,g:0,a:0}));
            }
            write!(f, "\n");
        }
        write!(f, "{}", "")
    }
}

impl fmt::Display for Pos{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Display for Cell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.color, self.fisk.is_some())
    }
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alpha: f64 = self.a as f64 / 100.0;
        write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, alpha)
    }
}

impl fmt::Display for Fishy{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: {}, Pos: {}, Dir: {}", self.color, self.position, self.direction)
    }
}

impl fmt::Display for FishTank{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.fishys{
            let fishy = i.borrow();
            writeln!(f, "{}", fishy);
        }
        for i in &self.grid{
            for ii in i{
                write!(f, "|{}|", ii);
            }
            write!(f,"\n");
        }
        writeln!(f, "----------")
    }
}

#[wasm_bindgen]
pub fn test() -> i32{
    return 2
}

// fn main(){
//     let col = Color::new_rand();
//     println!("{}", col);
    // let mut fish_tank = FishTank::new(2, Pos{x:3,y:3});
    // println!("{}", fish_tank);
    // for _i in 0..100{
    //     fish_tank.tick();
    //     println!("{}", fish_tank);
    // }
    //Fish Collision Tests In A Controlled Environment
    // let nebula = Rc::new(RefCell::new(Fishy{position: Pos{x:0, y:0}, swim_speed: 1, direction: Direction::Right, dir_count: 0, color: Color::new_rand()}));
    // let red_aqua = Rc::new(RefCell::new(Fishy{position: Pos{x:2, y:0}, swim_speed: 1, direction: Direction::Left, dir_count: 0, color: Color::new_rand()}));
    // let fishys = vec![Rc::clone(&nebula),Rc::clone(&red_aqua)];
    // let grid:Vec<Vec<Cell>> = vec![vec![Cell{color: water_blue, fisk: Some(Rc::clone(&nebula))}, Cell{color: water_blue, fisk: None}, Cell{color: water_blue, fisk: Some(Rc::clone(&red_aqua))}], vec![Cell{color: water_blue, fisk: None},Cell{color: water_blue, fisk: None},Cell{color: water_blue, fisk: None}]];
    // let mut fish_tank = FishTank { size: Pos{x:3,y:2}, fishys: fishys, grid: grid};
    // println!("{}", fish_tank);
    // for i in 0..10{
    //     fish_tank.fish_collider();
    //     println!("{}", fish_tank)
    // }

// }