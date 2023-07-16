#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::serve().launch().await?;
    Ok(())
}
