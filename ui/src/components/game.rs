use yew::{function_component, html, Callback, Html, classes, use_state};

use crate::{
    components::{GuessBoard, KeyboardInput},
    state::{
        game::State as GameState, use_game_context, use_wordlist, GameAction,
        GameContext,
    },
};

#[function_component]
pub fn Game() -> Html {
    let wordlist = use_wordlist();
    let GameContext { game, dispatch } = use_game_context();

    // TODO: Bit lazy and hacky.. should be done better
    let gameclass = use_state(|| "".to_string());

    let on_input = {
        let game = game.clone();
        let gameclass = gameclass.clone();
        Callback::from(move |key: String| {
            if game.state != GameState::Running {
                return;
            }

            gameclass.set("".to_string());
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
                        gameclass.set("error-not-a-word".to_string());
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
        <div class={classes!("game", (*gameclass).clone())}>
            <div class="container">
                <GuessBoard />
            </div>
            <div class="container">
                <KeyboardInput on_input={on_input} />
            </div>
        </div>
    }
}
