use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]
struct CliOptions {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long, default_value = "hello.txt")]
    output: String,

    #[clap(short, long, default_value = "world")]
    greeting: String,
}

/**
 *  cargo run --example clap -- --output=output.txt --greeting=Hi
 */
fn main() {
    let opts: CliOptions = CliOptions::parse();
    
    if opts.verbose {
        println!("Verbose mode enabled");
    }

    println!("Greeting: {}", opts.greeting);
    println!("Output file: {}", opts.output);
}
