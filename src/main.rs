use rand::prelude::*;
use std::fmt;
use std::rc::Rc;


pub enum Direction{
    Up = 0,
    Down = 2,
    Left = 3,
    Right = 1
}

pub struct Color{
    r: u8,
    b: u8,
    g: u8,
    a: u8
}

const water_blue: Color = Color{r:0, b:200, g: 50, a: 70};

pub struct Cell{
    color: Color,
    fisk: Option<Rc<Fishy>>
}

#[derive(Clone)]
pub struct Pos{
    x: u8,
    y: u8
}


pub struct Fishy {
    position: Pos,
    // name: String,
    direction: Direction,
    color: Color,
    // health: u8
}

pub struct FishTank {
    size: Pos,
    fishys: Vec<Rc<Fishy>>,
    grid: Vec<Vec<Cell>>
    // OxygenMap intended a feature to be added
}

impl Color {
    pub fn new_rand() -> Color{
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen_range(0..255);
        let green: u8 = rng.gen_range(0..255);
        let blue: u8 = rng.gen_range(0..255);
        let alpha: u8 = rng.gen_range(0..255);
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
        match rng.gen_range(0..3) {
            0 => dir = Direction::Up,
            1 => dir = Direction::Right,
            2 => dir = Direction::Down,
            _ => dir = Direction::Left,
        }
        return dir
    }
}

impl Cell{
    pub fn new(color: Color) -> Cell{
        return Cell{color, fisk: None}
    }
}

impl Fishy {
    pub fn new(size: Pos) -> Fishy{
        let color = Color::new_rand();
        let pos = Pos::new_rand(size);
        let dir = Direction::new_rand();
        return Fishy{color: color, position: pos, direction: dir}
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
            let fish = Rc::new(Fishy::new(size_c));
            let x = fish.position.x;
            let y = fish.position.y;
            let fish_c = Rc::clone(&fish);
            fishys.push(fish);
            grid[y as usize][x as usize].fisk = Some(Rc::clone(&fish_c));
        }
        return FishTank { size: size, fishys: fishys, grid: grid }
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
            writeln!(f, "{}", i);
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
    let fish_tank = FishTank::new(1, Pos{x:3,y:3});
    println!("{}", fish_tank);
}