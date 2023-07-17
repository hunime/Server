#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load DotENV file.
    dotenv::dotenv().ok();

    // Start rocket server.
    server::serve().launch().await?;

    Ok(())
}
