use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "   +{}+ \n   |{}|\n   +{}+ ",
        "-".repeat(args.text.len()),
        args.text,
        "-".repeat(args.text.len())
    );
    println!("    \\   / \n     \\ / ");
    println!(" .= .-_-. =.\n((_/)o o(\\_)) \n `-'(. .)`-' \n  /| \\_/ |\\ \n ( | GNU | ) \n /'\\_____/'\\ \n \\__)   (__/");
}
