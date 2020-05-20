use postgrest::Postgrest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Postgrest::new("http://localhost:3000");
    let resp = client.from("todos").select("*").execute().await?;
    println!("{}", resp.text().await?);
    Ok(())
}
