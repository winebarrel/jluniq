mod cli;
mod ndjson;

use std::fs;
use std::io;

fn main() {
    let opts = cli::parse_opts();

    let uniq_opts = ndjson::Opts { group: opts.group };

    if opts.file == "-" {
        let reader = io::BufReader::new(io::stdin());
        ndjson::uniq(reader, &opts.key, io::stdout(), uniq_opts)
    } else {
        let f = fs::File::open(opts.file).unwrap();
        let reader = io::BufReader::new(f);
        ndjson::uniq(reader, &opts.key, io::stdout(), uniq_opts)
    }
    .unwrap();
}
