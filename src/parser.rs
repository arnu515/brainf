use OpType::*;

#[derive(PartialEq, Debug)]
pub enum OpType {
    Increment,
    Decrement,

    Right,
    Left,

    Out,
    In,

    NoOp,

    Loop(Vec<Operation>),
}

#[derive(PartialEq, Debug)]
pub struct Operation(pub usize, pub OpType);

impl Operation {
    fn new(value: char, count: usize) -> Self {
        Self(
            count,
            match value {
                '+' => Increment,
                '-' => Decrement,
                '>' => Right,
                '<' => Left,
                '.' => Out,
                ',' => In,
                _ => NoOp,
            },
        )
    }
}

pub fn parse(ops: &mut Vec<Operation>, contents: &str) {
    let mut last: char = '\0';
    let mut last_count: usize = 0;
    let mut loop_stack = vec![];
    for c in contents.chars() {
        match c {
            '[' | ']' | ',' | '.' | '+' | '-' | '<' | '>' => (),
            _ => continue,
        }
        if last != '\0' && last != c && last != '[' && last != ']' {
            let ops = loop_stack.last_mut().unwrap_or(ops);
            ops.push(Operation::new(last, last_count));
        }
        match c {
            '[' => {
                let new_loop = vec![];
                loop_stack.push(new_loop);
                last = '[';
                last_count = 1;
            }
            ']' => {
                let loop_vec = loop_stack
                    .pop()
                    .expect("Expected loop start character ([) before loop end (]).");
                let prev_vec = loop_stack.last_mut().unwrap_or(ops);
                prev_vec.push(Operation(1, Loop(loop_vec)));
                last = ']';
                last_count = 1;
            }
            ',' | '.' | '+' | '-' | '<' | '>' => {
                if last == c {
                    last_count += 1;
                } else {
                    last = c;
                    last_count = 1;
                }
            }
            _ => unreachable!(),
        }
    }
    if !loop_stack.is_empty() {
        let loop_vec = loop_stack
            .pop()
            .expect("Expected loop start character ([) before loop end (]).");
        ops.push(Operation(1, Loop(loop_vec)));
    }
    if last != '\0' && last != '[' && last != ']' {
        let ops = loop_stack.last_mut().unwrap_or(ops);
        ops.push(Operation::new(last, last_count));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blank_string() {
        let str = "";
        let mut vec: Vec<Operation> = Vec::new();

        parse(&mut vec, str);
        assert!(vec.is_empty());
    }

    #[test]
    fn no_loops() {
        let str = "+++---..,,<<>>";
        let mut vec: Vec<Operation> = Vec::new();

        parse(&mut vec, str);
        assert_eq!(
            vec,
            vec![
                Operation(3, Increment),
                Operation(3, Decrement),
                Operation(2, Out),
                Operation(2, In),
                Operation(2, Left),
                Operation(2, Right),
            ]
        )
    }

    #[test]
    fn ignores_other_chars_no_loops() {
        let str = "+The quick\nbrown fox\tjumps over the lazy dog.";
        let mut vec: Vec<Operation> = Vec::new();

        parse(&mut vec, str);
        assert_eq!(vec, vec![Operation(1, Increment), Operation(1, Out)])
    }

    #[test]
    fn works_with_loops() {
        let str = "++[--].";
        let mut vec: Vec<Operation> = Vec::new();

        parse(&mut vec, str);
        assert_eq!(
            vec,
            vec![
                Operation(2, Increment),
                Operation(1, Loop(vec![Operation(2, Decrement)])),
                Operation(1, Out)
            ]
        )
    }
}
