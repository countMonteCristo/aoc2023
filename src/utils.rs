pub type Result = std::result::Result<(), ()>;

#[derive(Debug, PartialEq, Eq, Hash,Clone, Copy, Default)]
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
