//mod utils;
//use utils::*;

use iced::keyboard;
use iced::widget::{
    self, button, center, checkbox, column, container, keyed_column, row, scrollable, text,
    text_input,
};
use iced::window;
use iced::{Center, Element, Fill, Subscription, Task};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application(Tbgui::title, Tbgui::update, Tbgui::view)
        .subscription(Tbgui::subscription)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window_size((500.0, 800.0))
        .run_with(Tbgui::new)
}

#[derive(Debug)]
enum Tbgui {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    items: Vec<Item>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateItem,
    FilterChanged(Filter),
    ItemMessage(usize, ItemMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl Tbgui {
    fn new() -> (Self, Task<Message>) {
        (
            Self::Loading,
            Task::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Tbgui::Loading => false,
            Tbgui::Loaded(state) => state.dirty,
        };

        format!("TbGUI{} - IMM", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Tbgui::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Tbgui::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            items: state.items,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Tbgui::Loaded(State::default());
                    }
                    _ => {}
                }

                text_input::focus("new-item")
            }
            Tbgui::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;

                        Task::none()
                    }
                    Message::CreateItem => {
                        if !state.input_value.is_empty() {
                            state.items.push(Item::new(state.input_value.clone()));
                            state.input_value.clear();
                        }

                        Task::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Task::none()
                    }
                    Message::ItemMessage(i, item_message) => {
                        if let Some(item) = state.items.get_mut(i) {
                            item.update(item_message);
                            Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    Message::Saved(_result) => {
                        state.saving = false;
                        saved = true;

                        Task::none()
                    }
                    Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    Message::ToggleFullscreen(mode) => window::get_latest()
                        .and_then(move |window| window::change_mode(window, mode)),
                    Message::Loaded(_) => Task::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Task::perform(
                        SavedState {
                            input_value: state.input_value.clone(),
                            filter: state.filter,
                            items: state.items.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Task::none()
                };

                Task::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Tbgui::Loading => loading_message(),
            Tbgui::Loaded(State {
                input_value,
                filter,
                items,
                ..
            }) => {
                let title = text("tbgui")
                    .width(Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Center);

                let input = text_input("What needs to be done?", input_value)
                    .id("new-item")
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateItem)
                    .padding(15)
                    .size(30)
                    .align_x(Center);

                let controls = view_controls(items, *filter);
                let filtered_items = items.iter().filter(|item| filter.matches(item));

                let items: Element<_> = if filtered_items.count() > 0 {
                    keyed_column(
                        items
                            .iter()
                            .enumerate()
                            .filter(|(_, item)| filter.matches(item))
                            .map(|(i, item)| {
                                (
                                    item.id,
                                    item.view()
                                        .map(move |message| Message::ItemMessage(i, message)),
                                )
                            }),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        Filter::All => "You have not created a item yet...",
                        Filter::Unchecked => "All your items are done! :D",
                        Filter::Checked => "You have not completed a item yet...",
                    })
                };

                let content = column![title, input, controls, items]
                    .spacing(20)
                    .max_width(800);

                scrollable(container(content).center_x(Fill).padding(40)).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                _ => None,
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    sample: String,
    read1: String,
    read2: String,
    is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    CheckboxToggled(bool),
}

impl Item {
    fn new(sample: String) -> Self {
        Item {
            id: Uuid::new_v4(),
            sample,
            read1: String::new(),
            read2: String::new(),
            is_checked: false,
        }
    }

    fn update(&mut self, message: ItemMessage) {
        match message {
            ItemMessage::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
        }
    }

    fn view(&self) -> Element<ItemMessage> {
        let checkbox = checkbox(&self.sample, self.is_checked)
            .on_toggle(ItemMessage::CheckboxToggled)
            .width(Fill)
            .size(17)
            .text_shaping(text::Shaping::Advanced);

        row![checkbox,].spacing(20).align_y(Center).into()
    }
}

fn view_controls(items: &[Item], current_filter: Filter) -> Element<Message> {
    let items_checked = items.iter().filter(|item| item.is_checked).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![
        text!(
            "{items_checked} {} selected",
            if items_checked == 1 { "item" } else { "items" }
        )
        .width(Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Unchecked", Filter::Unchecked, current_filter),
            filter_button("Checked", Filter::Checked, current_filter,),
        ]
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Unchecked,
    Checked,
}

impl Filter {
    fn matches(self, item: &Item) -> bool {
        match self {
            Filter::All => true,
            Filter::Unchecked => !item.is_checked,
            Filter::Checked => item.is_checked,
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    center(text("Loading...").width(Fill).align_x(Center).size(50)).into()
}

fn empty_message(message: &str) -> Element<'_, Message> {
    center(
        text(message)
            .width(Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .height(200)
    .into()
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "IMM", "tbgui")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("tbgui.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
