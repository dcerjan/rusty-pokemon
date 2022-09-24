use std::{cmp::max, rc::Rc};
use yew::{use_effect_with_deps, use_state, use_state_eq, UseStateHandle};

use crate::{api::PokeApi, model::Pokemon};

#[derive(Clone)]
pub struct UsePaginatedPokemonHandle {
    pub loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
    pub pokemon: UseStateHandle<Vec<Pokemon>>,
    pub page: UseStateHandle<i32>,
    pub next: Rc<dyn Fn()>,
    pub prev: Rc<dyn Fn()>,
}

const PAGE_SIZE: i32 = 10;

pub fn use_paginated_pokemon() -> UsePaginatedPokemonHandle {
    let loading = use_state(|| false);
    let error = use_state(|| None);
    let pokemon = use_state(|| vec![]);
    let page = use_state_eq(|| 0);

    let next = {
        let page = page.clone();
        Rc::new(move || page.set(*page + 1))
    };
    let prev = {
        let page = page.clone();
        Rc::new(move || page.set(max(0, *page - 1)))
    };

    {
        let loading = loading.clone();
        let error = error.clone();
        let pokemon = pokemon.clone();
        let page_clone = page.clone();
        use_effect_with_deps(
            move |_| {
                loading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched =
                        PokeApi::fetch_pokemon_paginated(PAGE_SIZE, *page_clone * PAGE_SIZE).await;
                    pokemon.set(fetched);
                    if pokemon.len() == 0 {
                        error.set(Some("Unable to fetch pokemon".to_string()));
                    } else {
                        error.set(None);
                    }
                    loading.set(false);
                });
                || ()
            },
            *page,
        )
    }

    UsePaginatedPokemonHandle {
        loading,
        error,
        next,
        page,
        pokemon,
        prev,
    }
}

#[derive(Clone)]
pub struct UsePokemonHandle {
    pub loading: UseStateHandle<bool>,
    pub error: UseStateHandle<Option<String>>,
    pub pokemon: UseStateHandle<Option<Pokemon>>,
}

pub fn use_pokemon(name: String) -> UsePokemonHandle {
    let loading = use_state(|| false);
    let error = use_state(|| None);
    let pokemon = use_state(|| None);

    {
        let loading = loading.clone();
        let error = error.clone();
        let pokemon = pokemon.clone();
        let name_for_fetch = name.clone();
        let name_for_error = name.clone();

        use_effect_with_deps(
            move |_| {
                loading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched = PokeApi::fetch_pokemon_by_name(name_for_fetch).await;
                    match fetched {
                        Some(poke) => {
                            pokemon.set(Some(poke));
                            error.set(None);
                        }
                        _ => {
                            pokemon.set(None);
                            error.set(Some(
                                format!("Unable to fetch '{}'", name_for_error).to_string(),
                            ));
                            ()
                        }
                    };
                    loading.set(false);
                });
                || ()
            },
            name,
        )
    }

    UsePokemonHandle {
        loading,
        error,
        pokemon,
    }
}
