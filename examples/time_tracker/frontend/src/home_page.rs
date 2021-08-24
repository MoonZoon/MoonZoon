use zoon::*;

mod view;

pub fn view() -> RawElement {
    view::page().into_raw_element()
}

// blocks!{
//     append_blocks![els]
// }
