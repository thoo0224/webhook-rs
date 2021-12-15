use webhook::client::{WebhookClient, WebhookResult};

const IMAGE_URL: &'static str = "https://cdn.discordapp.com/avatars/312157715449249795/a_b8b3b0c35f3dee2b6586a0dd58697e29.png";

#[tokio::main]
async fn main() -> WebhookResult<()> {
    dotenv::dotenv()?; // Load the .env file (excluded from the repository)

    let url = dotenv::var("URL")?;
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

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

    Ok(())
}