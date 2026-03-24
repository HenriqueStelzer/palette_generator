mod background;
mod palette;

use background::*;
use leptos::prelude::*;
use palette::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let theme: RwSignal<Theme> = RwSignal::new(Theme::generate(240.0, 0.05, 0.25, 0.5));

    theme.update(|t| t.toggle_mode());

    Effect::new(move |_| {
        theme.with(|t| t.active_palette().sync_theme_css());
    });

    Effect::new(move |_| {
        init_background(&theme.get_untracked());
    });

    view! {
        <main>
            <canvas id="Three-d-bg" class="fixed w-full h-full inset-0 z-0"></canvas>
            <span id="Background" class="fixed inset-0 bg-surface-bg/75 z-10"></span>


            <div id="Body" class="relative z-20 md:subpixel-antialiased">
                <div id="Navbar">
                    <nav class="top-0 w-full z-30">
                        <div class="main-surface h-24 justify-content-start">
                        <a href="/" class="text-surface-main bg-accent-1 font-semibold font-mono text-xl p-2 -m-2 rounded-lg border-2 border-accent-1">Palette</a>
                        <a href="#Examples" class="text-subheading text-accent-1">Examples</a>
                        <a href="#Usage" class="text-subheading text-accent-1">Usage</a>
                        <a href="#About" class="text-subheading text-accent-1">About</a>
                        </div>
                    </nav>
                </div>

                <div id="Header">
                    <div class="main-surface h-120">

                    </div>
                </div>

                <div id="Examples">
                    <div class="flex gap-1 w-full">
                        <div class="main-surface h-160 w-1/2">

                        </div>
                        <div class="w-1/2">
                            <div class="main-surface h-60">

                            </div>
                            <div class="main-surface h-94">

                            </div>
                        </div>
                    </div>
                </div>

                <div id="Usage">
                    <div class="main-surface h-160">

                    </div>
                </div>

                <div id="About">
                    <div class="main-surface h-80">

                    </div>
                </div>

                <div id="Footer">
                    <div class="main-surface h-36">

                    </div>
                </div>
            </div>
        </main>
    }
}
