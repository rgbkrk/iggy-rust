use std::io::Write;
use std::io;

use clap::Parser;

/// Iggy: Fetches and writes a .gitignore for a given language
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Kyle Kelley", about="Fetches and writes a .gitignore for a given language")]
struct Args {
    /// Programming language to fetch a .gitignore for
    #[arg(required = true, index=1)]
    language: String,
}

async fn fetch_gitignore_template(lang: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore", lang);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Template not found")))
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let lang = &args.language;

    println!("Fetching .gitignore for {}", lang);

    let rt = tokio::runtime::Runtime::new()?;
    let body = rt.block_on(fetch_gitignore_template(lang))?;

    let mut file = std::fs::File::create(".gitignore")?;

    let buf = format!("{}\n", body);

    file.write_all(buf.as_bytes())?;

    Ok(())
}
