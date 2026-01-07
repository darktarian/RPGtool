
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn Page3() -> Element {
    info!("page 3 ");
    rsx! {
        div {
            div { class: "btn btn-cth-eldritch w-25", id: "btn-p3", "Page 3" }
        }
    }
}