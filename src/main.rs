use std::collections::HashSet;
//use itertools::izip;

type Position = (usize, usize);

fn main() {
    let mut ttt = Tictactoe::new(3, 3);
    // println!("{:?}", ttt);
    ttt.select('o', (0, 2));
    ttt.select('o', (0, 1));
    ttt.select('o', (2, 1));
    ttt.select('o', (0, 0));
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
                    //self.win(self.x_fields.clone());
                    if self.win(self.x_fields.clone()) {
                         println!("X won!")
                    };
                } else if x_or_o == 'o' {
                    self.o_fields.insert(pos);
                       if self.win(self.o_fields.clone()) {
                         println!("O won!")
                         }
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
        for h in 0..self.width {
            for w in 0..self.height {
                if self.occup_fields.contains(&(h, w)) {
                    if self.x_fields.contains(&(h, w)) {
                        print!("[ X ]");
                    } else if self.o_fields.contains(&(h, w)) {
                        print!("[ O ]");
                    }
                } else {
                    print!("[{} {}]", h, w);
                }
            }
            println!("");
        }
    }
    fn win(&mut self, chk: HashSet<Position>) -> bool {
        //Horizontal
        let mut chk_h = Vec::new();
        let mut bo_h = Vec::new();
        for p in chk {
            chk_h.push(p.0);
            bo_h.push(chk_h.iter().filter(|&n| *n == p.0).count() == 3);
            //println!("{:?}", chk_h.iter().filter(|&n| *n == p.0).count() == 3);
            //println!("{:?}", &chk_h);
            
        }
        
        bo_h.contains(&true)
        
        // true
    }
}

// #[test]
// fn test() {
//     let ttt = Tictactoe::new(3, 3);
//     println!("{:?}", ttt);
//     println!("HEY IM HERE");
// }
// HEY THERE
