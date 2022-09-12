use hyper::body::Buf;
use hyper::client::{Client, HttpConnector};
use hyper::{Body, Method, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;

use std::str::FromStr;

use crate::models::{DiscordApiCompatible, Message, MessageContext, Webhook};

pub type WebhookResult<Type> = std::result::Result<Type, Box<dyn std::error::Error + Send + Sync>>;

/// A Client that sends webhooks for discord.
pub struct WebhookClient {
    client: Client<HttpsConnector<HttpConnector>>,
    url: String,
}

impl WebhookClient {
    pub fn new(url: &str) -> Self {
        let https_connector = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https_connector);
        Self {
            client,
            url: url.to_owned(),
        }
    }

    /// Example
    /// ```ignore
    /// let client = WebhookClient::new("URL");
    /// client.send(|message| message
    ///     .content("content")
    ///     .username("username")).await?;
    /// ```
    pub async fn send<Func>(&self, function: Func) -> WebhookResult<bool>
    where
        Func: Fn(&mut Message) -> &mut Message,
    {
        let mut message = Message::new();
        function(&mut message);
        let mut message_context = MessageContext::new();
        match message.check_compatibility(&mut message_context) {
            Ok(_) => (),
            Err(error_message) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    error_message,
                )));
            }
        };
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

        // https://discord.com/developers/docs/resources/webhook#execute-webhook
        // execute webhook returns either NO_CONTENT or a message
        if response.status() == StatusCode::NO_CONTENT {
            Ok(true)
        } else {
            let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
            let err_msg = match String::from_utf8(body_bytes.to_vec()) {
                Ok(msg) => msg,
                Err(err) => {
                    "Error reading Discord API error message:".to_string() + &err.to_string()
                }
            };

            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                err_msg,
            )))
        }
    }

    pub async fn get_information(&self) -> WebhookResult<Webhook> {
        let response = self.client.get(Uri::from_str(&self.url)?).await?;
        let body = hyper::body::aggregate(response).await?;
        let webhook = serde_json::from_reader(body.reader())?;

        Ok(webhook)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::WebhookClient;
    use crate::models::{Message, NonLinkButtonStyle};

    async fn assert_client_error<BuildFunc, MessagePred>(
        message_build: BuildFunc,
        msg_pred: MessagePred,
    ) -> ()
    where
        BuildFunc: Fn(&mut Message) -> &mut Message,
        MessagePred: Fn(&str) -> bool,
    {
        let client = WebhookClient::new("https://discord.com");
        let result = client.send(message_build).await;
        match result {
            Err(err) => {
                assert!(
                    msg_pred(&err.to_string()),
                    "Unexpected error message {}",
                    err.to_string()
                )
            }
            Ok(_) => assert!(false, "Error is expected"),
        };
    }

    #[tokio::test]
    async fn send_message_custom_id_reuse_prohibited() {
        assert_client_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|button| {
                        button.custom_id("0").style(NonLinkButtonStyle::Primary)
                    })
                    .regular_button(|button| {
                        button.custom_id("0").style(NonLinkButtonStyle::Primary)
                    })
                })
            },
            |err_msg| err_msg.to_lowercase().contains("twice"),
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_button_style_required() {
        assert_client_error(
            |message| message.action_row(|row| row.regular_button(|button| button.custom_id("0"))),
            |err_msg| err_msg.to_lowercase().contains("style"),
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_url_required() {
        assert_client_error(
            |message| message.action_row(|row| row.link_button(|button| button.label("test"))),
            |err_msg| err_msg.to_lowercase().contains("url"),
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_max_action_rows_enforced() {
        assert_client_error(
            |message| {
                for _ in 0..(Message::max_action_row_count() + 1) {
                    message.action_row(|row| row);
                }
                message
            },
            |err_msg| {
                err_msg.to_lowercase().contains("exceed") && err_msg.to_lowercase().contains("row")
            },
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_max_label_len_enforced() {
        assert_client_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| {
                        btn.style(NonLinkButtonStyle::Primary)
                            .custom_id("a")
                            .label(&"l".repeat(Message::label_max_len() + 1))
                    })
                })
            },
            |err_msg| {
                err_msg.to_lowercase().contains("exceed")
                    && err_msg.to_lowercase().contains("label")
            },
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_custom_id_required() {
        assert_client_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| btn.style(NonLinkButtonStyle::Primary))
                })
            },
            |err_msg| err_msg.to_lowercase().contains("custom id"),
        )
        .await;
    }

    #[tokio::test]
    async fn send_message_max_custom_id_len_enforced() {
        assert_client_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| {
                        btn.style(NonLinkButtonStyle::Primary)
                            .custom_id(&"a".repeat(Message::custom_id_max_len() + 1))
                    })
                })
            },
            |err_msg| {
                err_msg.to_lowercase().contains("exceed")
                    && err_msg.to_lowercase().contains("custom id")
            },
        )
        .await;
    }
}
