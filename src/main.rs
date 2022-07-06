use std::collections::HashSet;
//use itertools::izip;

type Position = (usize, usize);

fn main() {
    let mut ttt = Tictactoe::new(3, 3);
    // println!("{:?}", ttt);
    ttt.select('x', (0, 2));
    ttt.select('o', (0, 1));
    ttt.select('x', (0, 0));
    println!("{:?}", ttt);
    ttt.show();
}

#[derive(Debug)]
struct Tictactoe {
    width: usize,
    height: usize,
    occup_fields: HashSet < Position >,
    x_fields: HashSet < Position >,
    o_fields: HashSet < Position >,
}

impl Tictactoe {
    pub fn new(height: usize, width: usize) -> Tictactoe {
        Tictactoe {
            width,
            height,
            occup_fields: HashSet::new(),
            x_fields: HashSet::new(),
            o_fields: HashSet::new(),
        }
    }
    pub fn select(&mut self, x_or_o: char, pos: Position) {
        if self.width >= pos.0 && self.height >= pos.1 {
            if !self.occup_fields.contains(&pos) {
                if x_or_o == 'x' {
                    self.x_fields.insert(pos);
                } else if x_or_o == 'o' {
                    self.o_fields.insert(pos);
                } else {
                    panic!("Type x or o!")
                }
                self.occup_fields.insert(pos);
            } else {
                panic!("Field occupied!");
            }
        } else {
            panic!("Position out of range!")
        }
    }
    pub fn show(&mut self) {
        for w in 0..self.width {
            for h in 0..self.height {
                if self.occup_fields.contains(&(w, h)) {
                    if self.x_fields.contains(&(w, h)) {
                        print!("[.X.]");
                    } else {
                        print!("[.O.]");
                    }
                } else {
                    print!("[{}{}]", h, w);
                }
            }
            println!("");
        }
    }
}

// #[test]
// fn test() {
//     let ttt = Tictactoe::new(3, 3);
//     println!("{:?}", ttt);
//     println!("HEY IM HERE");
// }
// HEY THERE