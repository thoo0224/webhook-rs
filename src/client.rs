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
    use crate::models::{ActionRow, DiscordApiCompatible, Embed, EmbedAuthor, EmbedField, EmbedFooter, Message, MessageContext, NonLinkButtonStyle};

    fn assert_message_error<BuildFunc, MessagePred>(
        message_build: BuildFunc,
        msg_pred: MessagePred,
    )
    where
        BuildFunc: Fn(&mut Message) -> &mut Message,
        MessagePred: Fn(&str) -> bool,
    {
        let mut message = Message::new();
        message_build(&mut message);
        match message.check_compatibility(&mut MessageContext::new()) {
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

    fn contains_all_predicate(needles: Vec<&str>) -> Box<dyn Fn(&str) -> bool> {
        let owned_needles: Vec<String> = needles.iter().map(|n| n.to_string()).collect();
        Box::new(move |haystack| {
            let lower_haystack = haystack.to_lowercase();
            owned_needles
                .iter()
                .all(|needle| lower_haystack.contains(needle))
        })
    }

    fn assert_valid_message<BuildFunc>(func: BuildFunc)
    where
        BuildFunc: Fn(&mut Message) -> &mut Message,
    {
        let mut message = Message::new();
        func(&mut message);
        if let Err(unexpected) = message.check_compatibility(&mut MessageContext::new()) {
            assert!(false, "Unexpected validation error {}", unexpected);
        }
    }

    #[test]
    fn empty_action_row_prohibited() {
        assert_message_error(
            |message| message.action_row(|row| row),
            contains_all_predicate(vec!["action row", "empty"]),
        );
    }

    #[test]
    fn send_message_custom_id_reuse_prohibited() {
        assert_message_error(
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
            contains_all_predicate(vec!["twice"]),
        );
    }

    #[test]
    fn send_message_custom_id_reuse_prohibited_across_action_rows() {
        assert_message_error(
            |message| {
                message
                    .action_row(|row| {
                        row.regular_button(|button| {
                            button.custom_id("0").style(NonLinkButtonStyle::Primary)
                        })
                    })
                    .action_row(|row| {
                        row.regular_button(|button| {
                            button.custom_id("0").style(NonLinkButtonStyle::Primary)
                        })
                    })
            },
            contains_all_predicate(vec!["twice"]),
        );
    }

    #[test] fn send_message_button_style_required() {
        assert_message_error(
            |message| message.action_row(|row| row.regular_button(|button| button.custom_id("0"))),
            contains_all_predicate(vec!["style"]),
        );
    }

    #[test]
    fn send_message_url_required() {
        assert_message_error(
            |message| message.action_row(|row| row.link_button(|button| button.label("test"))),
            contains_all_predicate(vec!["url"]),
        );
    }

    #[test]
    fn send_message_max_action_rows_enforced() {
        assert_message_error(
            |message| {
                for _ in 0..(Message::ACTION_ROW_COUNT_INTERVAL.max_allowed + 1) {
                    message.action_row(|row| row);
                }
                message
            },
            contains_all_predicate(vec!["interval", "row"]),
        );
    }

    #[test]
    fn send_message_max_label_len_enforced() {
        assert_message_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| {
                        btn.style(NonLinkButtonStyle::Primary)
                            .custom_id("a")
                            .label(&"l".repeat(Message::LABEL_LEN_INTERVAL.max_allowed + 1))
                    })
                })
            },
            contains_all_predicate(vec!["interval", "label"]),
        );
    }

    #[test]
    fn send_message_custom_id_required() {
        assert_message_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| btn.style(NonLinkButtonStyle::Primary))
                })
            },
            contains_all_predicate(vec!["custom id"]),
        );
    }

    #[test]
    fn send_message_max_custom_id_len_enforced() {
        assert_message_error(
            |message| {
                message.action_row(|row| {
                    row.regular_button(|btn| {
                        btn.style(NonLinkButtonStyle::Primary).custom_id(
                            &"a".repeat(Message::CUSTOM_ID_LEN_INTERVAL.max_allowed + 1),
                        )
                    })
                })
            },
            contains_all_predicate(vec!["interval", "custom id"]),
        );
    }

    #[test]
    fn max_button_count_enforced() {
        assert_message_error(
            |message| {
                message.action_row(|row| {
                    for i in 0..(ActionRow::BUTTON_COUNT_INTERVAL.max_allowed + 1) {
                        row.regular_button(|btn| {
                            btn.style(NonLinkButtonStyle::Primary)
                                .custom_id(&(i.to_string()))
                        });
                    }
                    row
                })
            },
            contains_all_predicate(vec!["interval", "button"]),
        );
    }

    #[test]
    fn max_button_count_enforced_only_per_action_row() {
        assert_valid_message(|message| {
            for i in 0..Message::ACTION_ROW_COUNT_INTERVAL.max_allowed {
                message.action_row(|row| {
                    for j in 0..(ActionRow::BUTTON_COUNT_INTERVAL.max_allowed) {
                        row.regular_button(|btn| {
                            btn.style(NonLinkButtonStyle::Primary)
                                .custom_id(&(i.to_string() + &j.to_string()))
                        });
                    }
                    row
                });
            }
            message
        });
    }

    #[test]
    fn message_valid_basic() {
        assert_valid_message(|message| {
            message
                .content("@test")
                .username("test")
                .avatar_url("test")
                .embed(|embed| {
                    embed
                        .title("test")
                        .description("test")
                        .footer("test", Some(String::from("test")))
                        .image("test")
                        .thumbnail("test")
                        .author(
                            "test",
                            Some(String::from("test")),
                            Some(String::from("test")),
                        )
                        .field("test", "test", false)
                })
        });
    }

    #[test]
    fn embed_title_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .title(&"a".repeat(Embed::TITLE_LEN_INTERVAL.max_allowed + 1))
                })
        },
     contains_all_predicate(vec!["interval", "embed", "title", "length"]),
        )
    }

    #[test]
    fn embed_description_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .description(&"a".repeat(Embed::DESCRIPTION_LEN_INTERVAL.max_allowed + 1))
                })
        },
     contains_all_predicate(vec!["interval", "embed", "description", "length"]),
        )
    }

    #[test]
    fn embed_author_name_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .author(&"a".repeat(EmbedAuthor::NAME_LEN_INTERVAL.max_allowed + 1), None, None)
                })
        },
         contains_all_predicate(vec!["interval", "embed", "author", "name", "length"]),
        )
    }

    #[test]
    fn embed_footer_text_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .footer(&"a".repeat(EmbedFooter::TEXT_LEN_INTERVAL.max_allowed + 1), None)
                })
        },
         contains_all_predicate(vec!["interval", "embed", "footer", "text", "length"]),
        )
    }

    #[test]
    fn embed_field_name_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .field(&"a".repeat(EmbedField::NAME_LEN_INTERVAL.max_allowed + 1), "None", false)
                })
        },
         contains_all_predicate(vec!["interval", "embed", "field", "name", "length"]),
        )
    }

    #[test]
    fn embed_field_value_len_enforced() {
        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .field("None", &"a".repeat(EmbedField::VALUE_LEN_INTERVAL.max_allowed + 1), false)
                })
        },
     contains_all_predicate(vec!["interval", "embed", "field", "value", "length"]),
        )
    }

    #[test]
    fn embed_total_char_length_enforced() {
        // adds 2 embeds with maximum length descriptions
        // which should overflow the maximum allowed characters for embeds in total
        assert!(Embed::DESCRIPTION_LEN_INTERVAL.max_allowed * 2 > Message::EMBED_TOTAL_TEXT_LEN_INTERVAL.max_allowed, "Key test values modified, fix this test!");

        assert_message_error(|message| {
            message
                .embed(|embed| {
                    embed
                        .description(&"a".repeat(Embed::DESCRIPTION_LEN_INTERVAL.max_allowed))
                })
                .embed(|embed| {
                    embed
                        .description(&"a".repeat(Embed::DESCRIPTION_LEN_INTERVAL.max_allowed))
                })
        },
         contains_all_predicate(vec!["interval", "character", "count", "embed"]),
        )
    }

    #[test]
    #[should_panic]
    fn field_count_enforced() {
        assert_valid_message(|message| {
            message
                .embed(|embed| {
                    for _ in 0..Embed::FIELDS_LEN_INTERVAL.max_allowed + 1 {
                        embed.field("None", "a", false);
                    }
                    embed
                })
        })
    }

    fn test_is_send<T>(t: T)
    where
        T: Send,
    {
        drop(t);
    }

    #[test]
    fn message_is_send() {
        let message = Message::new();
        // this should not compile if Message is not Send
        test_is_send(message);
    }
}
