use std::{
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub enum Direction {
    North,
    South,
    Up,
    Down,
    East,
    West,
}

pub struct Player {
    x: i32,
    y: i32,
    z: i32,
    direction: Direction,
    hp: i32,
    name: String,
}

impl Player {
    pub fn new(name: String, x: i32, y: i32, z: i32, health: i32, direction: Direction) -> Self {
        next_id();
        Self {
            name,
            x,
            y,
            z,
            hp: health.max(0),
            direction,
        }
    }

    pub fn def(name: String, x: i32, y: i32, z: i32) -> Self {
        Self::new(name, x, y, z, 20, Direction::North)
    }

    pub fn get_num_players() {
        unimplemented!();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_z(&self) -> i32 {
        self.z
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn set_hp(&mut self, value: i32) {
        self.hp = value;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn mov(&mut self, direction: Direction, units: i32) {
        match direction {
            Direction::North => self.x += units,
            Direction::South => self.x -= units,
            Direction::Up => self.y += units,
            Direction::Down => self.y += units,
            Direction::East => self.z += units,
            Direction::West => self.z -= units,
        }
    }

    pub fn teleport(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn teleport_to(&mut self, player: &Player) {
        self.teleport(player.x, player.y, player.z)
    }

    pub fn attack(&mut self, player: &mut Player, damage: i32) {
        player.set_hp(player.hp - damage);
        self.hp += damage / 2;
    }

    pub fn get_distance(&self, x: i32, y: i32, z: i32) -> f64 {
        (((x - self.x).pow(2) + (y - self.y).pow(2) + (z - self.z).pow(2)) as f64).sqrt()
    }

    pub fn get_distance_to(&self, player: &Player) -> f64 {
        self.get_distance(player.x, player.y, player.z)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "P".to_string() + &next_id().to_string(),
            x: 0,
            y: 0,
            z: 0,
            direction: Direction::North,
            hp: 20,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nHealth: {}\nCoordinates: X {} Y {} Z {}\nDirection: {}",
            self.name, self.hp, self.x, self.y, self.z, self.direction
        )
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &Self::North => "North",
                &Self::South => "South",
                &Self::Up => "Up",
                &Self::Down => "Down",
                &Self::East => "East",
                &Self::West => "West",
            }
        )
    }
}
