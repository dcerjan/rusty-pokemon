use yew::{function_component, html, Properties};

use crate::model::Pokemon;

#[derive(Clone, Properties, PartialEq)]
pub struct PokemonCardProps {
    pub pokemon: Pokemon,
}

#[function_component(PokemonCard)]
pub fn pokemon_card(props: &PokemonCardProps) -> Html {
    let background_image = format!(
        "background-image: url({})",
        props.pokemon.sprites.front_default
    );
    html! {
        <div class="card">
            <div class="title">
                { props.pokemon.name.to_string() }
            </div>
            <div class="image" style={background_image} />
        </div>
    }
}
