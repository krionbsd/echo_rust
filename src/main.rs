use clap::{App, Arg};
use anyhow::Result;

fn main() -> Result<()> {
    let matches = App::new("echo_rust")
        .version("0.1")
        .author("KP")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("rm_line")
                .short("n")
                .help("Don't print new line")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let rm_line = matches.is_present("rm_line");

    print!("{}{}", text.join(" "), if rm_line { "" } else { "\n" });
    Ok(())
}
