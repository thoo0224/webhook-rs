<div align="center">

# webhook-rs

Discord Webhook API Wrapper

[![Downloads](https://img.shields.io/crates/d/webhook)](https://github.com/thoo0224/Gifski.Net/releases/latest) [![Docs](https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square)](https://docs.rs/webhook/latest/webhook/)
</div>

### Example usage
For a full example, take a look at `examples/example.rs`.
```rust
let client = WebhookClient::new(&url);
client.send(|message| message
  .content("test")
  .username("Thoo")
  .avatar_url("https://cdn.discordapp.com/avatars/312157715449249795/a_b8b3b0c35f3dee2b6586a0dd58697e29.png")
  .embed(|embed| embed
    .title("test")
    .description("o hey men"))).await?;
```

### To do
- Attachments
- Components

### Contribute
Any type of contribution is greatly appreciated.
