use webhook::client::{WebhookClient, WebhookResult};

#[tokio::main]
async fn main() -> WebhookResult<()> {
    dotenv::dotenv()?; // Load the .env file (excluded from the repository)

    let url = dotenv::var("URL")?;
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

    client.send(|message| message
        .content("test")
        .username("Thoo")
        .avatar_url("https://cdn.discordapp.com/avatars/312157715449249795/a_b8b3b0c35f3dee2b6586a0dd58697e29.png")
        .embed(|embed| embed
            .title("test")
            .description("o hey men"))).await?;

    Ok(())
}