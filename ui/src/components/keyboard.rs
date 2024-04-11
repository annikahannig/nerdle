use std::collections::HashMap;
use yew::{
    classes, functional::function_component, html, Callback, Html, Properties,
};

use crate::state::{use_game_state, Game, Hint};

#[derive(Clone, PartialEq, Default)]
pub enum KeyState {
    #[default]
    Unused,
    Incorrect,
    Correct,
    Misplaced,
}

#[derive(Properties, Clone, PartialEq)]
pub struct KeyboardKeyProps {
    pub glyph: String,
    #[prop_or(KeyState::Unused)]
    pub state: KeyState,

    pub on_click: Callback<String>,
}

#[function_component]
pub fn KeyboardKey(props: &KeyboardKeyProps) -> Html {
    let glyph = props.glyph.clone();
    let state = match props.state {
        KeyState::Unused => "unused",
        KeyState::Incorrect => "incorrect",
        KeyState::Correct => "correct",
        KeyState::Misplaced => "misplaced",
    };
    let on_click = {
        let glyph = glyph.clone();
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            on_click.emit(glyph.clone());
        })
    };

    let id = format!("key-{}", glyph.clone().to_lowercase());

    html! {
        <button
            class={classes!("key", id, state)}
            onclick={on_click}>
            { glyph }
        </button>
    }
}

pub type KeyboardState = HashMap<String, KeyState>;

impl From<&Game> for KeyboardState {
    fn from(game: &Game) -> Self {
        let mut state = Self::default();
        let Game {
            guesses, solution, ..
        } = game;
        // Iterate over all characters in all guesses
        for guess in guesses.iter() {
            let hints = guess.hints(&solution);
            // Iterate over zipped chars with hints
            for (key, hint) in guess.to_string().chars().zip(hints) {
                // If the hint is correct, set the state to correct
                match hint {
                    Hint::Incorrect => {
                        state.insert(key.into(), KeyState::Incorrect);
                    }
                    Hint::Correct => {
                        state.insert(key.into(), KeyState::Correct);
                    }
                    Hint::Misplaced => {
                        state.insert(key.into(), KeyState::Misplaced);
                    }
                }
            }
        }

        state
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct KeyboardProps {
    #[prop_or_default]
    pub on_input: Callback<String>,
}

#[function_component]
pub fn KeyboardInput(props: &KeyboardProps) -> Html {
    let game = use_game_state();

    let rows = vec![
        vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
        vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"],
        vec!["ENTER", "Z", "X", "C", "V", "B", "N", "M", "BKSP"],
    ];

    let state = KeyboardState::from(&game);
    let on_input = props.on_input.clone();

    let on_click = Callback::from(move |glyph: String| {
        on_input.emit(glyph);
    });

    html! {
        <div class="keyboard">
            { for rows.iter().map(|row| {
                html! {
                    <div class="keyboard-row">
                        { for row.iter().map(|glyph| {
                            let glyph = glyph.to_string();
                            let key_state = state.get(&glyph).cloned().unwrap_or_default();
                            html! {
                                <KeyboardKey
                                    on_click={on_click.clone()}
                                    glyph={glyph}
                                    state={key_state} />
                            }
                        }) }
                    </div>
                }
            }) }
        </div>
    }
}
