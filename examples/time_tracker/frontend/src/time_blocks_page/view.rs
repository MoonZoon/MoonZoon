use zoon::*;
use crate::{theme::Theme, app};
use std::sync::Arc;
use shared::time_blocks::TimeBlockStatus;

pub fn page() -> impl Element {
    Column::new()
        .item(title())
        .item(content())
}

fn title() -> impl Element {
    El::new()
        .s(Width::fill().max(600))
        .s(Padding::new().y(35))
        .child(
            El::with_tag(Tag::H1)
                .s(Align::center())
                .s(Font::new().size(30).weight(NamedWeight::SemiBold))
                .child("Time Blocks")
        )
}

fn content() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Padding::new().x(10).bottom(10))
        .item(clients())
}

// -- clients --

fn clients() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Align::new().center_x())
        .items_signal_vec(super::clients().signal_vec_cloned().map(client))
}

fn client(client: Arc<super::Client>) -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background1))
        .s(RoundedCorners::all(10))
        .s(Padding::all(15))
        .s(Spacing::new(20))
        .item(client_name_and_stats(client.clone()))
        .item(add_entity_button("Add Time Block", clone!((client) move || super::add_time_block(&client))))
        .item(time_blocks(client))
}

fn client_name_and_stats(client: Arc<super::Client>) -> impl Element {
    Row::new()
        .s(Spacing::new(10))
        .item(client_name(client.clone()))
        .item(stats(client))
}

fn client_name(client: Arc<super::Client>) -> impl Element {
    El::new()
        .s(Font::new().color(Theme::Font1).size(20))
        .s(Background::new().color(Theme::Transparent))
        .s(Padding::all(8))
        .child(&client.name)
}

fn stats(client: Arc<super::Client>) -> impl Element {
    let tracked = client.tracked;


    let format = |value: f64| format!("{:.1}", value);
    Row::new()
        .s(Font::new().color(Theme::Font1))
        .s(Spacing::new(5))
        .s(Align::new().right())
        .multiline()
        .item(
            Column::new()
                .s(Spacing::new(5))
                .s(Padding::all(10))
                .s(Shadows::new(vec![
                    Shadow::new().y(8).blur(16).color(Theme::Shadow)
                ]))
                .s(RoundedCorners::all(10))
                .s(Align::new().right())
                .item(
                    Row::new()
                        .s(Spacing::new(10))
                        .item("Blocked")
                        .item(El::new().s(Align::new().right()).child(format(40.)))
                )
                .item(
                    Column::new()
                        .item(
                            Row::new()
                                .s(Spacing::new(10))
                                .item("Unpaid")
                                .item(El::new().s(Align::new().right()).child(format(0.)))
                        )
                        .item(
                            Row::new()
                                .s(Spacing::new(10))
                                .item("Paid")
                                .item(El::new().s(Align::new().right()).child(format(23.)))
                        )
                )
        )
        .item(
            Column::new()
                .s(Spacing::new(5))
                .s(Padding::all(10))
                .s(Shadows::new(vec![
                    Shadow::new().y(8).blur(16).color(Theme::Shadow)
                ]))
                .s(RoundedCorners::all(10))
                .s(Align::new().right())
                .item(
                    Row::new()
                        .s(Spacing::new(10))
                        .item("Tracked")
                        .item(El::new().s(Align::new().right()).child(format(34.8569)))
                )
                .item(
                    Row::new()
                        .s(Spacing::new(10))
                        .s(Font::new().no_wrap())
                        .item("To Block")
                        .item(El::new().s(Align::new().right()).child(format(-42.16)))
                )
        )
}

// -- time blocks --

fn time_blocks(client: Arc<super::Client>) -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .items_signal_vec(client.time_blocks.signal_vec_cloned().map(move |tb| {
            time_block(client.clone(), tb)
        }))
}

fn time_block(client: Arc<super::Client>, time_block: Arc<super::TimeBlock>) -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background0))
        .s(RoundedCorners::new().left(10).right(40 / 2))
        .item(timeblock_name_duration_and_delete_button(client, time_block.clone()))
        .item(status_buttons(time_block.clone()))
        .item_signal(time_block.invoice.signal_cloned().map_option(
            clone!((time_block) move |i: Arc<super::Invoice>| invoice(time_block.clone(), i.clone()).into_raw_element()),
            move || add_entity_button("Add Invoice", clone!((time_block) move || super::add_invoice(&time_block))).into_raw_element()
        ))
}

fn timeblock_name_duration_and_delete_button(client: Arc<super::Client>, time_block: Arc<super::TimeBlock>) -> impl Element {
    let id = time_block.id;
    Row::new()
        .s(Spacing::new(10))
        .s(Padding::new().left(8))
        .item(time_block_name(time_block.clone()))
        .item(time_block_duration(time_block.clone()))
        .item(delete_entity_button(move || super::delete_time_block(&client, id)))
}

fn time_block_name(time_block: Arc<super::TimeBlock>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    TextInput::new()
        .s(Width::fill())
        .s(Font::new().color(Theme::Font0))
        .s(Background::new().color(Theme::Transparent))
        .s(Borders::new().bottom(
            Border::new().color(Theme::Border1)
        ))
        .s(Padding::all(5))
        .focus(not(time_block.is_old))
        .label_hidden("time_block name")
        .text_signal(time_block.name.signal_cloned())
        .on_change(move |text| {
            time_block.name.set_neq(text);
            debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
                super::rename_time_block(time_block.id, &time_block.name.lock_ref())
            })))
        })
}

fn time_block_duration(time_block: Arc<super::TimeBlock>) -> impl Element {
    Row::new()
        .s(Font::new().color(Theme::Font0))
        .item(time_block_duration_input(time_block))
        .item("h")
}

fn time_block_duration_input(time_block: Arc<super::TimeBlock>) -> impl Element {
    let debounced_set_duration = Mutable::new(None);
    let (text_duration, text_duration_signal) = Mutable::new_and_signal_cloned(time_block.duration.get().num_hours().to_string());
    let (is_valid, is_valid_signal) = Mutable::new_and_signal(true);
    TextInput::new()
        .s(Width::zeros(4))
        .s(Font::new().color(Theme::Font0))
        .s(Background::new().color_signal(is_valid_signal.map_bool(|| Theme::Transparent, || Theme::BackgroundInvalid)))
        .s(Borders::new().bottom(
            Border::new().color(Theme::Border1)
        ))
        .s(Padding::all(5))
        .label_hidden("time_block duration")
        .text_signal(text_duration_signal)
        .on_change(move |text| {
            let hours = text.parse();
            is_valid.set_neq(hours.is_ok());
            text_duration.set_neq(text);
            if let Ok(hours) = hours {
                time_block.duration.set_neq(Duration::hours(hours).into());
                debounced_set_duration.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
                    super::set_time_block_duration(&time_block, time_block.duration.get())
                })))
            }
        })
}

fn status_buttons(time_block: Arc<super::TimeBlock>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Font::new().color(Theme::Font0))
        .s(RoundedCorners::all_max())
        .s(Borders::all(Border::new().color(Theme::Border1)))
        .s(Clip::both())
        .item(status_button("Non-billable", TimeBlockStatus::NonBillable, time_block.clone()))
        .item(status_button("Unpaid", TimeBlockStatus::Unpaid, time_block.clone()))
        .item(status_button("Paid", TimeBlockStatus::Paid, time_block))
}

fn status_button(label: &str, represent_status: TimeBlockStatus, time_block: Arc<super::TimeBlock>) -> impl Element {
    let hovered = Mutable::new(false);
    let hovered_or_active = Broadcaster::new(map_ref! {
        let hovered = hovered.signal(),
        let active = time_block.status.signal().map(move |status| status == represent_status) =>
        *hovered || *active
    });
    Button::new()
        .s(Padding::new().x(13).y(6))
        .s(Background::new().color_signal(hovered_or_active.signal().map_bool(|| Theme::Background3, || Theme::Transparent)))
        .s(Font::new().color_signal(hovered_or_active.signal().map_bool(|| Theme::Font3, || Theme::Font0)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(label)
        .on_press(move || super::set_time_block_status(&time_block, represent_status))
}

// -- invoice --

fn invoice(time_block: Arc<super::TimeBlock>, invoice: Arc<super::Invoice>) -> impl Element {
    El::new()
        .s(Padding::all(10))
        .child(
            Column::new()
                .s(Padding::new().left(10))
                .s(Background::new().color(Theme::Background1))
                .s(RoundedCorners::all(40 / 2))
                .item(invoice_custom_id_and_delete_button(time_block.clone(), invoice.clone()))
                .item(invoice_url_and_link_button(invoice))
        )
}

fn invoice_custom_id_and_delete_button(time_block: Arc<super::TimeBlock>, invoice: Arc<super::Invoice>) -> impl Element {
    Row::new()
        .item(invoice_custom_id(invoice))
        .item(
            El::new()
                .s(Align::new().right())
                .s(Padding::new().bottom(5))
                .child(delete_entity_button(move || super::delete_invoice(&time_block)))
        )
}

fn invoice_custom_id(invoice: Arc<super::Invoice>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    El::new()
        .child(
            TextInput::new()
                .s(Width::fill())
                .s(Font::new().color(Theme::Font1))
                .s(Background::new().color(Theme::Transparent))
                .s(Borders::new().bottom(Border::new().color(Theme::Border1)))
                .s(Padding::all(5))
                .placeholder(Placeholder::new("Invoice custom ID"))
                .focus(not(invoice.is_old))
                .label_hidden("invoice custom id")
                .text_signal(invoice.custom_id.signal_cloned())
                .on_change(move |text| {
                    invoice.custom_id.set_neq(text);
                    debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
                        super::set_invoice_custom_id(invoice.id, &invoice.custom_id.lock_ref())
                    })))
                })
        )
}

fn invoice_url_and_link_button(invoice: Arc<super::Invoice>) -> impl Element {
    Row::new()
        .item(invoice_url(invoice.clone()))
        .item(
            El::new()
                .s(Align::new().right())
                .s(Padding::new().top(5))
                .child(link_button(invoice))
        )
}

fn invoice_url(invoice: Arc<super::Invoice>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    El::new()
        .child(
            TextInput::new()
                .s(Width::fill())
                .s(Font::new().color(Theme::Font1))
                .s(Background::new().color(Theme::Transparent))
                .s(Borders::new().bottom(Border::new().color(Theme::Border1)))
                .s(Padding::all(5))
                .placeholder(Placeholder::new("Invoice URL"))
                .label_hidden("invoice url")
                .text_signal(invoice.url.signal_cloned())
                .on_change(move |text| {
                    invoice.url.set_neq(text);
                    debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
                        super::set_invoice_url(invoice.id, &invoice.url.lock_ref())
                    })))
                })
        )
}

// --

fn add_entity_button(title: &str, on_press: impl FnOnce() + Clone + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .child(
            Button::new()
                .s(Align::center())
                .s(Background::new().color_signal(hovered_signal.map_bool(
                    || Theme::Background3Highlighted,
                    || Theme::Background3,
                )))
                .s(Font::new().color(Theme::Font3).weight(NamedWeight::SemiBold))
                .s(Padding::all(5))
                .s(RoundedCorners::all_max())
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(on_press)
                .label(add_entity_button_label(title))
        )
}

fn add_entity_button_label(title: &str) -> impl Element {
    Row::new()
    .item(app::icon_add())
    .item(
        El::new()
            .s(Padding::new().right(8).bottom(1))
            .child(title)
    )
}

fn delete_entity_button(on_press: impl FnOnce() + Clone + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Align::center())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(on_press)
        .label(app::icon_delete_forever())
}

fn link_button(invoice: Arc<super::Invoice>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Align::center())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to_signal(invoice.url.signal_cloned())
        .new_tab()
        .label(app::icon_open_in_new())
}
