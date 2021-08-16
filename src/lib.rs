use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

type OString = Option<String>;
type OInt32 = Option<i32>;

pub struct Webhook {
    url: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
pub struct WebhookModel {
    pub name: String,
    #[serde(rename = "type")]
    pub webhook_type: i8,
    pub channel_id: String,
    pub token: Option<String>,
    pub avatar: Option<String>,
    pub guild_id: Option<String>,
    pub user: Option<WebhookUser>,
}

#[derive(Deserialize, Debug)]
pub struct WebhookUser {
    pub username: String,
    pub discriminator: String,
    pub id: String,
    pub avatar: String,
}

#[derive(Debug, Serialize)]
pub struct EmbedBuilder {
    title: OString,
    #[serde(rename = "type")]
    type_: OString,
    description: OString,
    url: OString,
    timestamp: OString,
    color: OInt32,
    fields: Vec<EmbedField>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    author: Option<EmbedAuthor>,
    video: Option<EmbedVideo>,
}

#[derive(Debug, Serialize)]
pub struct Embed {
    title: OString,
    #[serde(rename = "type")]
    type_: OString,
    description: OString,
    url: OString,
    timestamp: OString,
    color: OInt32,
    fields: Vec<EmbedField>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    author: Option<EmbedAuthor>,
    video: Option<EmbedVideo>,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            type_: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            fields: vec![],
            footer: None,
            image: None,
            thumbnail: None,
            author: None,
            video: None,
        }
    }

    pub fn title(&mut self, title: &str) -> &mut EmbedBuilder {
        self.title = Some(title.to_owned());
        self
    }

    pub fn type_(&mut self, type_: &str) -> &mut EmbedBuilder {
        self.type_ = Some(type_.to_owned());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut EmbedBuilder {
        self.description = Some(description.to_owned());
        self
    }

    pub fn url(&mut self, url: &str) -> &mut EmbedBuilder {
        self.url = Some(url.to_owned());
        self
    }

    pub fn color(&mut self, color: i32) -> &mut EmbedBuilder {
        self.color = Some(color);
        self
    }

    pub fn field(&mut self, name: &str, value: &str, inline: bool) -> &mut EmbedBuilder {
        self.fields.push(EmbedField::new(name, value, inline));
        self
    }

    pub fn timestamp(&mut self, timestamp: &str) -> &mut EmbedBuilder {
        self.timestamp = Some(timestamp.to_owned());
        self
    }

    pub fn footer(&mut self, text: &str, url: OString, proxy_url: OString) -> &mut EmbedBuilder {
        self.footer = Some(EmbedFooter::new(text, url, proxy_url));
        self
    }

    pub fn image(
        &mut self,
        url: &str,
        proxy_url: OString,
        height: OInt32,
        width: OInt32,
    ) -> &mut EmbedBuilder {
        self.image = Some(EmbedImage::new(url, proxy_url, height, width));
        self
    }

    pub fn thumbnail(
        &mut self,
        url: &str,
        proxy_url: OString,
        height: OInt32,
        width: OInt32,
    ) -> &mut EmbedBuilder {
        self.thumbnail = Some(EmbedThumbnail::new(url, proxy_url, height, width));
        self
    }

    pub fn author(
        &mut self,
        name: &str,
        url: &str,
        icon_url: OString,
        proxy_icon_url: OString,
    ) -> &mut EmbedBuilder {
        self.author = Some(EmbedAuthor::new(name, url, icon_url, proxy_icon_url));
        self
    }

    pub fn video(&mut self, url: &str, height: OInt32, width: OInt32) -> &mut EmbedBuilder {
        self.video = Some(EmbedVideo::new(url, height, width));
        self
    }

    pub fn build(&mut self) -> Embed {
        Embed {
            title: self.title.clone(),
            type_: self.type_.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            timestamp: self.timestamp.clone(),
            color: self.color.clone(),
            fields: self.fields.clone(),
            footer: self.footer.clone(),
            image: self.image.clone(),
            thumbnail: self.thumbnail.clone(),
            author: self.author.clone(),
            video: self.video.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

impl EmbedField {
    pub fn new(name: &str, value: &str, inline: bool) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            inline,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedFooter {
    text: String,
    icon_url: OString,
    proxy_icon_url: OString,
}

impl EmbedFooter {
    pub fn new(text: &str, icon_url: Option<String>, proxy_icon_url: Option<String>) -> Self {
        Self {
            text: text.to_owned(),
            icon_url,
            proxy_icon_url,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedImage {
    url: OString,
    proxy_url: OString,
    height: OInt32,
    width: OInt32,
}

impl EmbedImage {
    pub fn new(url: &str, proxy_url: OString, height: OInt32, width: OInt32) -> Self {
        Self {
            url: Some(url.to_owned()),
            proxy_url,
            height,
            width,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedThumbnail {
    url: OString,
    proxy_url: OString,
    height: OInt32,
    width: OInt32,
}

impl EmbedThumbnail {
    pub fn new(url: &str, proxy_url: OString, height: OInt32, width: OInt32) -> Self {
        Self {
            url: Some(url.to_owned()),
            proxy_url,
            height,
            width,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedAuthor {
    name: String,
    url: String,
    icon_url: OString,
    proxy_icon_url: OString,
}

impl EmbedAuthor {
    pub fn new(name: &str, url: &str, icon_url: OString, proxy_icon_url: OString) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned(),
            icon_url,
            proxy_icon_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedVideo {
    url: String,
    height: OInt32,
    width: OInt32,
}

impl EmbedVideo {
    pub fn new(url: &str, height: OInt32, width: OInt32) -> Self {
        Self {
            url: url.to_owned(),
            height,
            width,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Message {
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    embeds: Vec<Embed>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            tts: None,
            embeds: vec![],
        }
    }

    pub fn content(&mut self, content: &str) -> &mut Message {
        self.content = Some(content.to_owned());
        self
    }

    pub fn username(&mut self, name: &str) -> &mut Message {
        self.username = Some(name.to_owned());
        self
    }

    pub fn avatar_url(&mut self, url: &str) -> &mut Message {
        self.avatar_url = Some(url.to_owned());
        self
    }

    pub fn tts(&mut self, tts: bool) -> &mut Message {
        self.tts = Some(tts.to_owned());
        self
    }

    pub fn embed<F>(&mut self, embed: F) -> &mut Message
    where
        F: Fn(&mut EmbedBuilder) -> &mut EmbedBuilder,
    {
        let mut em = EmbedBuilder::new();
        let embed = embed(&mut em);
        self.embeds.push(embed.build());
        self
    }
}

impl Webhook {
    pub fn from_url(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            client: Client::new(),
        }
    }

    pub async fn get_info(&self) -> Result<WebhookModel, Box<dyn Error>> {
        let request = self.client.get(&self.url).send().await?;
        let content = request.text().await?;
        Ok(serde_json::from_str(&content)?)
    }

    pub async fn send<F>(&self, t: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&mut Message) -> &mut Message,
    {
        let mut msg = Message::new();
        let message = t(&mut msg);
        self.client.post(&self.url).json(&message).send().await?;
        Ok(())
    }
}
