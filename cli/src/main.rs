use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use compiler::parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let fstr = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    println!("{}", &fstr);

    let mut parser = parser::Parser::new(fstr, 0);
    let tokens = parser.lex();
    println!("{:?}", &tokens);

    Ok(())
}
