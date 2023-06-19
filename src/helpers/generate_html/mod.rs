use tera::Tera;
use anyhow::Error;

pub async fn generate() -> Result<&str, Error> {
// Use globbing
let tera = match Tera::new("templates/**/*.html") {
    Ok(t) => t,
    Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
    }
};
    Ok("")
}
