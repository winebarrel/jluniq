use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub(super) struct Options {
    pub file: String,
    pub key: String,
    pub group: bool,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [OPTIONS] [FILE]", program);
    print!("{}", opts.usage(&brief));
}

pub(super) fn parse_opts() -> Options {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let mut opts = getopts::Options::new();

    opts.optopt("k", "key", "JSON key to make it unique", "KEY");
    opts.optflag("g", "group", "Group rows with the same value");
    opts.optflag("v", "version", "Print version and exit");
    opts.optflag("h", "help", "Print usage and exit");

    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0)
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        process::exit(0)
    }

    let key = matches.opt_str("k").unwrap_or_else(|| {
        panic!("'-k' is required");
    });

    let group = matches.opt_present("g");

    let file = match matches.free.len() {
        1 => matches.free[0].to_string(),
        0 => "-".to_string(),
        _ => {
            print_usage(&program, opts);
            process::exit(1)
        }
    };

    Options {
        file: file,
        key: key,
        group: group,
    }
}
