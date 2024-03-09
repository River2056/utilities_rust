use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = App::new("echo")
        .version("1.0.0")
        .author("Kevin Tung <chen0625tung@gmail.com>")
        .about("Rust implementation of echo")
        .arg(
            Arg::with_name("text")
                .short("t")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let val = matches.value_of_lossy("text").unwrap();
    let val2 = matches.values_of("text").unwrap();

    println!("{:#?}", matches);
    println!("{:?}", val2);
}
