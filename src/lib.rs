use reqwest::Method;

pub struct Request {
    method: Option<Method>,
    url: String,
}

impl Request {
    pub fn new(url: &str) -> Request {
        Request {
            method: None,
            url: url.to_owned(),
        }
    }

    pub fn select(mut self, column_query: &str) -> Request {
        self.method = Some(Method::GET);
        self.url.push_str(&format!("?select={}", column_query));
        self
    }

    pub async fn execute(self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get(&self.url)
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}

pub struct PostgrestClient {
    rest_url: String,
}

impl PostgrestClient {
    pub fn new(rest_url: &str) -> PostgrestClient {
        PostgrestClient {
            rest_url: rest_url.to_owned(),
        }
    }

    pub fn from(&self, table: &str) -> Request {
        let mut url = self.rest_url.clone();
        url.push('/');
        url.push_str(table);
        Request::new(&url)
    }
}
