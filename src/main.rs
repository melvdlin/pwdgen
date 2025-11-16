use clap::builder::NonEmptyStringValueParser;
use clap::Parser;
use itertools::Itertools;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(name = "pwdgen")]
#[command(about = "A simple secure random password generator.", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    /// Length of the password
    len: usize,

    #[arg(
        short, long, value_name = "CHARS",
        default_value = PRINTABLE_ASCII,
        value_parser = NonEmptyStringValueParser::new()
    )]
    /// The character pool to draw from. Duplicates will not be eliminated.
    pool: String,
}

const PRINTABLE_ASCII: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

fn main() {
    let args = Args::parse();

    let len = args.len;
    let pool = args.pool.chars().collect_vec();
    let pwd = pool
        .choose_multiple(&mut rand::thread_rng(), len)
        .collect::<String>();

    println!("{pwd}");
}
