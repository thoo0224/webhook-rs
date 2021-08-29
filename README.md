# Webhook
[![Crates](https://img.shields.io/badge/crates.io-Webhook-brightgreen.svg)](https://crates.io/crates/webhook)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Use
To use this crate add this to your `Cargo.toml`
#### `webhook = "1.0.0"`

## Example
```rust
use webhook::Webhook;
use std::error::Error;

#[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let webhook = Webhook::from_url("{WEBHOOK URL}");
    webhook.send(|message| { message.
        content("Message")
        .tts(true)
        .username("{USERNAME}")
        .avatar_url("{AVATAR_URL}")
        .embed(|embed| embed. // You can have up to 10 embeds
            title("{TITLE}")
            .color({COLOR}) // Example: 0x32a852
            .field("{FIELD}", "{VALUE}", {INLINE} (bool)) // You can add multiple fields
            .field("{FIELD}", "{VALUE}", {INLINE} (bool))
            .video("{URL}", None, None)
            .image("{URL}", {HEIGHT}, {WIDTH})
            .author("{NAME}", "{URL}", "{ICON_URL}", None)
            .footer("{NAME}", "{ICON_URl}")
        )
    }).await?;
    Ok(())
}
```
