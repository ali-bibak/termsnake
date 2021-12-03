use crate::constvals::Consts;

pub struct Model {
    map: [[char; Consts::WINDOW_W]; Consts::WINDOW_H],
}

impl Model {
    pub fn new() -> Self {
        let model = Model {
            map: [['e'; Consts::WINDOW_W]; Consts::WINDOW_H],
        };
        return model;
    }

    pub fn get_map(&self) -> [[char; Consts::WINDOW_W]; Consts::WINDOW_H] {
        return self.map.clone();
    }

    pub fn debug(&self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                print!("{}", self.map[i][j]);
            }
            println!();
        }
    }
}