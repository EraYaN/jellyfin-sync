use cfg_if::cfg_if;

use leptos::*;

use leptos_meta::*;
use leptos_router::*;

pub mod backend;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use lazy_static::lazy_static;
        use std::sync::{Arc, Mutex};

        lazy_static! {
            static ref JELLYFIN : Arc<Mutex<backend::JellyfinApi>> = Arc::new(Mutex::new(backend::JellyfinApi::init("https://jellyfin.e.dehaan.family", "jellyfin-sync/0.1.0")));
        }

        pub fn register_server_functions() {
            _ = GetServerMetadata::register();
        }
    }
}

#[server(GetServerMetadata, "/api")]
pub async fn load_server_data() -> Result<backend::JellyfinServerMetadata, ServerFnError> {
    let mut jellyfin = JELLYFIN.lock().unwrap();
    jellyfin.load_server_data().await.unwrap();
    let metadata = jellyfin.metadata.clone();
    match metadata {
        None => Err(ServerFnError::ServerError("Could not load server metadata...".to_string())),
        Some(data) => Ok(data),
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/jellyfin-sync.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let metadata = create_local_resource(cx, move || (), |_| load_server_data());

    let (_pending, set_pending) = create_signal(cx, false);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <Transition
            fallback=move || view! { cx,  <p>"Loading..."</p> }
            set_pending=set_pending.into()
        >
            {move || match metadata.read(cx) {
                None => None,
                Some(Err(e)) => Some(view! { cx,  <p>"Error loading data."<Error err={e}/></p> }.into_any()),
                Some(Ok(m)) => {
                    Some(view! { cx,
                        <p>"Server Name: "{m.server_name}<br/>
                        "Admin User: "<Uuid id={m.admin_user_id}/></p>
                    }.into_any())
                }
            }}
        </Transition>

    }
}

/// Renders a server error
#[component]
fn Error(cx: Scope, err: ServerFnError) -> impl IntoView {
    let str = format!("{}", err);

    view! { cx,
        <pre>{str}</pre>
    }
}

/// Renders a uuid
#[component]
fn Uuid(cx: Scope, id: uuid::Uuid) -> impl IntoView {
    let id_str = id.to_string();

    view! { cx,
        <pre>{id_str}</pre>
    }
}
