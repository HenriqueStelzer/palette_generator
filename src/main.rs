mod palette;
mod background;

use leptos::prelude::*;
use palette::*;
use background::*;


fn main() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
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
        init_background(&theme.get());
    });

    view! {
        <main>
            <canvas id="three-d-bg" class="fixed w-100 h-100 inset-0 self-center z-10"></canvas>

            <div id="body" class="fixed w-auto h-auto">

                <div id="navbar">
                    <img type="image/png" sizes="512x512" src="public/android-chrome-512x512.png" />
                    <h1>Palette</h1>
                </div>

                <div id="header">

                </div>

                <div id="examples">

                </div>

                <div id="usage">
                
                </div>

                <div id="footer">

                </div>
            </div>
        </main>
    }
}
