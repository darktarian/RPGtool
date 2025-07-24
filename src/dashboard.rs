use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn Dashboard() -> Element {
    info!("page 2 ");
    rsx! {
        div{
            div { class:"btn btn-cth-eldritch w-25", id:"btn7", "dashboard" }
        }
    }
}
