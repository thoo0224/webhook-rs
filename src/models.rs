use serde::{Deserialize, Serialize};

type Snowflake = String;

#[derive(Deserialize, Debug)]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub webhook_type: i8,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub token: String,
    pub application_id: Option<Snowflake>
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub content: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: bool,
    pub embeds: Vec<Embed>,
    pub allow_mentions: Option<AllowedMentions>
}

impl Message {

    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            tts: false,
            embeds: vec![],
            allow_mentions: None
        }
    }

    pub fn content(&mut self, content: &str) -> &mut Self {
        self.content = Some(content.to_owned());
        self
    }

    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_owned());
        self
    }

    pub fn avatar_url(&mut self, avatar_url: &str) -> &mut Self {
        self.avatar_url = Some(avatar_url.to_owned());
        self
    }

    pub fn tts(&mut self, tts: bool) -> &mut Self {
        self.tts = tts;
        self
    }

    pub fn embed<Func>(&mut self, func: Func) -> &mut Self
    where Func: Fn(&mut Embed) -> &mut Embed {
        let mut embed = Embed::new();
        func(&mut embed);
        self.embeds.push(embed);

        self
    }

    pub fn allow_mentions(&mut self,
                          parse: Option<Vec<AllowedMention>>,
                          roles: Option<Vec<Snowflake>>,
                          users: Option<Vec<Snowflake>>,
                          replied_user: bool) -> &mut Self {
        self.allow_mentions = Some(AllowedMentions::new(parse, roles, users, replied_user));
        self
    }

}

#[derive(Serialize, Debug)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    embed_type: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>, // ISO8601,
    pub color: Option<String>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub video: Option<EmbedVideo>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>
}

impl Embed {

    pub fn new() -> Self {
        Self {
            title: None,
            embed_type: String::from("rich"),
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            video: None,
            thumbnail: None,
            provider: None,
            author: None,
            fields: vec![]
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn timestamp(&mut self, timestamp: &str) -> &mut Self {
        self.timestamp = Some(timestamp.to_owned());
        self
    }

    pub fn color(&mut self, color: &str) -> &mut Self {
        self.color = Some(color.to_owned());
        self
    }

    pub fn footer(&mut self, text: &str, icon_url: Option<String>) -> &mut Self {
        self.footer = Some(EmbedFooter::new(text, icon_url));
        self
    }

    pub fn image(&mut self, url: &str) -> &mut Self {
        self.image = Some(EmbedImage::new(url));
        self
    }

    pub fn video(&mut self, url: &str) -> &mut Self {
        self.video = Some(EmbedVideo::new(url));
        self
    }

    pub fn thumbnail(&mut self, url: &str) -> &mut Self {
        self.thumbnail = Some(EmbedThumbnail::new(url));
        self
    }

    pub fn provider(&mut self, name: &str, url: &str) -> &mut Self {
        self.provider = Some(EmbedProvider::new(name, url));
        self
    }

    pub fn author(&mut self, name: &str, url: Option<String>, icon_url: Option<String>) -> &mut Self {
        self.author = Some(EmbedAuthor::new(name, url, icon_url));
        self
    }

    pub fn field(&mut self, name: &str, value: &str, inline: bool) -> &mut Self {
        if self.fields.len() == 10 {
            panic!("You can't have more than")
        }

        self.fields.push(EmbedField::new(name, value, inline));
        self
    }

}

#[derive(Serialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool
}

impl EmbedField {
    pub fn new(name: &str, value: &str, inline: bool) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            inline
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

impl EmbedFooter {
    pub fn new(text: &str, icon_url: Option<String>) -> Self {
        Self {
            text: text.to_owned(),
            icon_url,
        }
    }
}

pub type EmbedImage = EmbedUrlSource;
pub type EmbedThumbnail = EmbedUrlSource;
pub type EmbedVideo = EmbedUrlSource;

#[derive(Serialize, Debug)]
pub struct EmbedUrlSource {
    pub url: String,
}

impl EmbedUrlSource {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned()
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedProvider {
    pub name: String,
    pub url: String
}

impl EmbedProvider {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned()
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>
}

impl EmbedAuthor {
    pub fn new(name: &str, url: Option<String>, icon_url: Option<String>) -> Self {
        Self {
            name: name.to_owned(),
            url,
            icon_url
        }
    }
}

pub enum AllowedMention {
    RoleMention,
    UserMention,
    EveryoneMention
}

fn resolve_allowed_mention_name(allowed_mention: AllowedMention) -> String {
    match allowed_mention {
        AllowedMention::RoleMention => "roles".to_string(),
        AllowedMention::UserMention => "users".to_string(),
        AllowedMention::EveryoneMention => "everyone".to_string()
    }
}

#[derive(Serialize, Debug)]
pub struct AllowedMentions {
    pub parse: Option<Vec<String>>,
    pub roles: Option<Vec<Snowflake>>,
    pub users: Option<Vec<Snowflake>>,
    pub replied_user: bool
}

impl AllowedMentions {
    pub fn new(parse: Option<Vec<AllowedMention>>,
               roles: Option<Vec<Snowflake>>,
               users: Option<Vec<Snowflake>>,
               replied_user: bool) -> Self {
        let mut parse_strings: Vec<String> = vec![];
        if parse.is_some() {
            parse.unwrap().into_iter().for_each(|x| {
                parse_strings.push(resolve_allowed_mention_name(x))
            })
        }

        Self {
            parse: Some(parse_strings),
            roles,
            users,
            replied_user
        }
    }
}