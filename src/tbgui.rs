use crate::utils::*;
use crate::views::*;
use iced::keyboard;
use iced::widget;
use iced::window;
use iced::{Element, Subscription, Task};

use crate::types::{Message, Screen, State};

#[derive(Debug)]
pub enum Tbgui {
    Loading,
    Loaded(State),
}

impl Tbgui {
    pub fn new() -> (Self, Task<Message>) {
        //(Self::Loading, Task::perform(load(), Message::Loaded))
        (
            Tbgui::Loaded(State::default()),
            Task::perform(load(), Message::Loaded),
        )
    }

    pub fn title(&self) -> String {
        "TbGUI - IMM".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
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
                    Message::Loaded(Err(e)) => {
                        *self = Tbgui::Loaded(State {
                            error_message: Some(e.error),
                            ..State::default()
                        });
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
                                        println!("Error running tbprofiler: {:?}", e);
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
                                        println!("Error downloading results: {:?}", e);
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
                                        println!("Error deleting results: {:?}", e);
                                    }
                                }
                            },
                            |_| Message::ProfilerRunCompleted,
                        )
                    }
                    Message::SettingsPressed => {
                        state.screen = Screen::Settings;
                        Task::none()
                    }
                    Message::HomePressed => {
                        state.screen = Screen::Home;
                        Task::none()
                    }
                };
                command
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            Tbgui::Loading => loading_message(),
            Tbgui::Loaded(State {
                screen,
                filter,
                items,
                error_message,
                ..
            }) => match screen {
                Screen::Home => view_home(filter, items, error_message),
                Screen::Settings => view_settings(),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
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
