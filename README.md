<div align="center">

# webhook-rs

Discord Webhook API Wrapper

[![Downloads](https://img.shields.io/crates/d/webhook)](https://github.com/thoo0224/Gifski.Net/releases/latest) [![Docs](https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square)](https://docs.rs/webhook/latest/webhook/)
</div>

### Example usage
For a full example, take a look at `examples/example.rs`.
```rust
let url: &str = "Webhook URL";
let client: WebhookClient = WebhookClient::new(URL);
client.send(|message| message
    .content("@everyone")
    .username("Thoo")
    .avatar_url(IMAGE_URL)
    .embed(|embed| embed
        .title("Webhook")
        .description("Hello, World!")
        .footer("Footer", Some(String::from(IMAGE_URL)))
        .image(IMAGE_URL)
        .thumbnail(IMAGE_URL)
        .author("Lmao#0001", Some(String::from(IMAGE_URL)), Some(String::from(IMAGE_URL)))
        .field("name", "value", false))).await?;
```

### To do
- Attachments
- Components

### Contribute
Any type of contribution is greatly appreciated.
