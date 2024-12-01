use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix {
    data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Matrix {
    pub fn contains(&self, y: i32, x: i32) -> bool {
        y >= 0 && y < self.height as i32 &&
            x >= 0 && x < self.width as i32
    }

    pub fn new(data: Vec<Vec<char>>) -> Matrix {
        let (height, width) = calculate_dimensions(&data);
        Matrix { data, width, height }
    }
}

fn calculate_dimensions(data: &Vec<Vec<char>>) -> (usize, usize) {
    let height = data.len();
    let width = data.first()
        .expect("lines has to have first line")
        .len();
    (height, width)
}

impl Index<usize> for Matrix {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
