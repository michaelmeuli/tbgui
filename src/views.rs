use iced::widget::{button, column, container, keyed_column, row, scrollable, text};
use iced::{Center, Element, Fill};
use crate::{Filter, Item, Message, empty_message, gear_button, view_controls};


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
        gear_button(),
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

    scrollable(container(column![title, run_controls, controls, items].spacing(20).max_width(800)).center_x(Fill).padding(40)).into()
}