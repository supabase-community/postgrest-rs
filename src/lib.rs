mod builder;
mod filter;

use builder::Builder;

pub struct Postgrest {
    url: String,
    schema: Option<String>,
}

impl Postgrest {
    pub fn new(url: &str) -> Self {
        Postgrest {
            url: url.to_string(),
            schema: None,
        }
    }

    pub fn schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    pub fn from(&self, table: &str) -> Builder {
        let url = format!("{}/{}", self.url, table);
        Builder::new(&url, self.schema.clone())
    }

    pub fn rpc(&self, function: &str, params: &str) -> Builder {
        let url = format!("{}/rpc/{}", self.url, function);
        Builder::new(&url, self.schema.clone()).rpc(params)
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
