pub struct Rectangle {
    pub hight: u64,
    pub weight: u64,
}

impl Rectangle {
    pub fn area(&self) -> u64 {
        return self.hight * self.weight;
    }
    pub fn change_value(&mut self, x: u64, y: u64) -> &Rectangle {
        self.hight = x;
        self.weight = y;
        return self;
    }
}
