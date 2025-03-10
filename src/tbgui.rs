use crate::config::TbguiConfig;
use crate::ssh::*;
use crate::types::{Message, Screen, State};
use crate::utils::*;
use crate::views::*;
use iced::futures::FutureExt;
use iced::widget;
use iced::window;
use iced::{Element, Subscription, Task};
use iced::{keyboard, time};
use std::time::Duration;

#[derive(Debug)]
pub enum Tbgui {
    Loading,
    Loaded(State),
}

impl Tbgui {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self::Loading,
            Task::perform(
                async {
                    delete_log_file();
                    confy::load("tbgui", None).unwrap_or_else(|_| {
                        let default_config = TbguiConfig::default();
                        confy::store("tbgui", None, &default_config).unwrap();
                        default_config
                    })
                }
                .map(|config: TbguiConfig| {
                    Ok::<TbguiConfig, String>(config)
                        .map_err(|e| format!("Error loading config: {:?}", e))
                }),
                Message::Loaded,
            ),
        )
    }

    pub fn title(&self) -> String {
        "TbGUI - IMM".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Tbgui::Loading => {
                if let Message::Loaded(result) = message {
                    match result {
                        Ok(state) => {
                            *self = Tbgui::Loaded(State {
                                config: state,
                                ..State::default()
                            });
                        }
                        Err(e) => {
                            log_error(&e);
                            *self = Tbgui::Loaded(State::default());
                        }
                    }
                }
                Task::done(Message::CreateClient)
            }
            Tbgui::Loaded(state) => {
                let command = match message {
                    Message::CreateClient => {
                        state.error_message = Some("Connecting".to_string());
                        state.info_view_message = Some("Connecting".to_string());
                        state.screen = Screen::Home;
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                create_client(&config).await.map_err(|e| {
                                    format!("Error returned from create_client(): {:?}", e)
                                })
                            },
                            Message::CreatedClient,
                        )
                    }
                    Message::CreatedClient(result) => {
                        match result {
                            Ok(client) => {
                                state.client = Some(client);
                                state.error_message =
                                    Some("Loading samples from remote".to_string());
                                state.info_view_message =
                                    Some("Loading samples from remote".to_string());
                            }
                            Err(e) => {
                                state.client = None;
                                state.error_message = Some(e.clone());
                                state.error_view_message = Some(e.clone());
                                log_error(&e);
                                state.screen = Screen::Error;
                            }
                        }
                        Task::done(Message::LoadRemoteState)
                    }
                    Message::LoadRemoteState => {
                        state.screen = Screen::Home;
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        get_raw_reads(&client, &config).await.map_err(|e| {
                                            format!("Error returned from get_raw_reads(): {:?}", e)
                                        })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            Message::LoadedRemoteState,
                        )
                    }
                    Message::LoadedRemoteState(result) => {
                        state.error_message = None;
                        match result {
                            Ok(remote_state) => {
                                state.items = remote_state.items;
                                Task::none()
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_message = Some(e);
                                Task::none()
                            }
                        }
                    }

                    Message::FilterChanged(filter) => {
                        state.filter = filter;
                        Task::none()
                    }
                    Message::Item(i, item_message) => {
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
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        run_tbprofiler(&client, items_checked, samples, &config)
                                            .await
                                            .map_err(|e| {
                                                format!("Error running tbprofiler: {:?}", e)
                                            })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            Message::ProfilerRunStarted,
                        )
                    }
                    Message::DownloadResults => {
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        download_results(&client, &config).await.map_err(|e| {
                                            format!(
                                                "Error returned from download_results(): {:?}",
                                                e
                                            )
                                        })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            Message::DownloadedResults,
                        )
                    }
                    Message::DeleteResults => {
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        delete_results(&client, &config).await.map_err(|e| {
                                            format!("Error returned from delete_results(): {:?}", e)
                                        })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            Message::DeletedResults,
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
                    Message::DownloadDefaultTemplate => {
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client { Some(client) => {
                                    download_default_template(&client, &config).await.map_err(|e| {
                                            format!("Error returned from download_default_template(): {:?}", e)
                                        })
                                } _ => {
                                    Err("Client is None".to_string())
                                }}
                            },
                            Message::DownloadedDefaultTemplate,
                        )
                    }
                    Message::UploadUserTemplate => {
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        upload_user_template(&client, &config).await.map_err(|e| {
                                            format!(
                                                "Error returned from upload_user_template(): {:?}",
                                                e
                                            )
                                        })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            Message::UploadedUserTemplate,
                        )
                    }
                    Message::ProfilerRunStarted(result) => {
                        match result {
                            Ok(result) => {
                                state.info_view_message =
                                    Some(format!("Batch started successfully: {}", result));
                                state.screen = Screen::Info;
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_view_message = Some(e);
                                state.screen = Screen::Error;
                            }
                        }
                        state.is_running = true;
                        Task::none()
                    }
                    Message::DownloadedResults(result) => {
                        match result {
                            Ok(_) => {
                                state.info_view_message =
                                    Some("Results downloaded successfully".to_string());
                                state.screen = Screen::Info;
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_view_message = Some(e);
                                state.screen = Screen::Error;
                            }
                        }
                        Task::none()
                    }
                    Message::DeletedResults(result) => {
                        match result {
                            Ok(_) => {
                                state.info_view_message =
                                    Some("Results deleted successfully".to_string());
                                state.screen = Screen::Info;
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_view_message = Some(e);
                                state.screen = Screen::Error;
                            }
                        }
                        Task::none()
                    }
                    Message::DownloadedDefaultTemplate(result) => {
                        match result {
                            Ok(_) => {
                                state.info_view_message =
                                    Some("Default template downloaded successfully".to_string());
                                state.screen = Screen::Info;
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_view_message = Some(e);
                                state.screen = Screen::Error;
                            }
                        }
                        Task::none()
                    }
                    Message::UploadedUserTemplate(result) => {
                        match result {
                            Ok(_) => {
                                state.info_view_message =
                                    Some("User template uploaded successfully".to_string());
                                state.screen = Screen::Info;
                            }
                            Err(e) => {
                                log_error(&e);
                                state.error_view_message = Some(e);
                                state.screen = Screen::Error;
                            }
                        }
                        Task::none()
                    }
                    Message::ConfigPressed => {
                        state.screen = Screen::Config;
                        Task::none()
                    }
                    Message::ConfigNameChanged(username) => {
                        state.config.username = username;
                        Task::none()
                    }
                    Message::ConfigNameSubmitted => {
                        let config = TbguiConfig {
                            username: state.config.username.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::done(Message::CreateClient)
                    }
                    Message::ConfigRawDirChanged(remote_raw_dir) => {
                        state.config.remote_raw_dir = remote_raw_dir;
                        Task::none()
                    }
                    Message::ConfigRawDirSubmitted => {
                        let config = TbguiConfig {
                            remote_raw_dir: state.config.remote_raw_dir.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::done(Message::LoadRemoteState)
                    }
                    Message::ConfigScriptPathChanged(tb_profiler_script) => {
                        state.config.tb_profiler_script = tb_profiler_script;
                        Task::none()
                    }
                    Message::ConfigScriptPathSubmitted => {
                        let config = TbguiConfig {
                            tb_profiler_script: state.config.tb_profiler_script.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::none()
                    }
                    Message::ConfigOutPathChanged(remote_out_dir) => {
                        state.config.remote_out_dir = remote_out_dir;
                        Task::none()
                    }
                    Message::ConfigOutPathSubmitted => {
                        let config = TbguiConfig {
                            remote_out_dir: state.config.remote_out_dir.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::none()
                    }
                    Message::ConfigDefaultTemplateChanged(default_template_remote) => {
                        state.config.default_template_remote = default_template_remote;
                        Task::none()
                    }
                    Message::ConfigDefaultTemplateSubmitted => {
                        let config = TbguiConfig {
                            default_template_remote: state.config.default_template_remote.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::none()
                    }
                    Message::ConfigUserTemplateChanged(user_template_remote) => {
                        state.config.user_template_remote = user_template_remote;
                        Task::none()
                    }
                    Message::ConfigUserTemplateSubmitted => {
                        let config = TbguiConfig {
                            user_template_remote: state.config.user_template_remote.clone(),
                            ..state.config.clone()
                        };
                        confy::store("tbgui", None, &config).unwrap();
                        Task::none()
                    }
                    Message::ResetConfig => {
                        let config = TbguiConfig::default();
                        confy::store("tbgui", None, &config).unwrap();
                        state.config = config;
                        Task::none()
                    }
                    Message::CheckIfRunning => {
                        let client = state.client.clone();
                        let config = state.config.clone();
                        Task::perform(
                            async move {
                                match client {
                                    Some(client) => {
                                        check_if_running(&client, &config).await.map_err(|e| {
                                            format!(
                                                "Error returned from check_if_running(): {:?}",
                                                e
                                            )
                                        })
                                    }
                                    _ => Err("Client is None".to_string()),
                                }
                            },
                            |result| match result {
                                Ok(is_running) => Message::CheckIfRunningCompleted(is_running),
                                Err(e) => {
                                    log_error(&e);
                                    Message::CheckIfRunningCompleted(false)
                                }
                            },
                        )
                    }
                    Message::CheckIfRunningCompleted(is_running) => {
                        state.is_running = is_running;
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
                info_view_message,
                error_view_message,
                config,
                is_running,
                ..
            }) => match screen {
                Screen::Home => view_home(filter, items, error_message, is_running),
                Screen::Settings => view_settings(),
                Screen::Config => view_config(config),
                Screen::Info => view_info(info_view_message),
                Screen::Error => view_error(error_view_message),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        let keyboard_subscription = keyboard::on_key_press(|key, modifiers| {
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
        });
        let periodic_subscription =
            time::every(Duration::from_secs(10)).map(|_| Message::CheckIfRunning);
        Subscription::batch(vec![keyboard_subscription, periodic_subscription])
    }
}
