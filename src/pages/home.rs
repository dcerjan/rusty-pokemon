use crate::components::Spinner;
use crate::hooks::use_paginated_pokemon;
use gloo;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let pokemon_paginated = use_paginated_pokemon();
    let next = {
        let pokemon_paginated = pokemon_paginated.clone();
        Callback::from(move |_| (pokemon_paginated.next)())
    };
    let prev = {
        let pokemon_paginated = pokemon_paginated.clone();
        Callback::from(move |_| (pokemon_paginated.prev)())
    };

    {
        let pokemon_paginated_clone = pokemon_paginated.clone();
        use_effect_with_deps(
            move |_| {
                gloo::console::log!(*pokemon_paginated_clone.loading);
                || ()
            },
            *pokemon_paginated.loading,
        )
    }

    let list = (*pokemon_paginated.pokemon).iter().map(|pokemon| -> Html {
        html! {
            <a href={format!("/pokemon/{}", pokemon.name)}>{pokemon.name.clone()}</a>
        }
    });

    html! {
        <div>
            if *pokemon_paginated.loading { <Spinner /> }
            <button class="Button" onclick={next}>{ "Next page" }</button>
            <button class="Button" onclick={prev}>{ "Prev page" }</button>
            <div class="list">
                { for list }
            </div>
        </div>
    }
}
