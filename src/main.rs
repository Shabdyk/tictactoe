use std::collections::HashSet;

type Position = (usize, usize);

fn main() {
    let mut ttt = Tictactoe::new(3, 3);
    // println!("{:?}", ttt);
    ttt.select('o', (1, 1));
    println!("{:?}", ttt);
}

#[derive(Debug)]
struct Tictactoe {
    width: usize,
    height: usize,
    occup_fields: HashSet<Position>,
    x_fields: HashSet<Position>,
    o_fields: HashSet<Position>,
}

impl Tictactoe {
    pub fn new(width: usize, height: usize) -> Tictactoe {
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
            }
            self.occup_fields.insert(pos);
        } else {
            panic!("Position out of range!")
        }
    }
    pub fn show(&mut self) {
        for h in self.height {
            for w in self.width {
                println!("_");
            }
        }
    }
}

// #[test]
// fn test() {
//     let ttt = Tictactoe::new(3, 3);
//     println!("{:?}", ttt);
//     println!("HEY IM HERE");
// }
