use webhook::Webhook;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let url = std::env::args().skip(1).next().expect("pass webhook url as first argument");
    let webhook = Webhook::from_url(&url);
    webhook.send(|m| m.
        content("Test")
        .username("ThooOk")
        .avatar_url("https://cdn.discordapp.com/avatars/557337479523598340/e80851184b12242bd373ab5f6163c57b.png?size=128")
        .embed(|e| e.
            title("Test")
            .color(0x32a852)
            .field("Field1", "Value1", false)
            .field("Field2", "Value2", false)
            .video("https://cdn.discordapp.com/attachments/718702021766676512/733363175260422174/faggot.mp4", None, None)
            //.author("Author", "",Some("https://cdn.discordapp.com/avatars/557337479523598340/e80851184b12242bd373ab5f6163c57b.png?size=128".to_owned()), None)
            //.image("https://cdn.discordapp.com/avatars/557337479523598340/e80851184b12242bd373ab5f6163c57b.png?size=128", None, None, None)
            .footer("Footer", Some("https://cdn.discordapp.com/avatars/557337479523598340/e80851184b12242bd373ab5f6163c57b.png?size=128".to_owned()), None)
        )
    ).await?;
    Ok(())
}