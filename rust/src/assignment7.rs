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

}#![feature(let_chains)]
pub type Branch<T> = Option<Box<BinarySearchTree<T>>>;

pub struct BinarySearchTree<T> {
    value: T,
    left: Branch<T>,
    right: Branch<T>,
}

impl<T> BinarySearchTree<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(&self) -> &Branch<T> {
        &self.left
    }

    pub fn right(&self) -> &Branch<T> {
        &self.right
    }
    
    pub fn left_mut(&mut self) -> &mut Branch<T> {
        &mut self.left
    }

    pub fn right_mut(&mut self) -> &mut Branch<T> {
        &mut self.right
    }

    pub fn head(&self) -> &T {
        &self.value
    }

    pub fn head_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set_left(&mut self, left: BinarySearchTree<T>) -> &mut Self {
        self.left = Some(Box::new(left));
        self
    }

    pub fn remove_left(&mut self) -> &mut Self {
        self.left = None;
        self
    }

    pub fn set_right(&mut self, right: BinarySearchTree<T>) -> &mut Self {
        self.right = Some(Box::new(right));
        self
    }

    pub fn remove_right(&mut self) -> &mut Self {
        self.right = None;
        self
    }

    pub fn has(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        if value == self.head() {
            return true;
        }

        if value < self.head() {
            if let Some(x) = self.left() {
                return x.has(value);
            }
        }

        if let Some(x) = self.right() {
            return x.has(value);
        } else {
            return false;
        }
    }

    pub fn insert(&mut self, value: T) -> &mut Self
    where
        T: PartialOrd,
    {
        if self.has(&value) {
            return self;
        }

        if &value < self.head() {
            if let Some(x) = self.left_mut() {
                x.insert(value);
            } else {
                self.set_left(BinarySearchTree::new(value));
            }
        } else if let Some(x) = self.right_mut() {
            x.insert(value);
        } else {
            self.set_right(BinarySearchTree::new(value));
        }
        
        self
    }
    
    pub fn find(&self, value: T) -> Option<&BinarySearchTree<T>> where T: PartialOrd, Self: Clone {
        if &value == self.head() {
            return Some(self)
        }
        
        let mut tree = Some(Box::new(self.clone()));
        
        while let Some(t) = &tree && &value < t.head() {
            tree = t.left().clone()
        }
        
        while let Some(t) = &tree && &value > t.head() {
            tree = t.right().clone()
        }
        
        None
    }
    
    pub fn find_mut(&mut self, value: T) -> Option<&mut BinarySearchTree<T>> where T: PartialOrd, Self: Clone {
        if &value == self.head() {
            return Some(self)
        }
        
        let mut tree = Some(Box::new(self.clone()));
        
        while let Some(t) = &tree && &value < t.head() {
            tree = t.left().clone()
        }
        
        while let Some(t) = &tree && &value > t.head() {
            tree = t.right().clone()
        }
        
        None
    }
}

impl<T> Clone for BinarySearchTree<T> where T: Clone {
    fn clone(&self) -> Self {
        let mut n = Self::new(self.head().clone());
        n
    }
}
