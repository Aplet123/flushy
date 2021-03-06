use clap::Clap;
use std::io::{Write,BufReader,BufRead};
use std::fs;
use std::fs::File;
use std::str::FromStr;
use std::format;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flush {
    Flush,
    FlushHyphen
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    AlignMode(Align),
    FlushMode(Flush)
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "left"          => Ok(Mode::AlignMode(Align::Left)),
            "center"        => Ok(Mode::AlignMode(Align::Center)),
            "right"         => Ok(Mode::AlignMode(Align::Right)),
            "flush"         => Ok(Mode::FlushMode(Flush::Flush)),
            "flush-hyphen"  => Ok(Mode::FlushMode(Flush::FlushHyphen)),
            _               => Ok(Mode::AlignMode(Align::Left))
        }
    }
}

#[derive(Clap, Clone, Debug, PartialEq, Eq)]
#[clap(version = "1.0.0", author = "poxter <generalpoxter@gmail.com>", about = "Aligns/flushes text files")]
struct Opts {
    #[clap(
        short = 'c',
        long = "columns",
        about = "Sets width of text file in columns",
        takes_value = true,
        value_name = "# cols",
        default_value = "80"
    )]
    col: usize,
    #[clap(
        short = 'm',
        long = "mode",
        about = "Sets align/flush mode",
        takes_value = true,
        value_name = "name",
        possible_values = &["left", "right", "center", "flush", "flush-hyphen"],
        default_value = "left"
    )]
    mode: String,
    #[clap(
        short = 'o',
        long = "output",
        about = "Path to output file; input file is overwritten if not specified",
        takes_value = true,
        value_name = "path",
    )]
    output: Option<String>,
    #[clap(
        about = "Path to file to format",
        value_name = "INPUT",
        required = true
    )]
    input: String
}

fn main() {
    let opts: Opts = Opts::parse();
    let mode = Mode::from_str(&opts.mode).unwrap();

    let mut tmp = File::create("/tmp/flushy.txt")
        .expect("Couldn't create /tmp/flushy.txt");
    let reader = BufReader::new(File::open(&opts.input)
        .expect(&format!("Couldn't read {}", opts.input))
    );

    for line in reader.lines() {
        let line_format = format(&line.unwrap(), opts.col, mode);
        write!(tmp, "{}", line_format)
            .expect("Couldn't write to /tmp/flushy.txt");
    }

    let out;
    match opts.output {
        Some(path)  => out = path,
        None        => out = opts.input
    };

    fs::copy("/tmp/flushy.txt", &out)
        .expect(&format!("Couldn't write to {}", out));
}

fn format(line: &str, col: usize, mode: Mode) -> String {
    let mut out = Vec::<String>::new();
    let mut cur = String::new();
    let mut last_space = 0;

    for c in line.chars() {
        if length(&cur) == 0 && c != ' ' || length(&cur) > 0 {
            if c == ' ' {
                last_space = length(&cur);
            }
            cur.push(c);
        }

        if length(&cur) == col {
            out.push(format_line(&cur, col, last_space, mode));

            match mode {
                Mode::AlignMode(_align) => {
                    if last_space == 0 || last_space == col - 1 {
                        cur = String::new();
                    } else {
                        cur = (&cur[(last_space+1)..]).to_string();
                    }
                },

                Mode::FlushMode(flush) => {
                    match flush {
                        Flush::Flush => {
                            cur = String::new();
                        },

                        Flush::FlushHyphen => {
                            if last_space == col - 1 {
                                cur = String::new();
                            } else {
                                cur = cur.chars().last().unwrap().to_string();
                            }
                        }
                    }
                }
            }

            last_space = 0;
        }
    }

    last_space = length(&cur);
    out.push(format_line(&cur, col, last_space, mode));

    return out.join("\n");
}

fn format_line(line: &str, col: usize, last_space: usize, mode: Mode) -> String {
    if last_space == 0 {
        return line.to_string();
    }

    match mode {
        Mode::AlignMode(align) => {
            let mut tmp = String::new();
            let tmp_txt = &line[..last_space];

            match align {
                Align::Left => {
                    tmp.push_str(tmp_txt);
                },

                Align::Center => {
                    tmp.push_str(&" ".repeat((col-last_space)/2));
                    tmp.push_str(tmp_txt);
                    tmp.push_str(&" ".repeat(col-length(&tmp)));
                },

                Align::Right => {
                    tmp.push_str(&" ".repeat(col-last_space));
                    tmp.push_str(tmp_txt);
                }
            }

            return tmp;
        },

        Mode::FlushMode(flush) => {
            let mut tmp = line.to_string();

            match flush {
                Flush::Flush => {
                    return tmp;
                },

                Flush::FlushHyphen => {
                    if length(line) == col && last_space != col - 1 {
                        tmp = (&tmp[..col-1]).to_string();
                        if last_space != col - 2 {
                            tmp.push_str("-");
                        }
                    }
                    return tmp;
                }
            }
        }
    }
}

fn length(x: &str) -> usize {
    return x.chars().count();
}
