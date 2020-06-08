//! # postgrest-rs
//!
//! [PostgREST][postgrest] client-side library.
//!
//! This library brings an ORM-like interface to PostgREST.
//!
//! ## Usage
//!
//! Simple example:
//! ```
//! use postgrest::Postgrest;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Postgrest::new("https://your-postgrest-endpoint");
//! let resp = client
//!     .from("your_table")
//!     .select("*")
//!     .execute()
//!     .await?;
//! let body = resp
//!     .text()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Using filters:
//! ```
//! # use postgrest::Postgrest;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Postgrest::new("https://your-postgrest-endpoint");
//! let resp = client
//!     .from("your_table")
//!     .eq("country", "Germany")
//!     .gte("id", "20")
//!     .select("*")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Updating a table:
//! ```
//! # use postgrest::Postgrest;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Postgrest::new("https://your-postgrest-endpoint");
//! let resp = client
//!     .from("your_table")
//!     .eq("username", "soedirgo")
//!     .update("{\"organization\": \"supabase\"}")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Executing stored procedures:
//! ```
//! # use postgrest::Postgrest;
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Postgrest::new("https://your-postgrest-endpoint");
//! let resp = client
//!     .rpc("add", "{\"a\": 1, \"b\": 2}")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Check out the [README][readme] for more examples.
//!
//! [postgrest]: https://postgrest.org
//! [readme]: https://github.com/supabase/postgrest-rs

extern crate reqwest;

mod builder;
mod filter;

pub use builder::Builder;

pub struct Postgrest {
    url: String,
    schema: Option<String>,
}

impl Postgrest {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Postgrest {
            url: url.into(),
            schema: None,
        }
    }

    pub fn schema<T>(mut self, schema: T) -> Self
    where
        T: Into<String>,
    {
        self.schema = Some(schema.into());
        self
    }

    pub fn from<T>(&self, table: T) -> Builder
    where
        T: Into<String>,
    {
        let url = format!("{}/{}", self.url, table.into());
        Builder::new(url, self.schema.clone())
    }

    pub fn rpc<T, U>(&self, function: T, params: U) -> Builder
    where
        T: Into<String>,
        U: Into<String>,
    {
        let url = format!("{}/rpc/{}", self.url, function.into());
        Builder::new(url, self.schema.clone()).rpc(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REST_URL: &str = "https://localhost:3000";

    #[test]
    fn initialize() {
        assert_eq!(Postgrest::new(REST_URL).url, REST_URL);
    }

    #[test]
    fn switch_schema() {
        assert_eq!(
            Postgrest::new(REST_URL).schema("private").schema,
            Some("private".to_string())
        );
    }
}
