mod utils;
use utils::*;

use async_ssh2_tokio::client::Client;
use iced::widget::{
    self, button, center, checkbox, column, container, keyed_column, row, scrollable, text,
};
use iced::window;
//use iced::{keyboard, time};
use iced::keyboard;
use iced::{Center, Element, Fill, Subscription, Task};
//use std::time::Duration;
use uuid::Uuid;

const USERNAME: &str = "mimeul";
const REMOTE_RAW_DIR: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/raw";
const TB_PROFILER_SCRIPT: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/scripts/tbprofiler.sh";
const REMOTE_RESULTS_DIR: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/out/results";

pub fn main() -> iced::Result {
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
    filter: Filter,
    items: Vec<Item>,
    client: Option<Client>,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<RemoteState, LoadError>),
    FilterChanged(Filter),
    ItemMessage(usize, ItemMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
    RunTbProfiler,
    ProfilerRunCompleted,
    DownloadResults,
    DeleteResults,
}

impl Tbgui {
    fn new() -> (Self, Task<Message>) {
        (Self::Loading, Task::perform(load(), Message::Loaded))
    }

    fn title(&self) -> String {
        "TbGUI - IMM".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Tbgui::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Tbgui::Loaded(State {
                            items: state.items,
                            client: Some(state.client),
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Tbgui::Loaded(State::default());
                    }
                    _ => {}
                }
                Task::none()
            }
            Tbgui::Loaded(state) => {
                let command = match message {
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
                    Message::RunTbProfiler => {
                        let items_checked =
                            state.items.iter().filter(|item| item.is_checked).count();
                        let samples = state
                            .items
                            .iter()
                            .filter(|item| item.is_checked)
                            .map(|item| item.sample.clone())
                            .collect::<Vec<String>>()
                            .join(",");
                        println!("Running TB-Profiler for samples: {}", samples);
                        let client = state.client.clone();
                        Task::perform(
                            async move {
                                if let Some(client) = client {
                                    if let Err(e) =
                                        run_tbprofiler(&client, items_checked, samples).await
                                    {
                                        eprintln!("Error running tbprofiler: {:?}", e);
                                    }
                                }
                            },
                            |_| Message::ProfilerRunCompleted,
                        )
                    }
                    Message::ProfilerRunCompleted => Task::none(),
                    Message::DownloadResults => {
                        let client = state.client.clone();
                        Task::perform(
                            async move {
                                if let Some(client) = client {
                                    if let Err(e) = download_results(&client).await {
                                        eprintln!("Error downloading results: {:?}", e);
                                    }
                                }
                            },
                            |_| Message::ProfilerRunCompleted,
                        )
                    }
                    Message::DeleteResults => {
                        let client = state.client.clone();
                        Task::perform(
                            async move {
                                if let Some(client) = client {
                                    if let Err(e) = delete_results(&client).await {
                                        eprintln!("Error deleting results: {:?}", e);
                                    }
                                }
                            },
                            |_| Message::ProfilerRunCompleted,
                        )
                    }
                };
                command
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Tbgui::Loading => loading_message(),
            Tbgui::Loaded(State { filter, items, .. }) => {
                let title = text("TB-Profiler")
                    .width(Fill)
                    .size(60)
                    .color([0.5, 0.5, 0.5])
                    .align_x(Center);

                //let button = button("Run Profiler").on_press(Message::RunTbProfiler);
                //let button: Element<Message> = button("Run Profiler").into();
                let run_controls = row![
                    button("Run Profiler").on_press(Message::RunTbProfiler),
                    button("Download Results").on_press(Message::DownloadResults),
                    button("Delete Results").on_press(Message::DeleteResults),
                    ]
                    .spacing(20);
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

                let content = column![title, run_controls, controls, items]
                    .spacing(20)
                    .max_width(800);

                scrollable(container(content).center_x(Fill).padding(40)).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        //let keyboard_subscription = keyboard::on_key_press(|key, modifiers| {
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
        //let periodic_subscription = time::every(Duration::from_secs(9 * 60)).map(|_| Message::DownloadResults);
        //Subscription::batch(vec![keyboard_subscription, periodic_subscription])
    }
}

#[derive(Debug, Clone)]
struct Item {
    id: Uuid,
    sample: String,
    is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    CheckboxToggled(bool),
}

impl Item {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

#[derive(Debug, Clone)]
enum LoadError {
    SSH,
}

#[derive(Debug, Clone)]
struct RemoteState {
    client: Client,
    items: Vec<Item>,
}

async fn load() -> Result<RemoteState, LoadError> {
    delete_log_file();
    match create_client().await {
        Ok(client) => {
            println!("Connected to the server");
            let reads = get_raw_reads(&client).await.map_err(|_| LoadError::SSH)?;

            let tasks = create_tasks(reads);
            Ok(RemoteState {
                client,
                items: tasks,
            })
        }
        Err(e) => {
            let error = format!("load(): Failed to connect to the server: {}", e);
            println!("{}", error);
            log_error(error.as_str());
            Err(LoadError::SSH)
        }
    }
}
