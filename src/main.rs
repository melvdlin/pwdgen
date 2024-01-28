use clap::Parser;
use itertools::Itertools;
use rand::{thread_rng, CryptoRng, Rng};

#[derive(Parser, Debug)]
#[command(name = "pwdgen")]
#[command(about = "A simple secure random password generator.", long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 8)]
    len: usize,

    /// The character pool to draw from. Duplicates will not be eliminated.
    #[arg(short, long, value_name = "CHARS", default_value = PRINTABLE_ASCII)]
    pool: String,
}

const PRINTABLE_ASCII: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

fn main() {
    let args = Args::parse();

    let len = args.len;
    let pool = args.pool.chars().collect_vec();
    let mut rng = thread_rng();
    let pwd = (0..len).map(|_| pick(&mut rng, &pool)).collect::<String>();

    println!("{pwd}");
}

fn pick<T: CryptoRng + Rng>(rng: &mut T, pool: &[char]) -> char {
    pool[rng.gen_range(0..pool.len())]
}
