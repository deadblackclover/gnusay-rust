use clap::{App, Arg};

fn main() {
    let matches = App::new("gnusay")
        .version("0.1.1")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("GNU say made using Rust")
        .arg(
            Arg::with_name("TEXT")
                .help("Text to display")
                .required(true)
                .index(1),
        )
        .get_matches();

    let text = matches.value_of("TEXT").unwrap();

    println!(
        "   {} \n  <{}>\n   {} ",
        "_".repeat(text.len()),
        text,
        "-".repeat(text.len())
    );
    println!("    \\   / \n     \\ / ");
    println!(" .= .-_-. =.\n((_/)o o(\\_)) \n `-'(. .)`-' \n  /| \\_/ |\\ \n ( | GNU | ) \n /'\\_____/'\\ \n \\__)   (__/");
}
