use std::ops::Add;

struct Kitty<T> {
    value: T,
}

impl<T: Add<Output = T> + Copy> Add for Kitty<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Kitty {
            value: self.value + other.value,
        }
    }
}

impl<T: Add<Output = T> + Copy> Kitty<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let kitty_1 = Kitty::new(10);
        let kitty_2 = Kitty::new(20);

        let result = kitty_1 + kitty_2;
        println!("Result: {}", result.value);
    }
}