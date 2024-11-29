use std::process::exit;

use brainf::{opts::Opts, parser, state::State};

fn print_help_text(prog_name: &str) {
    eprintln!(
        "Usage: {prog_name} [ARGS] <FILE>

Where <FILE> is the bf file you want to execute, and [ARGS] are:

    -n CAP      Make the byte array have CAP elements (default: 30000)
    -A          Print raw digits (separated by space) instead of ASCII
    -h, --help  Print out this text
    "
    );
}

struct Args {
    prog_name: String,
    file: String,
    init_cap: usize,
    opts: Opts,
}

fn argparse() -> Args {
    let mut args = std::env::args();
    let prog_name = args.next().unwrap();
    let args = args.collect::<Vec<_>>();

    let mut print_face_value = false;
    let mut init_cap = 30000;
    let mut file = None;

    let mut i = 0;
    let len = args.len();
    while i < len {
        let arg = &args[i];
        if arg == "--help" || arg == "-h" {
            print_help_text(&prog_name);
            exit(0);
        } else if arg == "-A" {
            print_face_value = true;
        } else if arg == "-n" {
            init_cap = args[i + 1]
                .parse()
                .expect("Invalid capacity. Must be a `usize`");
            i += 1;
        } else {
            file = Some(arg.to_string());
        }
        i += 1;
    }
    if let None = file {
        print_help_text(&prog_name);
        exit(1);
    }
    Args {
        prog_name,
        file: file.unwrap(),
        init_cap,
        opts: Opts { print_face_value },
    }
}

fn main() {
    let Args {
        prog_name: _,
        file,
        init_cap,
        opts,
    } = argparse();
    let contents = std::fs::read_to_string(file).unwrap();

    let mut state = State::new(init_cap, &opts);
    let mut ops = vec![];
    parser::parse(&mut ops, &contents);
    state.exec(&ops);
}
