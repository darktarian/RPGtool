use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn Page4() -> Element {
    info!("page 4 ");
    rsx! {
        div {
            div { class: "btn btn-cth-eldritch w-25", id: "btn-p4", "Page 4" }
        }
    }
}