mod builder;

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
