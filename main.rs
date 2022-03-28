mod cargo_book;
mod cargo_docs;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
enum Executable {
    #[clap(name = "docs")]
    Docs(cargo_docs::Options),
    #[clap(name = "book")]
    Book(cargo_book::Options),
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    Ok(match Executable::parse() {
        Executable::Docs(options) => {
            options.run().await?;
        }
        Executable::Book(options) => {
            options.run().await?;
        }
    })
}
