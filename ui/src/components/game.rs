use yew::{function_component, html, Callback, Html};

use crate::{
    components::{GuessBoard, KeyboardInput},
    debug::log,
    state::{
        game::State as GameState, use_game_context, use_wordlist, GameAction,
        GameContext,
    },
};

#[function_component]
pub fn Game() -> Html {
    let wordlist = use_wordlist();
    let GameContext { game, dispatch } = use_game_context();

    let on_input = {
        let game = game.clone();
        Callback::from(move |key: String| {
            if game.state != GameState::Running {
                return;
            }

            let mut word = game.current.to_string();
            match key.as_str() {
                "BKSP" => {
                    word.pop();
                    dispatch.emit(GameAction::SetCurrent(word));
                }
                "ENTER" => {
                    if wordlist.contains(&word) {
                        dispatch.emit(GameAction::AddGuess);
                    } else {
                        log!("DISPLAY NOT A WRD");
                    }
                }
                _ => {
                    if word.len() < game.solution.len() {
                        word.push_str(&key);
                        dispatch.emit(GameAction::SetCurrent(word));
                    }
                }
            }
        })
    };

    html! {
        <div class="game">
            <div class="container">
                <GuessBoard />
            </div>
            <div class="container">
                <KeyboardInput on_input={on_input} />
            </div>
        </div>
    }
}
