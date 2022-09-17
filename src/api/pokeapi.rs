use gloo::{self, net::http::Request};

use crate::model::{Pokemon, PokemonPaginatedResponse};

pub async fn fetch_pokemon_paginated(limit: i32, offset: i32) -> Vec<Pokemon> {
    let mut pokemon = Vec::new();

    match Request::get(
        format!("https://pokeapi.co/api/v2/pokemon?limit={limit}&offset={offset}").as_str(),
    )
    .send()
    .await
    .unwrap()
    .json::<PokemonPaginatedResponse>()
    .await
    {
        Ok(page) => {
            async {
                for item in page.results {
                    match Request::get(
                        format!("https://pokeapi.co/api/v2/pokemon/{}", item.name).as_str(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .json::<Pokemon>()
                    .await
                    {
                        Ok(poke) => pokemon.push(poke),
                        _ => (),
                    }
                }
            }
            .await;
            ()
        }
        _ => (),
    };
    pokemon
}

pub async fn fetch_pokemon_by_name(name: String) -> Option<Pokemon> {
    let mut pokemon = None;

    match Request::get(format!("https://pokeapi.co/api/v2/pokemon/{name}").as_str())
        .send()
        .await
        .unwrap()
        .json::<Pokemon>()
        .await
    {
        Ok(poke) => pokemon = Some(poke),
        _ => (),
    };
    pokemon
}
