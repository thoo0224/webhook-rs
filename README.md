<div align="center">

# webhook-rs

Discord Webhook API Wrapper

[![Downloads](https://img.shields.io/crates/d/webhook)](https://github.com/thoo0224/Gifski.Net/releases/latest) [![Docs](https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square)](https://docs.rs/webhook/latest/webhook/)
</div>

### Example usage
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

### Contribute
Any type of contribution is greatly appreciated.
