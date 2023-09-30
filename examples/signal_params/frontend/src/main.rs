use zoon::*;

// The purpose of this example is to experiment with API designs and implementations -
// see the related issue: https://github.com/MoonZoon/MoonZoon/issues/186

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        // A
        .item(el_with_optional_child(always(Text::new("A"))))
        // B
        .item(el_with_optional_child(always("B")))
        // C
        .item(el_with_optional_child::<signal::Always<Text>>(None))
        // D
        .item(el_with_optional_child_with_flag(
            true,
            always(Text::new("D")),
        ))
        // E
        .item(el_with_optional_child_with_flag(false, always("E")))
        // F
        .item(el_with_optional_child_with_flag::<signal::Always<Text>>(
            true, None,
        ))
        // G
        .item(el_with_optional_child_with_flag::<signal::Always<Text>>(
            false, None,
        ))
        // --
        // H
        .item(el_with_optional_child::<signal::Always<_>>(Some(always(
            "H",
        ))))
        // I
        // .item(el_with_optional_child::<???>(Some(always("I").map(|s| ["mapped", " ", s].concat()))))
        // ----
        //
        // O
        .item(link_with_optional_to(always("https://example.com/O")))
        // P
        .item(link_with_optional_to(always(String::from(
            "https://example.com/P",
        ))))
        // Q
        .item(link_with_optional_to::<signal::Always<u8>>(None))
        // R
        .item(link_with_optional_to_with_flag(
            true,
            always("https://example.com/R"),
        ))
        // R
        .item(link_with_optional_to_with_flag(false, always("R")))
        // S
        .item(link_with_optional_to_with_flag::<signal::Always<u8>>(
            true, None,
        ))
        // T
        .item(link_with_optional_to_with_flag::<signal::Always<u8>>(
            false, None,
        ))
        // --
        // U
        .item(link_with_optional_to::<signal::Always<_>>(Some(always(
            "https://example.com/U",
        ))))
    // V
    // .item(el_with_optional_child::<???>(Some(always("V").map(|s| ["mapped", " ", s].concat()))))
    // ----
    // And what about event handlers? Make them optional somehow as well? What about their signals?
}

fn el_with_optional_child<S: Signal<Item = impl IntoOptionElement<'static>> + Unpin + 'static>(
    child: impl Into<Option<S>>,
) -> impl Element {
    let el = El::new();
    if let Some(child) = child.into() {
        return el.child_signal(child).left_either();
    }
    el.right_either()
}

fn el_with_optional_child_with_flag<
    S: Signal<Item = impl IntoOptionElement<'static>> + Unpin + 'static,
>(
    add_child: bool,
    child: impl Into<Option<S>>,
) -> impl Element {
    let el = El::new();
    if add_child {
        if let Some(child) = child.into() {
            return el.child_signal(child).left_either();
        }
    }
    el.right_either()
}

fn link_with_optional_to<S: Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static>(
    to: impl Into<Option<S>>,
) -> impl Element {
    let link = Link::new().label("Link 1");
    if let Some(to) = to.into() {
        return link.to_signal(to).left_either();
    }
    link.to("https://example.com/default").right_either()
}

fn link_with_optional_to_with_flag<S: Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static>(
    add_to: bool,
    to: impl Into<Option<S>>,
) -> impl Element {
    let link = Link::new().label("Link 2");
    if add_to {
        if let Some(to) = to.into() {
            return link.to_signal(to).left_either();
        }
    }
    link.to("https://example.com/default").right_either()
}
