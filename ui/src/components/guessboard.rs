use gloo::dialogs::alert;
use yew::{
    classes, function_component, html, use_callback, use_effect_with,
    use_state, Callback, Html, Properties,
};

use crate::state::game::{
    use_game_state, Game, Guess, History as GameHistory, State as GameState,
};

#[derive(Properties, Clone, PartialEq)]
pub struct HistogramProps {
    pub histogram: Vec<usize>,
    pub total: usize,
}

#[function_component]
pub fn Histogram(props: &HistogramProps) -> Html {
    let HistogramProps { total, histogram } = props;
    let buckets = histogram
        .iter()
        .zip(0..)
        .skip(1)
        .map(|(count, tries)| {
            let perc = (*count as f32 / *total as f32) * 100.0;
            html! {
                <div class="bucket">
                    <div class="bar">
                        <div style={format!("width: {}%", perc)}>{tries}</div>
                    </div>
                    <span>{ count }</span>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="buckets">
            {buckets}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct StatsProps {
    #[prop_or_default]
    pub visible: bool,
}

#[function_component]
pub fn Stats(props: &StatsProps) -> Html {
    let StatsProps { visible } = props;
    let game = use_game_state();
    let history = GameHistory::new();

    let title = match game.state {
        GameState::Win => "Congratulations!",
        GameState::Loss => "You lost!",
        GameState::Running => "Good luck!",
    };

    let cls = match visible {
        true => vec!["stats", "visible"],
        false => vec!["stats"],
    };

    let streak = history.streak();
    let max_streak = history.max_streak();
    let played = history.played();
    let histogram = history.histogram();

    let on_share = use_callback(game, move |_, game| {
        let clipboard = web_sys::window()
            .unwrap()
            .navigator()
            .clipboard()
            .expect("clipboard required");
        let share = game.to_share();
        let _ = clipboard.write_text(&share);
        alert("Copied to clipboard!");
    });

    html! {
        <div class={classes!(cls)}>
            <div class="rows">
                <div class="row">
                    <h1>{ title }</h1>
                </div>
                <div class="row">
                    <div class="stat">
                        <b>{ "Played" }</b>
                        <span>{ played }</span>
                    </div>
                    <div class="stat">
                        <b>{ "Current Streak" }</b>
                        <span>{ streak }</span>
                    </div>
                    <div class="stat">
                        <b>{ "Max Streak" }</b>
                        <span>{ max_streak }</span>
                    </div>
                </div>
                <div class="row histogram">
                    <Histogram total={played} histogram={histogram} />
                </div>
                <div class="row share">
                    <button onclick={on_share}>{ "Share" }</button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct GuessViewProps {
    #[prop_or_default]
    pub guess: Guess,
}

#[function_component]
pub fn GuessView(props: &GuessViewProps) -> Html {
    let Game { solution, .. } = use_game_state();
    let GuessViewProps { guess } = props;
    let hints = guess.hints(&solution);

    let char_count = guess.to_string().chars().take(5).count();

    // Zip chars with hints
    let chars = guess
        .to_string()
        .chars()
        .zip(
            hints
                .iter()
                .map(|h| h.to_css_class())
                .collect::<Vec<String>>(),
        )
        .map(|(c, hint)| {
            html! {
                <span class={hint}>{c}</span>
            }
        })
        .collect::<Html>();

    let pad = (0..(5 - char_count))
        .map(|_| html! { <span class="pad"></span> })
        .collect::<Html>();

    html! {
        <div class="guess">
            { chars }
            { pad }
        </div>
    }
}

#[function_component]
pub fn GuessBoard() -> Html {
    let show_stats = use_state(|| false);
    let Game {
        state,
        guesses,
        current,
        ..
    } = use_game_state();
    let max_guesses = 6;
    let pad = if guesses.len() < max_guesses {
        (0..(max_guesses - guesses.len() - 1))
            .map(|_| html! { <GuessView /> })
            .collect::<Html>()
    } else {
        html! {}
    };

    let on_click_stats = {
        let show_stats = show_stats.clone();
        Callback::from(move |_| show_stats.set(!*show_stats))
    };

    {
        let show_stats = show_stats.clone();
        let state = state.clone();
        use_effect_with(state, move |state| match state {
            GameState::Running => {
                show_stats.set(false);
            }
            _ => {
                show_stats.set(true);
            }
        });
    }

    let stats_cls = match *show_stats {
        true => "stats",
        false => "",
    };

    html! {
        <div class="main">
            <div class="tabs">
                <button onclick={on_click_stats}>{ "Statistics" }</button>
            </div>
        <div class={classes!("guess-board", state.to_css_class(), stats_cls)}>
            <Stats visible={*show_stats} />
            <div class="guesses">
                <div class="history">
                    { for guesses.iter().map(|guess| html! {
                        <GuessView key={guess.to_string()} guess={guess.clone()} /> }) }
                </div>
                <div class="current">
                if state != GameState::Loss {
                    <GuessView guess={current.clone()} />
                }
                </div>
                <div class="padding">
                    { pad }
                </div>
            </div>
        </div>
        </div>
    }
}
