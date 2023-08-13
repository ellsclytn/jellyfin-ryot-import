mod error;
mod jellyfin;
mod ryot;

use error::Result;
use jellyfin::Jellyfin;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let jf = Jellyfin::new()?;

    match query.as_str() {
        "shows" => jf.get_ryot_shows_json().await?,
        "movies" => jf.get_ryot_movies_json().await?,
        _ => {
            eprintln!("Invalid query");
            std::process::exit(1);
        }
    };

    Ok(())
}
