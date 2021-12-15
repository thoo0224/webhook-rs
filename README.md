<div align="center">

# webhook-rs

Discord Webhook API Wrapper

[![Downloads](https://img.shields.io/crates/d/webhook)](https://github.com/thoo0224/webhook-rs/releases/latest) [![Docs](https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square)](https://docs.rs/webhook/latest/webhook/)
</div>

### Example usage
For a full example, take a look at `examples/example.rs`.
```rust
let url: &str = "Webhook URL";
let client: WebhookClient = WebhookClient::new(URL);
client.send(|message| message
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

### Get started
To get started, simply add the crate to your `Cargo.toml`.

```toml
[dependencies]
webhook = "1.0.0"
```

If you only want the types, you can get rid of the networking-related
dependencies by using the feature `models`.

```toml
[dependencies]
webhook = { version = "1.0.0", features = ["models"] }
```

### To do
- Attachments
- Components

### Contribute
Any type of contribution is greatly appreciated.
