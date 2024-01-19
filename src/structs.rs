extern crate rand;

use rand::Rng;

#[derive(Clone)]
pub struct Mine {
    position: (u32, u32),
    active: bool,
}

pub struct MineField {
    pub size: (u32, u32),
    pub mines: Vec<Mine>,
}

impl MineField {
    pub fn find_by_coordinates(&mut self, x: u32, y: u32) -> bool {
        for element in self.mines.iter() {
            if element.position.0 == x && element.position.1 == y && element.active == false {
                println!("Good job!");
                return false;
            }
        }
        true
    }

    fn generate_empty_fields(&mut self) {
        let mut x = self.size.0;
        let mut y = self.size.1;

        while x > 0 {
            while y > 0 {
                let mine = Mine {
                    position: (x - 1, y - 1),
                    active: false,
                };
                &self.mines.push(mine);
                y -= 1;
            }
            y = self.size.1;
            x -= 1;
        }
    }

    pub fn generate_mines(&mut self) {
        &self.generate_empty_fields();

        for element in self.mines.iter_mut() {
            if 1 == rand::thread_rng().gen_range(0..3) {
                (*element).active = true;
            }
        }

        let _ = &self.mines.reverse();
    }

    pub fn print_fields(&self) {
        // let x = self.size.0;
        let y = self.size.1;

        for i in self.mines.iter() {
            print!("x");
            if (y - 1) == i.position.1 {
                println!("");
            }
        }
        println!("");
    }

    pub fn print_field_solved(&self) {
        let y = self.size.1;

        for i in self.mines.iter() {
            if i.active {
                print!("o");
            } else {
                print!("x");
            }
            if (y - 1) == i.position.1 {
                println!("");
            }
        }
        println!("");
    }

    pub fn new(size: (u32, u32), mines: Vec<Mine>) -> MineField {
        MineField { size, mines }
    }
}
