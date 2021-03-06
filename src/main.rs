use std::collections::HashSet;
//use itertools::izip;

type Position = (usize, usize);

fn main() {
    let mut ttt = Tictactoe::new(10, 10);
    // println!("{:?}", ttt);
    ttt.select('x', (1, 3));
    // ttt.show();
    ttt.select('x', (1, 4));
    // ttt.show();
    ttt.select('o', (7, 1));
    ttt.select('x', (1, 2));
    // ttt.show();
    ttt.select('x', (1, 5));
    ttt.select('x', (1, 6));
    // println!("{:?}", ttt);
    // ttt.show();
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
                        println!("X won! {:?}", self.x_fields)
                    };
                } else if x_or_o == 'o' {
                    self.o_fields.insert(pos);
                    if self.win(self.o_fields.clone()) {
                        println!("O won! {:?}", self.o_fields)
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
        self.show();
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
                    // print!("[{} {}]", h, w);
                    print!("[   ]");
                }
            }
            println!("");
        }
        println!("____________________")
    }
    fn win(&mut self, chk: HashSet<Position>) -> bool {
        //Horizontal/Vertical for nxm
        let mut res_h = false;
        let mut res_v = false;
        let mut res_d = false;
        for (n, m) in chk.clone() {
            res_v = chk.contains(&(n, m + 1))
                && chk.contains(&(n, m + 2))
                && chk.contains(&(n, m + 3))
                && chk.contains(&(n, m + 4));
            res_h = chk.contains(&(n + 1, m))
                && chk.contains(&(n + 2, m))
                && chk.contains(&(n + 3, m))
                && chk.contains(&(n + 4, m));
            res_d = chk.contains(&(n + 1, m + 1))
                && chk.contains(&(n + 2, m + 2))
                && chk.contains(&(n + 3, m + 3))
                && chk.contains(&(n + 4, m + 4));
            if res_v || res_h || res_d {
                break;
            }
        }
        res_h || res_v || res_d
    }
}

// #[test]
// fn test() {
//     let ttt = Tictactoe::new(3, 3);
//     println!("{:?}", ttt);
//     println!("HEY IM HERE");
// }
// HEY THERE
