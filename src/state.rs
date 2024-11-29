use crate::{opts::Opts, parser};
use std::{io::Read, process::exit};

pub struct State<'a> {
    state: Box<[u8]>,
    cap: usize,
    curr: usize,
    opts: &'a Opts,
}

impl State<'_> {
    pub fn new<'a>(init_cap: usize, opts: &'a Opts) -> State<'a> {
        State {
            state: vec![0; init_cap].into_boxed_slice(),
            curr: 0,
            cap: init_cap,
            opts,
        }
    }

    pub fn exec(&mut self, ops: &[parser::Operation]) {
        let stdin = std::io::stdin();
        use parser::OpType::*;
        for op in ops {
            let parser::Operation(count, op) = op;
            match op {
                Increment => {
                    self.state[self.curr] =
                        (self.state[self.curr] as usize).wrapping_add(*count) as u8;
                }
                Decrement => {
                    self.state[self.curr] =
                        (self.state[self.curr] as usize).wrapping_sub(*count) as u8;
                }

                Left => {
                    self.curr = self.curr.wrapping_sub(*count);
                }
                Right => {
                    if self.curr + *count >= self.cap {
                        self.curr = *count % self.cap;
                    } else {
                        self.curr += *count;
                    }
                }

                Out => {
                    for _ in 0..*count {
                        if self.opts.print_face_value {
                            print!("{} ", self.state[self.curr]);
                        } else {
                            print!("{}", self.state[self.curr] as u8 as char);
                        }
                    }
                }
                In => {
                    for _ in 0..*count {
                        let mut buf = [0u8; 1];
                        let n = stdin
                            .lock()
                            .take(1)
                            .read(&mut buf)
                            .expect("Could not read from stdin");
                        if n == 0 {
                            exit(0);
                        }
                        self.state[self.curr] = buf[0] as u8;
                    }
                }

                Loop(ops) => {
                    while self.state[self.curr] != 0 {
                        self.exec(&ops);
                    }
                }

                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let contents = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        let opts = Opts::new(false);
        let mut state = State::new(20, &opts);
        let mut ops = vec![];
        parser::parse(&mut ops, &contents);
        state.exec(&ops);

        assert_eq!(state.state[..7], [0, 0, 72, 100, 87, 33, 10]);
        assert_eq!(state.curr, 6);
    }

    #[test]
    fn comment_loop() {
        let contents = "[This is a test comment loop ++,,,,<<,,,,<>>>.>>,[[[]]]]+++>+++";

        let opts = Opts::new(false);
        let mut state = State::new(2, &opts);
        let mut ops = vec![];
        parser::parse(&mut ops, &contents);
        state.exec(&ops);

        assert_eq!(state.state[..], [3, 3]);
        assert_eq!(state.curr, 1);
    }
}
