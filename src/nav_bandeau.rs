use dioxus::prelude::*;

#[component]
pub(crate) fn Bandeau() -> Element {
    rsx! {

        nav {class: "navbar navbar-expand-sm bg-dark rounded-4 border border-light-subtle mb-3",
            div { class:"container-fluid",
                a { class:"navbar-brand", href:"#", "HOME" }
                div { class:"collapse; navbar-collapse", id:"navbarSupportedContent",
                    ul { class:"navbar-nav me-auto mb-1",
                        li { class:"nav-item",
                            a { class:"nav-link active", "aria-current":"page", href:"#", "Home" }
                         }
                     }

             }
            }


        }


    }
}
