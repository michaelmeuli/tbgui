use crate::config::TbguiConfig;
use crate::types::{Filter, Item, Message};
use iced::alignment::Horizontal::Left;
use iced::widget::{
    button, center, column, container, keyed_column, row, scrollable, svg, text, text_input, Space,
};
use iced::{Center, Element, Fill};

pub fn view_home<'a>(
    filter: &'a Filter,
    items: &'a [Item],
    error_message: &'a Option<String>,
) -> Element<'a, Message> {
    let title = text("TB-Profiler")
        .width(Fill)
        .size(60)
        .color([0.5, 0.5, 0.5])
        .align_x(Center);

    let run_controls = row![
        button("Run Profiler").on_press(Message::RunTbProfiler),
        button("Download Results").on_press(Message::DownloadResults),
        button("Delete Results").on_press(Message::DeleteResults),
        Space::with_width(iced::Length::Fill),
        gear_button().on_press(Message::SettingsPressed),
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
        if let Some(error) = error_message {
            empty_message(error)
        } else {
            empty_message(match filter {
                Filter::All => "No raw read sequences found...",
                Filter::Unchecked => "All raw read sequences selected...",
                Filter::Checked => "No raw read sequences selected...",
            })
        }
    };

    scrollable(
        container(
            column![title, run_controls, controls, items]
                .spacing(20)
                .max_width(800),
        )
        .center_x(Fill)
        .padding(40),
    )
    .into()
}

pub fn view_settings<'a>() -> Element<'a, Message> {
    let title = text("Settings")
        .width(Fill)
        .size(60)
        .color([0.5, 0.5, 0.5])
        .align_x(Center);
    let controls = row![
        button("Home").on_press(Message::HomePressed).width(80),
        Space::with_width(iced::Length::Fill),
        gear_button().on_press(Message::ConfigPressed),
    ];
    let template = column![
        button("Download default template")
            .on_press(Message::DownloadDefaultTemplate)
            .width(250),
        button("Upload user template")
            .on_press(Message::UploadUserTemplate)
            .width(250),
    ]
    .spacing(20);

    let content = column![
        title, 
        controls, 
        Space::with_height(iced::Length::Fixed(40.0)),
        template
        ]
        .spacing(20)
        .max_width(800);

    scrollable(container(content).center_x(Fill).padding(40)).into()
}

pub fn view_config<'a>(config: &'a TbguiConfig) -> Element<'a, Message> {
    let title = text("Settings")
        .width(Fill)
        .size(60)
        .color([0.5, 0.5, 0.5])
        .align_x(Center);
    let controls = row![
        button("Home").on_press(Message::HomePressed).width(80),
        button("Reset to default").on_press(Message::ResetConfig).width(150),
        Space::with_width(iced::Length::Fill),
        gear_button().on_press(Message::ConfigPressed),
    ]
    .spacing(20);

    let name_text = text("Username:").width(Fill).size(16).align_x(Left);
    let name_input = text_input("username", &config.username)
        .on_input(Message::ConfigNameChanged)
        .on_submit(Message::ConfigNameSubmitted)
        .padding(5)
        .size(16)
        .align_x(Left);
    let name = column![name_text, name_input].spacing(10);

    let rawdir_text = text("Path to raw dir on remote:")
        .width(Fill)
        .size(16)
        .align_x(Left);
    let rawdir_input = text_input("Path to raw dir on remote", &config.remote_raw_dir)
        .on_input(Message::ConfigRawDirChanged)
        .on_submit(Message::ConfigRawDirSubmitted)
        .padding(5)
        .size(16)
        .align_x(Left);
    let rawdir = column![rawdir_text, rawdir_input].spacing(10);

    let script_text = text("Path to TB Profiler script on remote:")
        .width(Fill)
        .size(16)
        .align_x(Left);
    let script_input = text_input(
        "Path to TB Profiler script on remote",
        &config.tb_profiler_script,
    )
    .on_input(Message::ConfigScriptPathChanged)
    .on_submit(Message::ConfigScriptPathSubmitted)
    .padding(5)
    .size(16)
    .align_x(Left);
    let script = column![script_text, script_input].spacing(10);

    let results_text = text("Remote results dir:")
        .width(Fill)
        .size(16)
        .align_x(Left);
    let results_input = text_input("Remote results dir", &config.remote_results_dir)
        .on_input(Message::ConfigResultsPathChanged)
        .on_submit(Message::ConfigResultsPathSubmitted)
        .padding(5)
        .size(16)
        .align_x(Left);
    let results = column![results_text, results_input].spacing(10);

    let default_template_text = text("Default template remote:")
        .width(Fill)
        .size(16)
        .align_x(Left);
    let default_template_input =
        text_input("Default template remote", &config.default_template_remote)
            .on_input(Message::ConfigDefaultTemplateChanged)
            .on_submit(Message::ConfigDefaultTemplateSubmitted)
            .padding(5)
            .size(16)
            .align_x(Left);
    let default_template = column![default_template_text, default_template_input].spacing(10);

    let user_template_text = text("Default template remote:")
        .width(Fill)
        .size(16)
        .align_x(Left);
    let user_template_input = text_input("Default template remote", &config.user_template_remote)
        .on_input(Message::ConfigUserTemplateChanged)
        .on_submit(Message::ConfigUserTemplateSubmitted)
        .padding(5)
        .size(16)
        .align_x(Left);
    let user_template = column![user_template_text, user_template_input].spacing(10);
    //user_template_remote

    let content = column![
        title,
        controls,
        name,
        rawdir,
        script,
        results,
        default_template,
        user_template
    ]
    .spacing(20)
    .max_width(800);

    scrollable(container(content).center_x(Fill).padding(40)).into()
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

pub fn loading_message<'a>() -> Element<'a, Message> {
    center(text("Loading...").width(Fill).align_x(Center).size(50)).into()
}

pub fn empty_message(message: &str) -> Element<'_, Message> {
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

pub fn gear_button() -> iced::widget::Button<'static, Message> {
    let handle = svg::Handle::from_path(format!(
        "{}/icons/gear-solid.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    button(svg(handle).width(20).height(20))
}
