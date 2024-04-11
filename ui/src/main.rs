use yew::{functional::function_component, html, Callback, Html};

use nerdle_ui::components::Game;
use nerdle_ui::state::{ConfigProvider, GameStateProvider, WordlistProvider};

#[function_component]
fn App() -> Html {
    html! {
        <ConfigProvider>
        <WordlistProvider>
        <GameStateProvider>
            <Game />
        </GameStateProvider>
        </WordlistProvider>
        </ConfigProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
