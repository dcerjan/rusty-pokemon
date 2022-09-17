use crate::components::{PokemonCard, Spinner};
use crate::hooks::use_pokemon;
use gloo;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct PokemonProps {
    pub name: String,
}

#[function_component(Pokemon)]
pub fn pokemon(props: &PokemonProps) -> Html {
    let name_clone = props.name.clone();
    let data = use_pokemon(name_clone);
    let maybe_pokemon = data.pokemon.as_ref();

    gloo::console::log!("<>", props.name.clone(), *data.loading.clone());

    let card = {
        match maybe_pokemon {
            Some(p) => {
                html! {
                    <PokemonCard pokemon={p.clone()} />
                }
            }
            _ => html! {},
        }
    };

    html! {
        <div>
            if *data.loading {
                <Spinner />
            } else {
                { card }
            }
        </div>
    }
}
