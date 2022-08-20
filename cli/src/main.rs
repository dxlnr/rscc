use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
// use compiler::parser;
use compiler::parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let fstr = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let mut tokenizer = parser::token::Tokenizer::new(fstr);
    while let Some(token) = tokenizer.next() {
        println!("{:?}", &token);
    }

    Ok(())
}
