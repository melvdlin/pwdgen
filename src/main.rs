use ::clap::builder::NonEmptyStringValueParser;
use ::clap::builder::TypedValueParser;
use ::clap::Parser;
use ::itertools::Itertools;
use ::rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(name = "pwdgen")]
#[command(about = "A simple secure random password generator.", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8)]
    /// Length of the password
    len: usize,

    #[arg(
        long, value_name = "NAME",
        value_parser = NonEmptyStringValueParser::new().try_map(|s| Set::try_from_str(&s)),
    )]
    /// The character set to draw from. Character set names are case-insensitive.
    ///
    /// Possible values are printable-ascii, base64 and alphanumeric.
    ///
    /// Defaults to printable-ascii if no character set or pool was specified.
    set: Option<Set>,

    #[arg(
        short, long, value_name = "CHARS",
        value_parser = NonEmptyStringValueParser::new()
    )]
    /// The character pool to draw from. Additive with `set`. Duplicates will not be eliminated.
    pool: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Set {
    #[default]
    PrintableAscii,
    Base64,
    Alphanumeric,
}
fn main() {
    let args = Args::parse();

    let len = args.len;

    let set = args.set.map(|set| set.chars().chars());
    let pool = args.pool.as_ref().map(|pool| pool.chars());

    let mut chars = set.into_iter().chain(pool).flatten().collect_vec();
    if chars.is_empty() {
        eprintln!("No character pool was specified. Defaulting to printable-ascii.");
        chars = Set::PrintableAscii.chars().chars().collect_vec();
    }

    let pwd = chars
        .choose_multiple(&mut rand::thread_rng(), len)
        .collect::<String>();

    println!("{pwd}");
}

impl Set {
    fn try_from_str(s: &str) -> Result<Self, String> {
        let variants = [Set::PrintableAscii, Set::Base64, Set::Alphanumeric];
        let tags: &[&[&str]] = &[
            &[
                "printableascii",
                "printable_ascii",
                "printable-asci",
                "printable ascii",
            ],
            &["base64", "base_64", "base-64", "base 64"],
            &["alphanumeric"],
        ];

        let s = s.to_lowercase();
        tags.iter()
            .zip(variants)
            .find_map(|(&tags, set)| (tags.iter().any(|&tag| tag == s)).then_some(set))
            .ok_or(format!(
                "unknown character set; must be one of [{}]",
                tags.iter().copied().flatten().join(", ")
            ))
    }

    fn chars(self) -> &'static str {
        match self {
            | Set::PrintableAscii => PRINTABLE_ASCII,
            | Set::Base64 => BASE_64,
            | Set::Alphanumeric => ALPHANUMERIC,
        }
    }
}

const PRINTABLE_ASCII: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;
const BASE_64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1023456789+/";
const ALPHANUMERIC: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1023456789";
