pub type Result = std::result::Result<(), ()>;

/*
pub struct Table<T> {
    data: Vec<Vec<T>>,
}


pub trait New {
    fn new(s: &str) -> Self;
}

impl New for char {
    fn new(s: &str) -> Self { s.chars().nth(0).unwrap() }
}


impl<T: New> Table<T> {
    pub fn new(s: &Vec<&str>,) -> Self {
        let data = s
            .iter()
            .map(|&r|
                r.split(' ').map(|x| T::new(x)).collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();

        Self { data }
    }

    pub fn row_iter(&self, row: usize) -> impl Iterator<Item=&T> {
        self.data[row].iter()
    }

    pub fn col_iter(&self, col: usize) -> impl Iterator<Item=&T> {
        self.data.iter().map(move |v| v.iter().nth(col).unwrap())
    }

    pub fn w(&self) -> usize {
        self.data[0].len()
    }

    pub fn h(&self) -> usize {
        self.data.len()
    }
}

*/


#[derive(Debug, PartialEq, Eq, Hash,Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T:
    std::ops::Mul<T, Output = T> +
    std::ops::Add<T, Output = T> +
    std::ops::Sub<T, Output = T> +
    std::ops::Div<T, Output = T> +
    std::ops::AddAssign<T> +
    std::ops::SubAssign<T> +
    std::ops::MulAssign<T> +
    std::ops::DivAssign<T> +
    Copy
    > Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self{x, y}
    }

    pub fn mul(&self, k: T) -> Self {
        Self{x: self.x * k, y: self.y * k}
    }
    pub fn div(&self, k: T) -> Self {
        Self{x: self.x / k, y: self.y / k}
    }
    pub fn add(&self, q: &Self) -> Self {
        Self{x: self.x + q.x, y: self.y + q.y}
    }
    pub fn sub(&self, q: &Self) -> Self {
        Self{x: self.x - q.x, y: self.y - q.y}
    }

    pub fn imul(&mut self, q: &Self) {
        self.x *= q.x;
        self.y *= q.y;
    }
    pub fn idiv(&mut self, q: &Self) {
        self.x /= q.x;
        self.y /= q.y;
    }
    pub fn iadd(&mut self, q: &Self) {
        self.x += q.x;
        self.y += q.y;
    }
    pub fn isub(&mut self, q: &Self) {
        self.x -= q.x;
        self.y -= q.y;
    }

    pub fn euclidian(&self) -> T {
        self.x * self.x + self.y * self.y
    }

}
