use std::{
    io::{Read, Stdin, Write},
    str::FromStr,
};

pub struct Scanner(Stdin);

impl Scanner {
    pub fn new() -> Self {
        Self(std::io::stdin())
    }

    pub fn with(i: Stdin) -> Self {
        Self(i)
    }

    pub fn next_line(&self) -> String {
        let mut str = String::new();
        let _ = std::io::stdout().flush();
        self.0.read_line(&mut str).expect("did not enter a string");
        if let Some('\n') = str.chars().next_back() {
            str.pop();
        }

        if let Some('\r') = str.chars().next_back() {
            str.pop();
        }

        str
    }

    pub fn next<T: FromStr>(&self) -> T {
        let mut str = String::new();

        while str.parse::<T>().is_err() {
            str = self.next_line();
        }

        str.parse::<T>().ok().unwrap()
    }

    pub fn next_int<T: Int + FromStr>(&self) -> T {
        self.next::<T>()
    }

    pub fn next_u8(&self) -> u8 {
        self.next_int()
    }

    pub fn next_u16(&self) -> u16 {
        self.next_int()
    }

    pub fn next_u32(&self) -> u32 {
        self.next_int()
    }

    pub fn next_u64(&self) -> u64 {
        self.next_int()
    }

    pub fn next_u128(&self) -> u128 {
        self.next_int()
    }

    pub fn next_usize(&self) -> usize {
        self.next_int()
    }

    pub fn next_i8(&self) -> i8 {
        self.next_int()
    }

    pub fn next_i16(&self) -> i16 {
        self.next_int()
    }

    pub fn next_i32(&self) -> i32 {
        self.next_int()
    }

    pub fn next_i64(&self) -> i64 {
        self.next_int()
    }

    pub fn next_i128(&self) -> i128 {
        self.next_int()
    }

    pub fn next_isize(&self) -> isize {
        self.next_int()
    }

    pub fn next_bool(&self) -> bool {
        self.next::<bool>()
    }

    pub fn next_float(&self) -> f32 {
        self.next::<f32>()
    }

    pub fn next_double(&self) -> f64 {
        self.next::<f64>()
    }

    pub fn next_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        self.0.read(&mut buf).expect("any token");
        *buf.get(0).unwrap_or(&0) as char
    }
}

pub trait Int {}
pub trait Float {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for usize {}

impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
impl Int for isize {}

impl Float for f32 {}
impl Float for f64 {}
