pub type Value = f64;

#[derive(Default, Clone)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn get(&self, index: usize) -> Value {
        self.values[index]
    }

    pub fn add_constant(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn get_count(&self) -> usize {
        self.values.len()
    }

    pub fn print_value(value: &Value) {
        print!("{value}")
    }
}