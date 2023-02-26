use rand::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;


pub enum Direction{
    Up = 0,
    Down = 2,
    Left = 3,
    Right = 1
}

#[derive(Clone)]
pub struct Color{
    r: u8,
    b: u8,
    g: u8,
    a: u8
}

const water_blue: Color = Color{r:0, b:200, g: 50, a: 70};
const dir_modifier_step: u8 = 9;
const dir_modifier_start: u8 = 91;

pub struct Cell{
    color: Color,
    fisk: Option<Rc<RefCell<Fishy>>>
}

#[derive(Clone)]
pub struct Pos{
    x: i32,
    y: i32
}

pub struct Fishy {
    position: Pos,
    // name: String,
    swim_speed: u8,
    direction: Direction,
    dir_count: u8,
    color: Color,
    // health: u8
}

pub struct FishTank {
    size: Pos,
    fishys: Vec<Rc<RefCell<Fishy>>>,
    grid: Vec<Vec<Cell>>
    // OxygenMap intended a feature to be added
}


impl Color {
    pub fn new_rand() -> Color{
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen_range(0..=255);
        let green: u8 = rng.gen_range(0..=255);
        let blue: u8 = rng.gen_range(0..=255);
        let alpha: u8 = rng.gen_range(0..=255);
        return Color { r: red, b: blue, g: green, a: alpha } 
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

impl Fishy {
    pub fn new(size: Pos) -> Fishy{
        let color = Color::new_rand();
        let pos = Pos::new_rand(size);
        let dir = Direction::new_rand();
        let dir_count = 0;
        let swim_speed = 1;
        return Fishy{color: color, position: pos, swim_speed: swim_speed, direction: dir, dir_count: dir_count}
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
        let mut hit_wall = false;
        if (x < 0) || (y < 0) || (x > size.x - 1) || (y > size.y - 1){
            self.direction = self.direction.invert();
            hit_wall = true;
            (x,y) = self.swim_dir();
        }
        let mut cell = &grid[y as usize][x as usize];
        if cell.fisk.is_some(){
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
        return FishTank { size: size, fishys: fishys, grid: grid }
    }

    pub fn tick(& mut self){
        let size = self.size.clone();
        for fishy in &self.fishys{
            let (x,y) = fishy.borrow_mut().swim(&size, & mut self.grid);
            // self.grid[y as usize][x as usize].color = fishy.borrow().color.clone();
            self.grid[y as usize][x as usize].fisk = Some(Rc::clone(fishy));
        }
    }

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
        write!(f, "({},{},{},{})", self.r, self.g, self.b, self.a)
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

fn main(){
    let mut fish_tank = FishTank::new(2, Pos{x:3,y:3});
    println!("{}", fish_tank);
    for _i in 0..10{
        fish_tank.tick();
        println!("{}", fish_tank);
    }
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

}