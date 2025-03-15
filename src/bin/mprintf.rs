use clap::Parser;

/// A program to print formatted information about the moon
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Formatting Strings
    #[arg(short, long)]
    format: String,
}

const HALFMON: f64 = 14.76529434;
const EMOJIS: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];
const SEMOJI: [&str; 8] = ["🌑", "🌘", "🌗", "🌖", "🌕", "🌔", "🌓", "🌒"];
const PNAMES: [&str; 8] = [
    "New",
    "Waxing Crescent",
    "First Quarter",
    "Waxing Gibbous",
    "Full",
    "Waning Gibbous",
    "Last Quarter",
    "Waning Crescent",
];

fn phaseidx(ilumfrac: f64, mage: f64) -> usize {
    let half: bool = mage > HALFMON;

    match (ilumfrac, half) {
        (0.00..0.04, _) => 0,
        (0.96..1.00, _) => 4,
        (0.46..0.54, true) => 6,
        (0.46..0.54, false) => 2,
        (0.54..0.96, true) => 5,
        (0.54..0.96, false) => 3,
        (_, true) => 7,
        (_, false) => 1,
    }
}

fn mprintf(p: (f64, f64), f: &str) -> String {
    let mut s = String::new();
    let mut chr = f.chars();
    let mut x: String;
    while let Some(c) = chr.next() {
        match c {
            '%' => s.push_str(match chr.next() {
                Some('n') => "\n",
                Some('t') => "\t",
                Some('%') => "%",
                Some('a') => {
                    x = format!("{}", p.1);
                    &x
                }
                Some('P') => {
                    x = format!("{:.2}", p.0 * 100.0);
                    &x
                }
                Some('e') => EMOJIS[phaseidx(p.0, p.1)],
                Some('s') => SEMOJI[phaseidx(p.0, p.1)],
                Some('p') => PNAMES[phaseidx(p.0, p.1)],
                _ => break,
            }),
            x => s.push(x),
        };
    }
    s
}

fn main() {
    let p = pracstro::moon::MOON.phase(pracstro::time::Date::now());
    println!("{}", mprintf(p, "%p %e (%P%%)"));
}
