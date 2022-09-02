use webhook::client::{WebhookClient, WebhookResult};
use webhook::models::NonLinkButtonStyle;

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

// to try out using application webhook run:
// `application_webhook_example(&url).await?;`
async fn application_webhook_example(url: &str) -> WebhookResult<()> {
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

    client
        .send(|message| {
            message
                .username("Thoo")
                .avatar_url(IMAGE_URL)
                .action_row(|row| {
                    row.regular_button(|button| {
                        button
                            .style(NonLinkButtonStyle::Primary)
                            .label("Primary!")
                            .emoji("625891304081063986", "mage", false)
                            .custom_id("id_0")
                    })
                        .regular_button(|button| {
                            button
                                .style(NonLinkButtonStyle::Secondary)
                                .label("Secondary!")
                                .emoji("625891304081063986", "mage", false)
                                .custom_id("id_1")
                        })
                        .link_button(|button| button.label("Click Me!").url("https://discord.com"))
                })
        })
        .await?;

    Ok(())
}