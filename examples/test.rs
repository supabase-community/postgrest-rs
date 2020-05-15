use postgrest::Postgrest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Postgrest::new("https://hacks.soedirgo.dev/postgrest");
    let resp = client
        .from("todos")
        .select("*")
        .execute()
        .await?;
    println!("{}", resp.text().await?);
    Ok(())
}
