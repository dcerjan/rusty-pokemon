use super::Storage;
use crate::model::{Pokemon, PokemonPaginatedResponse};
use gloo::{self, net::http::Request};

pub struct PokeApi;

impl PokeApi {
    pub async fn fetch_pokemon_by_name(name: String) -> Option<Pokemon> {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{name}");
        let url_clone = url.clone();
        let cached = Storage::read::<Pokemon>(url);

        match cached {
            Some(_) => cached,
            None => {
                match Request::get(format!("https://pokeapi.co/api/v2/pokemon/{name}").as_str())
                    .send()
                    .await
                    .unwrap()
                    .json::<Pokemon>()
                    .await
                {
                    Ok(poke) => {
                        Storage::write(url_clone, poke.clone())
                            .expect("Unable to write to local storage");
                        Some(poke)
                    }
                    _ => None,
                }
            }
        }
    }

    pub async fn fetch_pokemon_paginated(limit: i32, offset: i32) -> Vec<Pokemon> {
        let mut pokemon = vec![];

        let response = Request::get(
            format!("https://pokeapi.co/api/v2/pokemon?limit={limit}&offset={offset}").as_str(),
        )
        .send()
        .await
        .unwrap()
        .json::<PokemonPaginatedResponse>()
        .await;

        match response {
            Ok(page) => {
                async {
                    let all_requests = futures::future::join_all(
                        page.results
                            .iter()
                            .map(|item| PokeApi::fetch_pokemon_by_name(item.name.clone())),
                    );

                    let bulk: Vec<Option<Pokemon>> = all_requests.await;

                    pokemon = bulk
                        .iter()
                        .filter_map(|it| it.as_ref())
                        .map(|it| it.to_owned())
                        .collect();
                }
                .await;
                pokemon
            }
            _ => vec![],
        }
    }
}
