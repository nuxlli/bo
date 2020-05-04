use exitfailure::ExitFailure;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;

// use handlebars::Handlebars;

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(StructOpt)]
#[structopt(about = DESCRIPTION)]
struct Cli {
    // /// Template file to process. Omit to use standard input, or use --in or --input-dir (default [-])
    #[structopt(short = "f", long, parse(from_os_str))]
    file: Option<PathBuf>,

    // // Directory which is examined recursively for templates (alternative to --file and --in)
    // #[structopt(long, parse(from_os_str))]
    // input_dir: PathBuf,
    /// Template string to process (alternative to --file and --input-dir)
    #[structopt(short = "i", long)]
    r#in: Option<String>,
}

fn process(buf: String) {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&buf)
        .unwrap();

    let globals = liquid::object!({
        "num": 4f64
    });

    let output = template.render(&globals).unwrap();
    println!("{}", output);
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut buffer = String::new();

    // From input arg
    if let Some(input) = args.r#in {
        buffer = input;
    // From file arg
    } else if let Some(file_path) = args.file {
        if file_path.as_path().exists() {
            let file = File::open(file_path)?;
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut buffer)?;
        }
        // TODO: Show error for not found file
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
    };

    process(buffer);

    Ok(())
}
