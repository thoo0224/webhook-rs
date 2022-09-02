use serde::{Deserialize, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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
    pub application_id: Option<Snowflake>,
}

#[derive(Debug)]
struct MessageContext {
    custom_ids: HashSet<String>,
    errors: Vec<String>,
}

impl MessageContext {
    /// Tries to register a custom id.
    ///
    /// # Arguments
    ///
    /// * `id`: the custom id to be registered
    ///
    ///
    /// # Return value
    /// Returns true if the custom id is unique.
    ///
    /// Returns false if the supplied custom id is duplicate of an already registered custom id.
    pub fn register_custom_id(&mut self, id: &str) -> bool {
        self.custom_ids.insert(id.to_string())
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn get_error(&mut self, index: usize) -> Option<&String> {
        self.errors.get(index)
    }

    pub fn new() -> MessageContext {
        MessageContext {
            custom_ids: HashSet::new(),
            errors: vec![],
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub content: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: bool,
    pub embeds: Vec<Embed>,
    pub allow_mentions: Option<AllowedMentions>,
    #[serde(rename = "components")]
    pub action_rows: Vec<ActionRow>,
    #[serde(skip_serializing)]
    context: Rc<RefCell<MessageContext>>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            tts: false,
            embeds: vec![],
            allow_mentions: None,
            action_rows: vec![],
            context: Rc::new(RefCell::new(MessageContext::new())),
        }
    }
    pub(crate) fn first_error_message(&self) -> Option<String> {
        self.context
            .borrow_mut()
            .get_error(0)
            .map(|s| s.to_string())
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
    where
        Func: Fn(&mut Embed) -> &mut Embed,
    {
        let mut embed = Embed::new();
        func(&mut embed);
        self.embeds.push(embed);

        self
    }

    pub fn action_row<Func>(&mut self, func: Func) -> &mut Self
    where
        Func: Fn(&mut ActionRow) -> &mut ActionRow,
    {
        let mut row = ActionRow::new(&self.context);
        if self.action_rows.len() > Self::max_action_row_count() - 1 {
            self.context.borrow_mut().add_error(&format!(
                "Action row count exceeded {} (maximum)",
                Self::max_action_row_count()
            ));
            return self;
        }

        func(&mut row);
        self.action_rows.push(row);

        self
    }

    pub fn max_action_row_count() -> usize {
        5
    }

    pub fn label_max_len() -> usize {
        80
    }

    pub fn custom_id_max_len() -> usize {
        100
    }

    pub fn allow_mentions(
        &mut self,
        parse: Option<Vec<AllowedMention>>,
        roles: Option<Vec<Snowflake>>,
        users: Option<Vec<Snowflake>>,
        replied_user: bool,
    ) -> &mut Self {
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
    // ISO8601,
    pub timestamp: Option<String>,
    pub color: Option<String>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub video: Option<EmbedVideo>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>,
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
            fields: vec![],
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

    pub fn author(
        &mut self,
        name: &str,
        url: Option<String>,
        icon_url: Option<String>,
    ) -> &mut Self {
        self.author = Some(EmbedAuthor::new(name, url, icon_url));
        self
    }

    pub fn field(&mut self, name: &str, value: &str, inline: bool) -> &mut Self {
        if self.fields.len() == 25 {
            panic!("You can't have more than 25 fields in an embed!")
        }

        self.fields.push(EmbedField::new(name, value, inline));
        self
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
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
            url: url.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedProvider {
    pub name: String,
    pub url: String,
}

impl EmbedProvider {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

impl EmbedAuthor {
    pub fn new(name: &str, url: Option<String>, icon_url: Option<String>) -> Self {
        Self {
            name: name.to_owned(),
            url,
            icon_url,
        }
    }
}

pub enum AllowedMention {
    RoleMention,
    UserMention,
    EveryoneMention,
}

fn resolve_allowed_mention_name(allowed_mention: AllowedMention) -> String {
    match allowed_mention {
        AllowedMention::RoleMention => "roles".to_string(),
        AllowedMention::UserMention => "users".to_string(),
        AllowedMention::EveryoneMention => "everyone".to_string(),
    }
}

#[derive(Serialize, Debug)]
pub struct AllowedMentions {
    pub parse: Option<Vec<String>>,
    pub roles: Option<Vec<Snowflake>>,
    pub users: Option<Vec<Snowflake>>,
    pub replied_user: bool,
}

impl AllowedMentions {
    pub fn new(
        parse: Option<Vec<AllowedMention>>,
        roles: Option<Vec<Snowflake>>,
        users: Option<Vec<Snowflake>>,
        replied_user: bool,
    ) -> Self {
        let mut parse_strings: Vec<String> = vec![];
        if parse.is_some() {
            parse
                .unwrap()
                .into_iter()
                .for_each(|x| parse_strings.push(resolve_allowed_mention_name(x)))
        }

        Self {
            parse: Some(parse_strings),
            roles,
            users,
            replied_user,
        }
    }
}

// ready to be extended with other components
// non-composite here specifically means *not an action row*
#[derive(Debug)]
enum NonCompositeComponent {
    Button(Button),
}

impl Serialize for NonCompositeComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NonCompositeComponent::Button(button) => button.serialize(serializer),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ActionRow {
    #[serde(rename = "type")]
    pub component_type: u8,
    components: Vec<NonCompositeComponent>,
    #[serde(skip_serializing)]
    context: Rc<RefCell<MessageContext>>,
}

impl ActionRow {
    fn new(context: &Rc<RefCell<MessageContext>>) -> ActionRow {
        ActionRow {
            component_type: 1,
            components: vec![],
            context: Rc::clone(context),
        }
    }

    pub fn link_button<Func>(&mut self, button_mutator: Func) -> &mut Self
    where
        Func: Fn(&mut LinkButton) -> &mut LinkButton,
    {
        let mut button = LinkButton::new(&self.context);
        button_mutator(&mut button);
        if let Some(b) = button.create_button() {
            self.components.push(NonCompositeComponent::Button(b));
        }

        self
    }

    pub fn regular_button<Func>(&mut self, button_mutator: Func) -> &mut Self
    where
        Func: Fn(&mut RegularButton) -> &mut RegularButton,
    {
        let mut button = RegularButton::new(&self.context);
        button_mutator(&mut button);
        if let Some(b) = button.create_button() {
            self.components.push(NonCompositeComponent::Button(b));
        }
        self
    }
}

#[derive(Debug, Clone)]
pub enum NonLinkButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
}

impl NonLinkButtonStyle {
    fn get_button_style(&self) -> ButtonStyles {
        match *self {
            NonLinkButtonStyle::Primary => ButtonStyles::Primary,
            NonLinkButtonStyle::Secondary => ButtonStyles::Secondary,
            NonLinkButtonStyle::Success => ButtonStyles::Success,
            NonLinkButtonStyle::Danger => ButtonStyles::Danger,
        }
    }
}

// since link button has an explicit way of creation via the action row
// this enum is kept hidden from the user ans the NonLinkButtonStyle is created to avoid
// user confusion
#[derive(Debug)]
enum ButtonStyles {
    Primary,
    Secondary,
    Success,
    Danger,
    Link,
}

impl ButtonStyles {
    /// value for serialization purposes
    fn value(&self) -> i32 {
        match *self {
            ButtonStyles::Primary => 1,
            ButtonStyles::Secondary => 2,
            ButtonStyles::Success => 3,
            ButtonStyles::Danger => 4,
            ButtonStyles::Link => 5,
        }
    }
}

impl Serialize for ButtonStyles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.value())
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PartialEmoji {
    pub id: Snowflake,
    pub name: String,
    pub animated: Option<bool>,
}

/// the button struct intended for serialized
#[derive(Serialize, Debug)]
struct Button {
    #[serde(rename = "type")]
    pub component_type: i8,
    pub style: ButtonStyles,
    pub label: Option<String>,
    pub emoji: Option<PartialEmoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: Option<bool>,

    #[serde(skip_serializing)]
    #[allow(dead_code)]
    context: Rc<RefCell<MessageContext>>,
}

impl Button {
    /// creates a link button
    fn new_link(
        label: Option<String>,
        emoji: Option<PartialEmoji>,
        url: Option<String>,
        disabled: Option<bool>,
        context: &Rc<RefCell<MessageContext>>,
    ) -> Self {
        Self {
            component_type: 2,
            style: ButtonStyles::Link,
            label,
            emoji,
            custom_id: None,
            url,
            disabled,
            context: Rc::clone(context),
        }
    }

    /// creates a regular button
    fn new_regular(
        style: NonLinkButtonStyle,
        label: Option<String>,
        emoji: Option<PartialEmoji>,
        custom_id: String,
        disabled: Option<bool>,
        context: &Rc<RefCell<MessageContext>>,
    ) -> Self {
        Self {
            component_type: 2,
            style: style.get_button_style(),
            label,
            emoji,
            custom_id: Some(custom_id),
            url: None,
            disabled,
            context: Rc::clone(context),
        }
    }
}

/// Data holder for shared fields of link and regular buttons
#[derive(Debug)]
struct ButtonCommonBase {
    pub label: Option<String>,
    pub emoji: Option<PartialEmoji>,
    pub disabled: Option<bool>,
    context: Rc<RefCell<MessageContext>>,
}

impl ButtonCommonBase {
    fn new(
        label: Option<String>,
        emoji: Option<PartialEmoji>,
        disabled: Option<bool>,
        context: &Rc<RefCell<MessageContext>>,
    ) -> Self {
        ButtonCommonBase {
            label,
            emoji,
            disabled,
            context: Rc::clone(context),
        }
    }
    fn label(&mut self, label: &str) -> &mut Self {
        if label.len() > Message::label_max_len() {
            self.context.borrow_mut().add_error(&format!(
                "Label length exceeds {} characters",
                Message::label_max_len()
            ));
            return self;
        }
        self.label = Some(label.to_string());
        self
    }

    fn emoji(&mut self, emoji_id: Snowflake, name: &str, animated: bool) -> &mut Self {
        self.emoji = Some(PartialEmoji {
            id: emoji_id,
            name: name.to_string(),
            animated: Some(animated),
        });
        self
    }

    fn disabled(&mut self, disabled: bool) -> &mut Self {
        self.disabled = Some(disabled);
        self
    }
}

/// a macro which takes an identifier (`base`) of the ButtonCommonBase (relative to `self`)
/// and generates setter functions that delegate their inputs to the `self.base`
macro_rules! button_base_delegation {
    ($base:ident) => {
        pub fn emoji(&mut self, emoji_id: &str, name: &str, animated: bool) -> &mut Self {
            self.$base.emoji(emoji_id.to_string(), name, animated);
            self
        }

        pub fn disabled(&mut self, disabled: bool) -> &mut Self {
            self.$base.disabled(disabled);
            self
        }

        pub fn label(&mut self, label: &str) -> &mut Self {
            self.$base.label(label);
            self
        }
    };
}

#[derive(Debug)]
pub struct LinkButton {
    button_base: ButtonCommonBase,
    url: Option<String>,
}

impl LinkButton {
    fn new(context: &Rc<RefCell<MessageContext>>) -> Self {
        LinkButton {
            button_base: ButtonCommonBase::new(None, None, None, context),
            url: None,
        }
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    button_base_delegation!(button_base);
}

pub struct RegularButton {
    button_base: ButtonCommonBase,
    custom_id: Option<String>,
    style: Option<NonLinkButtonStyle>,
}

impl RegularButton {
    fn new(context: &Rc<RefCell<MessageContext>>) -> Self {
        RegularButton {
            button_base: ButtonCommonBase::new(None, None, None, context),
            custom_id: None,
            style: None,
        }
    }

    pub fn custom_id(&mut self, custom_id: &str) -> &mut Self {
        if custom_id.len() > Message::custom_id_max_len() {
            self.button_base.context.borrow_mut().add_error(&format!(
                "Custom ID length exceeds {} characters",
                Message::custom_id_max_len()
            ));
            return self;
        }
        if !self
            .button_base
            .context
            .borrow_mut()
            .register_custom_id(custom_id)
        {
            self.button_base.context.borrow_mut().add_error(&format!(
                "Attempt to use the same custom ID ({}) twice! (buttonLabel: {})",
                custom_id,
                match self.button_base.label.as_ref() {
                    Some(label) => label,
                    None => "Label not set",
                }
            ));
            return self;
        }

        self.custom_id = Some(custom_id.to_string());
        self
    }

    pub fn style(&mut self, style: NonLinkButtonStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    button_base_delegation!(button_base);
}

trait ButtonConstructor {
    fn create_button(&self) -> Option<Button>;
}

impl ButtonConstructor for LinkButton {
    fn create_button(&self) -> Option<Button> {
        if self.url.is_none() {
            self.button_base
                .context
                .borrow_mut()
                .add_error("Url of a Link button must be set!");
            return None;
        }

        Some(Button::new_link(
            self.button_base.label.clone(),
            self.button_base.emoji.clone(),
            self.url.clone(),
            self.button_base.disabled,
            &self.button_base.context,
        ))
    }
}

impl ButtonConstructor for RegularButton {
    fn create_button(&self) -> Option<Button> {
        if self.style.is_none() {
            self.button_base
                .context
                .borrow_mut()
                .add_error("Button style of a NonLink button must be set!");
            return None;
        }
        if self.custom_id.is_none() {
            self.button_base
                .context
                .borrow_mut()
                .add_error("Custom ID of a NonLink button must be set!");
            return None;
        }

        Some(Button::new_regular(
            self.style.as_ref().unwrap().clone(),
            self.button_base.label.clone(),
            self.button_base.emoji.clone(),
            self.custom_id.as_ref().unwrap().clone(),
            self.button_base.disabled,
            &self.button_base.context,
        ))
    }
}
