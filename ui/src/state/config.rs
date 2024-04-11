use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, hook, html, use_context, use_effect_with, use_state,
    Callback, Children, ContextProvider, Html, Properties,
};

use crate::{api, api::Wordle, debug::log, task::spawn};

#[derive(Properties, Clone, PartialEq)]
pub struct ConfigProviderProps {
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config {
    pub wordle: Wordle,
}

#[function_component]
pub fn ConfigProvider(props: &ConfigProviderProps) -> Html {
    let ConfigProviderProps { children } = props;
    let config = use_state(Config::default);

    {
        let config = config.clone();
        use_effect_with((), move |_| {
            spawn(async move {
                let wordle = api::load_wordle().await?;
                config.set(Config { wordle });
                Ok(())
            });

            || {}
        });
    }

    html! {
        <ContextProvider<Config> context={(*config).clone()}>
            {children}
        </ContextProvider<Config>>
    }
}

#[hook]
pub fn use_config() -> Config {
    use_context::<Config>().expect("ConfigProvider not found")
}
