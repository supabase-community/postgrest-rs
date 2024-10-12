use postgrest::Postgrest;

use std::error::Error;

const REST_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn basic_data() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("username")
        .eq("status", "OFFLINE")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "kiwicopple");

    Ok(())
}

#[tokio::test]
async fn special_characters_eq() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("username")
        .eq("username", "ihave.special,c:haracter(s)")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "ihave.special,c:haracter(s)");

    Ok(())
}

#[tokio::test]
async fn special_characters_neq() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .select("username")
        .neq("username", "ihave.special,c:haracter(s)")
        .and("catchphrase.eq.fat bat")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert!(body.is_empty());

    Ok(())
}

#[tokio::test]
async fn relational_join() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("channels")
        .select("slug, messages(message)")
        .eq("slug", "public")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["messages"][0]["message"], "Hello World 👋");
    assert_eq!(body[0]["slug"], "public");

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn insert() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("messages")
        .insert(r#"[{"message": "Test message 0", "channel_id": 1, "username": "kiwicopple"}]"#)
        .execute()
        .await?;
    let status = resp.status();

    assert_eq!(status.as_u16(), 201);

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn insert_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct Message {
        message: String,
        channel_id: i32,
        username: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("messages")
        .insert(&[Message {
            message: "Test message 1".to_string(),
            channel_id: 1,
            username: "kiwicopple".to_string(),
        }])?
        .execute()
        .await?;
    let status = resp.status();

    assert_eq!(status.as_u16(), 201);

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn upsert() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(
            r#"[{"username": "dragarcia", "status": "OFFLINE"},
                {"username": "supabot2", "status": "ONLINE"}]"#,
        )
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "dragarcia");
    assert_eq!(body[1]["username"], "supabot2");

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn upsert_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct User {
        username: String,
        status: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(&[
            User {
                username: "dragarcia".to_string(),
                status: "OFFLINE".to_string(),
            },
            User {
                username: "supabot2".to_string(),
                status: "ONLINE".to_string(),
            },
        ])?
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "dragarcia");
    assert_eq!(body[1]["username"], "supabot2");

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn upsert_existing() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(r#"{"username": "dragarcia", "status": "ONLINE"}"#)
        .on_conflict("username")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "dragarcia");
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn upsert_existing_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct User {
        username: String,
        status: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(&User {
            username: "dragarcia".to_string(),
            status: "ONLINE".to_string(),
        })?
        .on_conflict("username")
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "dragarcia");
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn upsert_nonexisting() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(r#"{"username": "supabot3", "status": "ONLINE"}"#)
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "supabot3");
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn upsert_nonexisting_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct User {
        username: String,
        status: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .upsert(&User {
            username: "supabot3".to_string(),
            status: "ONLINE".to_string(),
        })?
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(body[0]["username"], "supabot3");
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
#[cfg(not(feature = "serde"))]
async fn update() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .eq("status", "ONLINE")
        .update(r#"{"status": "ONLINE"}"#)
        .execute()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(status.as_u16(), 200);
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
#[cfg(feature = "serde")]
async fn update_serde() -> Result<(), Box<dyn Error>> {
    #[derive(serde::Serialize)]
    struct User {
        status: String,
    }

    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("users")
        .eq("status", "ONLINE")
        .update(&User {
            status: "ONLINE".to_string(),
        })?
        .execute()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert_eq!(status.as_u16(), 200);
    assert_eq!(body[0]["status"], "ONLINE");

    Ok(())
}

#[tokio::test]
async fn delete() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .from("messages")
        .neq("username", "supabot")
        .delete()
        .execute()
        .await?;
    let status = resp.status();

    assert_eq!(status.as_u16(), 200);

    Ok(())
}

#[tokio::test]
async fn rpc() -> Result<(), Box<dyn Error>> {
    let client = Postgrest::new(REST_URL);
    let resp = client
        .rpc("get_status", r#"{"name_param": "leroyjenkins"}"#)
        .execute()
        .await?;
    let body = resp.text().await?;
    let body = json::parse(&body)?;

    assert!(body.is_null());

    Ok(())
}
