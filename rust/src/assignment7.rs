use std::fmt::{Display, Error, Formatter};
pub struct Game;

impl Game {
    pub fn play(g: GameWheel) {}
}

pub struct Slice {
    color: String,
    prize_amount: u32,
}

impl Slice {
    pub fn new(c: String, p: i32) -> Self {
        Self { color: c, prize_amount: p }
    }

    pub fn get_prize_amount(&self) -> &i32 {
        self.prize_amount
    }

    pub fn get_color(&self) -> &String {
        color
    }
}

impl Display for Slice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!("Color: {}, Prize Amount: ${}", self.color, self.prize_amount)
    }
}

pub struct GameWheel {
    slices: Vec<Slice>;
    current_pos: usize;
}

impl Display for GameWheel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.slices.len() {
            write!("{i} - ");
            write!("{}\n", self.slices.get(i).unwrap());
        }
    }
}

impl GameWheel {
    pub fn scramble(&mut self) {
        let mut black = Vec::new();
        let mut blue = Vec::new();
        let mut red = Vec::new();
        
        for slice in self.slices {
            (match slice.get_color() {
                &"black".to_owned() => black,
                &"blue".to_owned() => blue,
                &"red".to_owned() => red,
                _ => unreachable!(),
            }).push(slice);
        }

        let mut out = Vec::new();

        for i in 0..self.slices.len() {
            if i % 5 == 0 {
                out.push(black.remove(Self::random_idx(black)));
            } else if i % 2 == 0 {
                out.push(blue.remove(Self::random_idx(black)));
            } else {
                out.push(red.remove(Self::random_idx(black)));
            } 
        }

        self.slices = out;
    }

}
