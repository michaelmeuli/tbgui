use async_ssh2_tokio::client::Client;
use iced::widget::{checkbox, text};
use iced::window;
use iced::{Element, Length};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct State {
    pub screen: Screen,
    pub filter: Filter,
    pub items: Vec<Item>,
    pub client: Option<Client>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<RemoteState, LoadError>),
    FilterChanged(Filter),
    ItemMessage(usize, ItemMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
    RunTbProfiler,
    ProfilerRunCompleted,
    DownloadResults,
    DeleteResults,
    SettingsPressed,
    HomePressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Uuid,
    pub sample: String,
    pub is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    CheckboxToggled(bool),
}

impl Item {
    pub fn update(&mut self, message: ItemMessage) {
        match message {
            ItemMessage::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
        }
    }

    pub fn view(&self) -> Element<ItemMessage> {
        checkbox(&self.sample, self.is_checked)
            .on_toggle(ItemMessage::CheckboxToggled)
            .width(Length::Fill)
            .size(17)
            .text_shaping(text::Shaping::Advanced)
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Unchecked,
    Checked,
}

impl Filter {
    pub fn matches(self, item: &Item) -> bool {
        match self {
            Filter::All => true,
            Filter::Unchecked => !item.is_checked,
            Filter::Checked => item.is_checked,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadError {
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct RemoteState {
    pub client: Client,
    pub items: Vec<Item>,
}