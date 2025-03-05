use clap::Parser;
use julian::system2jdn;
use moontool_rs::astro::phase;
use std::time::SystemTime;

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
    match ilumfrac {
        0.00..0.04 => 0,
        0.96..1.00 => 4,
        0.46..0.54 => {
            if half {
                6
            } else {
                2
            }
        }
        0.54..0.96 => {
            if half {
                5
            } else {
                3
            }
        }
        _ => {
            if half {
                7
            } else {
                1
            }
        }
    }
}

fn mprintf(p: moontool_rs::astro::MoonState, f: &str) -> String {
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
                    x = format!("{}", p.mage);
                    &x
                }
                Some('P') => {
                    x = format!("{:.2}", p.pphase * 100.0);
                    &x
                }
                Some('e') => EMOJIS[phaseidx(p.pphase, p.mage)],
                Some('s') => SEMOJI[phaseidx(p.pphase, p.mage)],
                Some('p') => PNAMES[phaseidx(p.pphase, p.mage)],
                _ => break,
            }),
            x => s.push(x),
        };
    }
    s
}

fn main() {
    let jdate_split = system2jdn(SystemTime::now()).unwrap();
    let jdate = jdate_split.0 as f64 + ((jdate_split.1 as f64) / 86400.0) - 0.5;
    let p = phase(jdate);
    println!("{}", mprintf(p, "%p %e (%P%%)"));
}
