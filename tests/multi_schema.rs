#[macro_use]
extern crate json; // array!, object!, value!

use postgrest::Postgrest;

use std::error::Error;

const DEFAULT_REST_URL: &str = "http://localhost:3000";

fn create_client() -> Postgrest {
    let url = std::env::var("REST_URL").unwrap_or_else(|_| DEFAULT_REST_URL.to_string());
    let client = Postgrest::new(&url);
    if let Ok(key) = std::env::var("APIKEY") {
        client.insert_header("apikey", &key)
    } else {
        client
    }
}

#[tokio::test]
async fn read_other_schema() -> Result<(), Box<dyn Error>> {
    let client = create_client();
    let resp = client
        .from("users")
        .select("username")
        .eq("username", "leroyjenkins")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body, array![]);

    let other_client = create_client().schema("personal");
    let other_resp = other_client
        .from("users")
        .select("username")
        .eq("username", "leroyjenkins")
        .execute()
        .await?;
    let other_body = other_resp.text().await?;
    let other_body = json::parse(&other_body)?;

    assert_eq!(other_body, array![{"username": "leroyjenkins"}]);

    Ok(())
}

#[tokio::test]
async fn write_other_schema() -> Result<(), Box<dyn Error>> {
    let client = create_client();
    let resp = client
        .from("users")
        .select("status")
        .eq("username", "supabot")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["status"], "ONLINE");

    let other_client = create_client().schema("personal");
    let other_resp = other_client
        .from("users")
        .update("{\"status\": \"OFFLINE\"}")
        .eq("username", "supabot")
        .execute()
        .await?;
    let other_body = other_resp.text().await?;
    let other_body = json::parse(&other_body)?;

    assert_eq!(other_body[0]["status"], "OFFLINE");

    Ok(())
}

#[tokio::test]
async fn read_nonexisting_schema() -> Result<(), Box<dyn Error>> {
    let client = create_client().schema("private");
    let resp = client.from("channels").select("*").execute().await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "Invalid schema: private"
    );

    Ok(())
}

#[tokio::test]
async fn write_nonexisting_schema() -> Result<(), Box<dyn Error>> {
    let client = create_client().schema("private");
    let resp = client
        .from("channels")
        .update("{\"slug\": \"private\"}")
        .eq("slug", "random")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "Invalid schema: private"
    );

    Ok(())
}

#[tokio::test]
async fn other_schema_rpc() -> Result<(), Box<dyn Error>> {
    let client = create_client().schema("personal");
    let resp = client
        .rpc("get_status", "{\"name_param\": \"leroyjenkins\"}")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body, "ONLINE");

    Ok(())
}

#[tokio::test]
async fn nonexisting_rpc_in_schema() -> Result<(), Box<dyn Error>> {
    let client = create_client().schema("personal");
    let resp = client
        .rpc("nonexistent_procedure", "{\"param\": 0}")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "Could not find the function personal.nonexistent_procedure(param) in the schema cache"
    );

    Ok(())
}

#[tokio::test]
async fn nonexisting_schema_for_rpc() -> Result<(), Box<dyn Error>> {
    let client = create_client().schema("private");
    let resp = client
        .rpc("get_status", "{\"name_param\": \"leroyjenkins\"}")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "Invalid schema: private"
    );

    Ok(())
}
