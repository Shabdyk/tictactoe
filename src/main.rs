use std::collections::HashSet;
//use itertools::izip;

type Position = (usize, usize);

fn main() {
    let mut ttt = Tictactoe::new(3, 3);
    // println!("{:?}", ttt);
    ttt.select('x', (2, 2));
    println!("{:?}", ttt);
    ttt.show();
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
        for h in 0..self.height {
            for w in 0..self.width {
                for i in &self.occup_fields{
                    if w == i.0 && h ==i.1 {
                        if self.x_fields.contains(&(w,h)){
                            print!("[X]");
                        } else{
                            print!("[O]")
                        }
                    } else {
                        print!("[  ]");
                    }
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
