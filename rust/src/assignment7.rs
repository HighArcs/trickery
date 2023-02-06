pub struct Game;

impl Game {
    pub fn play(g: GameWheel) {}
}

pub struct GameWheel {
    slices: Vec<Slice>;
    current_pos: usize;
}

impl std::fmt::Display for GameWheel {}