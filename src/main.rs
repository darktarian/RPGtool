use crate::chthulhu_gen::CthulhuGenAll;
use crate::diceboard::DiceBoard;
use crate::gen_struct::cthulhu_struct::Character;
use crate::page2::Page2;
use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus::desktop::tao::window::Theme;
use dioxus::desktop::tao::window::WindowBuilder;
use dioxus::desktop::{Config, WindowCloseBehaviour};
use dioxus::prelude::*;
use dioxus_desktop::muda::Menu;
use dioxus_desktop::muda::MenuItem;
use dioxus_desktop::muda::PredefinedMenuItem;
use dioxus_desktop::muda::Submenu;
use rusqlite::Connection;
use std::rc::Rc;
mod chthulhu_gen;
mod dice_custom;
mod diceboard;
mod gen_struct;
mod nav_bandeau;
mod page2;
mod pdfprinter;
mod utils;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const DB: Asset = asset!("/assets/cthulhuhack.db");

/*thread_local! {
   pub static DB: RefCell<Connection> = RefCell::new(rusqlite::Connection::open("cthulhuhack.db").expect("Failed to open database"));
}*/

#[derive(Clone, PartialEq, Debug)]
enum CurrentView {
    Dashboard,
    CthulhuGen,
    Page2,
}

#[derive(Clone, Debug)]
struct AppContext {
    connect: Rc<Connection>,
    cthulhu_char: Signal<Character>,
}

fn main() {
    // on détermine un config minimale pour l'app.
    let conf = Config::new()
        .with_disable_context_menu(false)
        .with_window(
            WindowBuilder::new()
                .with_theme(Some(Theme::Dark))
                .with_transparent(false)
                .with_maximizable(true)
                .with_decorations(true)
                .with_resizable(true)
                .with_title("Generator")
                .with_closable(true)
                .with_inner_size(LogicalSize::new(1024.0, 768.0)),
        )
        .with_menu(create_menu())
        .with_close_behaviour(WindowCloseBehaviour::LastWindowExitsApp);
    dioxus::LaunchBuilder::desktop().with_cfg(conf).launch(App);
}

#[component]
fn App() -> Element {
    let app = AppContext {
        connect: Rc::new(
            rusqlite::Connection::open(DB.bundled().absolute_source_path())
                .expect("Failed to open database"),
        ),

        //sans doute need Rc pour le rendre modifiable ou passer par un signal ce qui me semble mieux.
        cthulhu_char: Signal::new(Character::default()),
    };
    use_context_provider(|| app);

    //use_context_provider(|| Rc::new(Character::default()));

    let mut current_view = use_signal(|| CurrentView::Dashboard);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css",
            integrity: "sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH",
            crossorigin: "anonymous"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz",
            crossorigin: "anonymous"
        }
        document::Link{
            href:"https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css",
            rel:"stylesheet",
        }
        //document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        //Bandeau {  }
        //ChackGenerate{}
        div { class:"container-fluid", id:"cont_fuild",
            div { class: "row vh-100",
                //pour le menu vertical
                div {class: "col-2 sidebar text-white d-flex flex-column align-items-stretch",
                    //autour des bouton
                    div { class: "flex-grow-2 d-flex flex-column justify-content-around",
                        button {
                            class: if *current_view.read() == CurrentView::Dashboard { "active btn btn-secondary w-100 my-1 py-1" } else { "inactive btn btn-outline-secondary w-100 my-1 py-1" },
                            onclick: move |_| current_view.set(CurrentView::Dashboard),
                            "Dice board."
                        }
                        button {
                            class: if *current_view.read() == CurrentView::CthulhuGen { " active btn btn-secondary w-100 my-1 py-1" } else { "inactive btn btn-outline-secondary w-100 my-1 py-1" },
                            onclick: move |_| current_view.set(CurrentView::CthulhuGen),
                            "Cthulhu Hack Generator"
                        }
                        button {
                            class: if *current_view.read() == CurrentView::Page2 { " active btn btn-secondary w-100 my-1 py-1" } else { "inactive btn btn-outline-secondary w-100 my-1 py-1" },
                            onclick: move |_| current_view.set(CurrentView::Page2),
                            "W.I.P"
                        }
                    }//fin autour des boutons
                }//fin menu

                //contenu
                div { class: "content-body; col-10 p-3",
                    {render_current_view(current_view.read().clone())}
                }
            }//fin row
        }//cont_fuild

    } //fin rsx
}

fn render_current_view(view: CurrentView) -> Element {
    match view {
        CurrentView::Dashboard => rsx! { DiceBoard {} },
        CurrentView::CthulhuGen => rsx! { CthulhuGenAll {} },
        CurrentView::Page2 => rsx! { Page2 {} },
    }
}

fn create_menu() -> Menu {
    let menu = dioxus_desktop::muda::Menu::new();
    let window_menu = Submenu::new("Window", true);

    window_menu
        .append_items(&[
            &PredefinedMenuItem::fullscreen(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::maximize(None),
            &PredefinedMenuItem::minimize(None),
            &PredefinedMenuItem::close_window(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ])
        .unwrap();
    menu.append_items(&[&window_menu]).unwrap();

    if cfg!(debug_assertions) {
        let help_menu = Submenu::new("Help", true);

        help_menu
            .append_items(&[&MenuItem::with_id(
                "dioxus-toggle-dev-tools",
                "Toggle Developer Tools",
                true,
                None,
            )])
            .unwrap();

        // By default we float the window on top in dev mode, but let the user disable it
        help_menu
            .append_items(&[&MenuItem::with_id(
                "dioxus-float-top",
                "Float on Top (dev mode only)",
                true,
                None,
            )])
            .unwrap();

        _ = menu.append_items(&[&help_menu]);

        #[cfg(target_os = "macos")]
        {
            help_menu.set_as_help_menu_for_nsapp();
        }
    }

    #[cfg(target_os = "macos")]
    {
        window_menu.set_as_windows_menu_for_nsapp();
    }

    menu
}
