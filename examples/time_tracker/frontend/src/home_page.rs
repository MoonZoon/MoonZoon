use zoon::*;

mod view;

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElOrText {
    view::page().into_raw()
}
