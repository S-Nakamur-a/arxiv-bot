#[derive(Serialize, Deserialize, Clone)]
enum TextType {
    #[serde(rename(serialize = "mrkdwn"))]
    Markdown,
    #[serde(rename(serialize = "plain_text"))]
    PlainText,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfirmDialog {
    title: TextObject,
    text: TextObject,
    confirm: TextObject,
    deny: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ButtonStyle>,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "lowercase"))]
enum ButtonStyle {
    Primary,
    Danger,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Button {
    text: TextObject,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ButtonStyle>,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Accessory {
    Button(Button)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TextObject {
    #[serde(rename(serialize = "type"))]
    block_text_type: TextType,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Block
{
    Section(Section),
    Actions(Actions),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    #[serde(rename(serialize = "type"))]
    section_type: String,
    text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<TextObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<Accessory>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Actions {
    #[serde(rename(serialize = "type"))]
    action_type: String,
    elements: Vec<Accessory>
}

impl Section {
    pub fn new() -> Self {
        Section {
            section_type: "section".to_string(),
            text: TextObject {
                block_text_type: TextType::Markdown,
                text: "".to_string(),
                emoji: None,
                verbatim: Some(true),
            },
            block_id: None,
            fields: None,
            accessory: None,
        }
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text.text = text.to_string();
        self
    }

    pub fn block_id(&mut self, block_id: &str) -> &mut Self {
        self.block_id = Some(block_id.to_string());
        self
    }

    pub fn button(&mut self, button: &Button) -> &mut Self {
        self.accessory = Some(Accessory::Button(button.clone()));
        self
    }
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            action_type: "actions".to_string(),
            elements: vec![]
        }
    }

    pub fn button(&mut self, button: &Button) -> &mut Self {
        self.elements.push(Accessory::Button(button.clone()));
        self
    }
}

impl Button {
    pub fn new() -> Self {
        Button{
            text: TextObject {
                block_text_type: TextType::PlainText,
                text: "".to_string(),
                emoji: None,
                verbatim: None
            },
            action_id: "".to_string(),
            url: None,
            value: None,
            style: None
        }
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text.text = text.to_string();
        self
    }

    pub fn emoji(&mut self, emoji: bool) -> &mut Self {
        self.text.emoji = Some(emoji);
        self
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn action_id(&mut self, action_id: &str) -> &mut Self {
        self.action_id = action_id.to_string();
        self
    }

    pub fn primary(&mut self) -> &mut Self {
        self.style = Some(ButtonStyle::Primary);
        self
    }

    pub fn danger(&mut self) -> &mut Self {
        self.style = Some(ButtonStyle::Danger);
        self
    }
}

#[test]
fn test_message() {
    let mut b = Section::new();
    let mut button = Button::new();
    button.url("https://www.google.com/").text(":ok_woman:").emoji(true).action_id("a");
    b.text("test text").button(&button);
    let mut a = Actions::new();
    let mut b1 = Button::new();
    b1.action_id("b").text("b");
    let mut b2 = Button::new();
    b2.action_id("c").text("c");
    a.button(&b1).button(&b2);
    println!("{}", json!({"blocks": [&b, &a]}));
}