pub struct Pokemon<'a> {
    health: i32,
    strength: i32,
    speed: i32,
    direction: Direction,
    x: i32,
    y: i32,
    z: i32,
    name: &str
}

pub enum Direction {
    North = 1,
    South = 2,
    Left = 3,
    Right = 4,
}

impl Pokemon {
    pub fn new(name: &str, health: i32, strength: i32, speed: i32) -> Self {
        Self { 
            name,
            health,
            strength,
            speed,
            x = 0,
            y = 0,
            z = 0,
            direction = Direction::North
        }
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
        self.health
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_hp(&mut hp) -> &mut Self {
        if hp < 0 {
            self.health = 0;
        } else {
            self.health = hp;
        }

        self
    }

    pub fn mov(&mut self, direction: Direction, units: i32) -> &mut Self {
        match direction {
            Direction::North => self.x += units,
            Direction::South => self.x -= units,
            Direction::Left => self.y += units,
            Direction::Right => self.y -= units,
        }

        self
    }

    pub fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn chance(min: i32, max: i32) -> i32 {
        (rand::random<f64>() * (max - min + 1) + min) as i32
    }

    pub fn pokemon_name() -> &str {
        let v = vec!["Pikachu", "Charizard", "Bulbasaur", "Sylveon", "[enemy pokemon]"];

        v[Self::chance(0, v.len())]
    }

    pub fn calculate_battle_chance(&mut self) -> &mut Self {
        let chance = Self::chance(50, 100);

        if chance > 50 {
            let enemy = Self::new(Self::pokemon_name(), Self::chance(0, 100), Self::chance(0, 100), Self::chance(0, 100));

            println!("You have ran into a {}!", enemy.name);

            println!("Type 1 to fight or 2 to run away:");

            // nope i give up
        }
    }
}