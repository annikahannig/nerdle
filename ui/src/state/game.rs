use std::rc::Rc;

use anyhow::Result;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::{
    function_component, hook, html, use_callback, use_context, use_effect_with,
    use_reducer, Callback, Children, ContextProvider, Html, Properties,
    Reducible,
};

use crate::{api::Wordle, state::use_config};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Hint {
    Correct,
    Incorrect,
    Misplaced,
}

impl Hint {
    pub fn to_css_class(&self) -> String {
        match self {
            Hint::Correct => "correct",
            Hint::Incorrect => "incorrect",
            Hint::Misplaced => "misplaced",
        }
        .into()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Guess(String);

impl Guess {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn matches(&self, solution: &str) -> bool {
        self.0.to_uppercase() == solution.to_uppercase()
    }

    pub fn new(value: String) -> Guess {
        Guess(value)
    }

    pub fn hints(&self, solution: &str) -> Vec<Hint> {
        let Self(value) = self;
        let value = value.to_uppercase();
        let solution = solution.to_uppercase();
        value
            .chars()
            .zip(solution.chars())
            .map(|(a, b)| {
                if a == b {
                    Hint::Correct
                } else if solution.contains(a) {
                    Hint::Misplaced
                } else {
                    Hint::Incorrect
                }
            })
            .collect()
    }
}

impl<T> From<T> for Guess
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameAction {
    SetSolution(u32, String),
    SetCurrent(String),
    AddGuess,
    /*
    Clear,
    */
}

pub fn get_keys(prefix: &str) -> Vec<String> {
    let store = LocalStorage::raw();
    let nkeys = store.length().unwrap();
    let mut keys = Vec::new();
    for i in 0..nkeys {
        let key = store.key(i).unwrap();
        if let Some(key) = key {
            if key.starts_with(prefix) {
                keys.push(key);
            }
        }
    }
    keys
}

pub fn get_games() -> Result<Vec<Game>> {
    let keys = get_keys("game:");
    let mut games: Vec<Game> = keys
        .iter()
        .map(|key| {
            let game: Game = LocalStorage::get(key.clone()).unwrap();
            game
        })
        .collect::<Vec<Game>>();
    games.sort_by_key(|game| game.id);
    Ok(games)
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Running,
    Win,
    Loss,
}

impl State {
    pub fn to_css_class(&self) -> String {
        match self {
            State::Running => "running",
            State::Win => "win",
            State::Loss => "loss",
        }
        .into()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub solution: String,
    pub guesses: Vec<Guess>,
    pub current: Guess,
    pub state: State,
}

impl Game {
    pub fn new(solution: &str) -> Self {
        Game {
            solution: solution.into(),
            ..Default::default()
        }
        .update()
    }

    pub fn restore(self) -> Self {
        let key = format!("game:{}", self.id);
        let game = LocalStorage::get(key);
        match game {
            Ok(game) => game,
            Err(_) => self,
        }
    }

    pub fn save(&self) {
        if self.id == 0 {
            return;
        }
        let key = format!("game:{}", self.id);
        LocalStorage::set(key, self).expect("failed to save game");
    }

    pub fn tries(&self) -> usize {
        self.guesses.len()
    }

    pub fn update(self) -> Self {
        let max_tries = 6;
        let mut game = self.clone();

        // Get last guess from history
        let last_guess = game.guesses.last();

        let next_state = {
            match last_guess {
                None => State::Running,
                Some(last_guess) => {
                    match last_guess.matches(&game.solution) {
                        true => State::Win,
                        false => {
                            match game.tries() >= max_tries {
                                true => State::Loss,
                                false => State::Running,
                            }
                        }
                    }
                }
            }
        };
        game.state = next_state;
        game.save();
        game
    }

    pub fn to_share(&self) -> String {
        let text = format!("Nerdle {}/6\n\n", self.tries());
        let text = text
            + &self
                .guesses
                .iter()
                .map(|guess| {
                    let hints = guess.hints(&self.solution);
                    let hints = hints
                        .iter()
                        .map(|hint| match hint {
                            Hint::Correct => "🟩",
                            Hint::Incorrect => "⬛",
                            Hint::Misplaced => "🟧",
                        })
                        .collect::<String>();
                    hints + "\n"
                })
                .collect::<String>();
        text
    }
}

impl Reducible for Game {
    type Action = GameAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            GameAction::SetSolution(id, solution) => {
                let game = Game {
                    id,
                    solution,
                    ..(*self).clone()
                };
                game.restore()
            }
            GameAction::SetCurrent(current) => Game {
                current: current.into(),
                ..(*self).clone()
            },
            GameAction::AddGuess => {
                let mut guesses = (*self).guesses.clone();
                let guess = self.current.clone();
                guesses.push(guess);
                Game {
                    guesses,
                    current: Guess::default(),
                    ..(*self).clone()
                }
            }
        };
        let next_state = next_state.update();
        next_state.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    games: Vec<Game>,
}

impl History {
    pub fn new() -> Self {
        let games = get_games().unwrap();
        Self { games }
    }

    /// Number of games played
    pub fn played(&self) -> usize {
        self.games.iter().filter(|game| game.tries() > 0).count()
    }

    /// Get tries histogram
    pub fn histogram(&self) -> Vec<usize> {
        let mut histogram = vec![0; 7];
        for game in self.games.iter().filter(|game| game.state == State::Win) {
            let tries = game.tries();
            histogram[tries] += 1;
        }
        histogram
    }

    /// Get current streak
    pub fn streak(&self) -> usize {
        let reverse = self.games.iter().rev();
        let streak =
            reverse.take_while(|game| game.state == State::Win).count();
        streak
    }

    /// Get max streak
    pub fn max_streak(&self) -> usize {
        let mut max_streak = 0;
        let mut streak = 0;
        for game in &self.games {
            if game.state == State::Win {
                streak += 1;
                max_streak = max_streak.max(streak);
            } else {
                streak = 0;
            }
        }
        max_streak
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct GameStateProviderProps {
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameContext {
    pub game: Game,
    pub dispatch: Callback<GameAction>,
}

impl GameContext {
    pub fn dispatch(&self, action: GameAction) {
        self.dispatch.emit(action);
    }
}

#[function_component]
pub fn GameStateProvider(props: &GameStateProviderProps) -> Html {
    let GameStateProviderProps { children } = props;
    let config = use_config();
    let game = use_reducer(Game::default);
    let dispatch =
        use_callback(game.clone(), |action, game| game.dispatch(action));

    {
        let config = config.clone();
        let game = game.clone();
        use_effect_with((game, config), |(game, config)| {
            let Wordle { id, solution, .. } = config.wordle.clone();
            game.dispatch(GameAction::SetSolution(id, solution));
        });
    }

    let ctx = GameContext {
        dispatch,
        game: (*game).clone().into(),
    };

    html! {
        <ContextProvider<GameContext> context={ctx}>
            {children}
        </ContextProvider<GameContext>>
    }
}

#[hook]
pub fn use_game_context() -> GameContext {
    use_context::<GameContext>().expect("context required")
}

#[hook]
pub fn use_game_state() -> Game {
    use_game_context().game
}
