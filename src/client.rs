use hyper_tls::{HttpsConnector};
use hyper::{Body, Method, Request, StatusCode, Uri};
use hyper::client::{Client, HttpConnector};
use hyper::body::Buf;

use std::str::FromStr;

use crate::models::{Message, Webhook};

pub type WebhookResult<Type> = std::result::Result<Type, Box<dyn std::error::Error + Send + Sync>>;

/// A Client that sends webhooks for discord.
pub struct WebhookClient {
    client: Client<HttpsConnector<HttpConnector>>,
    url: String
}

impl WebhookClient {

    pub fn new(url: &str) -> Self {
        let https_connector = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https_connector);
        Self {
            client,
            url: url.to_owned()
        }
    }

    /// Example
    /// ```rust
    /// let client = WebhookClient::new("URL");
    /// client.send(|message| message
    ///     .content("content")
    ///     .username("username")).await?;
    /// ```
    pub async fn send<Func>(&self, function: Func) -> WebhookResult<bool>
    where Func: Fn(&mut Message) -> &mut Message {
        let mut message = Message::new();
        function(&mut message);
        let result = self.send_message(&message).await?;

        Ok(result)
    }

    pub async fn send_message(&self, message: &Message) -> WebhookResult<bool> {
        let body = serde_json::to_string(message)?;
        let request = Request::builder()
            .method(Method::POST)
            .uri(&self.url)
            .header("content-type", "application/json")
            .body(Body::from(body))?;
        let response = self.client.request(request).await?;

        Ok(response.status() == StatusCode::OK)
    }

    pub async fn get_information(&self) -> WebhookResult<Webhook> {
        let response = self.client.get(Uri::from_str(&self.url)?).await?;
        let body = hyper::body::aggregate(response).await?;
        let webhook = serde_json::from_reader(body.reader())?;

        Ok(webhook)
    }

}