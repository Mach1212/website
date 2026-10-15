use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let greeting = use_server_future(hello)?;

    rsx! {
        document::Title { "website" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        main { class: "min-h-screen flex flex-col items-center justify-center gap-4 bg-slate-950 text-slate-100",
            h1 { class: "text-6xl font-bold tracking-tight", "Hello, World!" }
            p { class: "text-lg text-sky-400",
                match &*greeting.read() {
                    Some(Ok(text)) => text.clone(),
                    Some(Err(err)) => format!("Server error: {err}"),
                    None => "Loading…".to_string(),
                }
            }
        }
    }
}

/// Greeting rendered by the server, proving the fullstack round trip.
#[get("/api/hello")]
async fn hello() -> Result<String> {
    Ok("Hello from the Dioxus server!".to_string())
}
