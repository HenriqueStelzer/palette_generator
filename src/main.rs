mod palette;
mod background;

use leptos::prelude::*;
use palette::*;
use background::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    
    let theme: RwSignal<Theme> = RwSignal::new(Theme::new());

    Effect::new(move |_| {
        theme.with(|t| {
            t.active_palette().sync_theme_css()
        });
    });

    Effect::new(move |_| {
        if let Err(e) = init_background() {
            web_sys::console::log_1(&format!("Erro: {:?}", e).into());
        }
    });

    view! {
        <main>
            <canvas id="canvas" class="absolute bg-center"></canvas>
        </main>
    }
}