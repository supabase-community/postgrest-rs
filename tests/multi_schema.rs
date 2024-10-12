#[macro_use]
extern crate json; // array!, object!, value!

use postgrest::Postgrest;

use std::error::Error;

const REST_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn read_other_schema() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("username")
        .eq("username", "leroyjenkins")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body, array![]);

    let other_client = Postgrest::new(REST_URL).schema("personal");
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
#[cfg(not(feature = "serde"))]
async fn write_other_schema() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("status")
        .eq("username", "supabot")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["status"], "ONLINE");

    let other_client = Postgrest::new(REST_URL).schema("personal");
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
#[cfg(feature = "serde")]
async fn write_other_schema_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct User {
        status: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("status")
        .eq("username", "supabot")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["status"], "ONLINE");

    let other_client = Postgrest::new(REST_URL).schema("personal");
    let other_resp = other_client
        .from("users")
        .update(&User {
            status: "OFFLINE".to_string(),
        })?
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
    let client = Postgrest::new(REST_URL).schema("private");
    let resp = client.from("channels").select("*").execute().await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "The schema must be one of the following: public, personal"
    );

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn write_nonexisting_schema() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL).schema("private");
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
        "The schema must be one of the following: public, personal"
    );

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn write_nonexisting_schema_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct Channel {
        slug: String,
    }

    let client = Postgrest::new(REST_URL).schema("private");
    let resp = client
        .from("channels")
        .update(&Channel {
            slug: "private".to_string(),
        })?
        .eq("slug", "random")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "The schema must be one of the following: public, personal"
    );

    Ok(())
}

#[tokio::test]
async fn other_schema_rpc() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL).schema("personal");
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
    let client = Postgrest::new(REST_URL).schema("personal");
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
    let client = Postgrest::new(REST_URL).schema("private");
    let resp = client
        .rpc("get_status", "{\"name_param\": \"leroyjenkins\"}")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(
        body["message"],
        "The schema must be one of the following: public, personal"
    );

    Ok(())
}
