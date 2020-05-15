use builder::Builder;

mod builder;

pub struct Postgrest {
    rest_url: String,
}

impl Postgrest {
    pub fn new(rest_url: &str) -> Postgrest {
        Postgrest {
            rest_url: rest_url.to_string(),
        }
    }

    pub fn from(&self, table: &str) -> Builder {
        let mut url = self.rest_url.clone();
        url = format!("{}/{}", url, table);
        Builder::new(&url)
    }
}
