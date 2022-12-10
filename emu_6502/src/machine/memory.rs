#[derive(Clone)]
pub struct Memory {
    storage: Vec<i32>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            storage: Vec::new(),
        }
    }

    // screen is between $0200 to $05ff
    pub fn screen(&self)-> Vec<i32> {
        return (200..500)
        .collect::<Vec<i32>>()
        .iter()
        .map(|&x: &i32| self.read((x as usize)))
        .collect()
    }

    pub fn read(&self, index: usize) -> i32 {
        if self.storage.len() <= (index) {
            return 0;
        } else {
              return self.storage[index];
        }
    }

    pub fn write(&mut self, index: usize, value: i32) -> i32 {
        if self.storage.len() <= index{
            self.storage.resize(index + 1, 0);   
        }
        println!("{} {}", index, value);
        self.storage[index] = value;
        return value;
    }
}
