use yew::{
    function_component, hook, html, use_context, use_effect_with, use_state,
    Children, ContextProvider, Html, Properties,
};

use crate::{api, task::spawn};

#[derive(Clone, PartialEq, Default)]
pub struct Wordlist {
    words: Vec<String>,
}

impl Wordlist {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn contains(&self, word: &str) -> bool {
        let word = word.to_lowercase();
        self.words.contains(&word)
    }
}

impl From<String> for Wordlist {
    fn from(text: String) -> Self {
        let words = text.lines().map(|s| s.to_string()).collect();
        Self { words }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct WordlistProviderProps {
    pub children: Children,
}

#[function_component]
pub fn WordlistProvider(props: &WordlistProviderProps) -> Html {
    let WordlistProviderProps { children } = props;
    let wordlist = use_state(Wordlist::default);
    let is_ready = use_state(|| false);

    {
        let is_ready = is_ready.clone();
        let wordlist = wordlist.clone();
        use_effect_with((), move |_| {
            spawn(async move {
                let words = api::load_wordlist().await?;
                wordlist.set(words);
                is_ready.set(true);
                Ok(())
            });
            || {}
        });
    }

    if !(*is_ready) {
        return html! { "Loading Words..." };
    }

    html! {
        <ContextProvider<Wordlist> context={(*wordlist).clone()}>
            {children}
        </ContextProvider<Wordlist>>
    }
}

#[hook]
pub fn use_wordlist() -> Wordlist {
    use_context::<Wordlist>().expect("wordlist context required")
}
