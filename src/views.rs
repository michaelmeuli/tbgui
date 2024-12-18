use crate::types::{Filter, Item, Message};
use iced::mouse::Button;
use iced::widget::{button, center, column, container, keyed_column, row, scrollable, svg, text};
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
    let handle = svg::Handle::from_path(format!(
        "{}/icons/gear-solid.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let controls = row![
        button("Home").on_press(Message::HomePressed).width(80),
        gear_button().on_press(Message::HomePressed),
    ]
    .spacing(360);
    let template = column![
        button("Download default template").on_press(Message::HomePressed).width(250),
        button("Upload user template").on_press(Message::HomePressed).width(250),
    ]
    .spacing(20);

    let content = column![title, controls, template].spacing(20).max_width(800);

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
