use crate::error::Result;
use crate::ryot;
use reqwest::{header, Client, RequestBuilder};
use std::env;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProviderIds {
    pub tvdb: Option<String>,
    pub imdb: Option<String>,
    pub tmdb: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub played: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub name: String,
    pub id: String,
    pub provider_ids: Option<ProviderIds>,
    pub index_number: Option<u32>,
    pub user_data: Option<UserData>,
}

pub struct Jellyfin {
    client: Client,
    base_url: String,
    user_id: String,
    tv_library_id: String,
}

impl Jellyfin {
    pub fn new() -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        let api_token = env::var("JF_API_KEY")?;
        let mut auth_header = header::HeaderValue::from_str(&api_token)?;
        let accept = "application/json";
        let accept_header = header::HeaderValue::from_str(accept)?;

        auth_header.set_sensitive(true);
        headers.insert("X-Emby-Token", auth_header);
        headers.insert(header::ACCEPT, accept_header);

        let client = Client::builder().default_headers(headers).build()?;

        let base_url = env::var("JF_BASE_URL")?;
        let user_id = env::var("JF_USER_ID")?;
        let tv_library_id = env::var("JF_TV_LIBRARY_ID")?;

        Ok(Self {
            base_url,
            client,
            user_id,
            tv_library_id,
        })
    }

    fn get(&self, path: &str) -> Result<RequestBuilder> {
        let url = format!("{}{}", &self.base_url, path);
        let req = self.client.request(reqwest::Method::GET, url);

        Ok(req)
    }

    pub async fn get_items(&self, parent_id: Option<&str>) -> Result<Items> {
        let parent_id = match parent_id {
            Some(id) => id,
            None => &self.tv_library_id,
        };

        let resp = self
            .get(format!("/Users/{}/Items", &self.user_id).as_str())?
            .query(&[("fields", "ProviderIds"), ("parentId", parent_id)])
            .send()
            .await?
            .json::<Items>()
            .await?;

        Ok(resp)
    }

    pub async fn get_ryot_shows_json(&self) -> Result<()> {
        let shows = self.get_items(None).await?;
        let mut ryot_items: Vec<ryot::Item> = Vec::new();

        for show in shows.items.iter() {
            let tmdb_id = match &show.provider_ids {
                Some(ProviderIds {
                    tmdb: Some(tmdb), ..
                }) => tmdb,
                _ => continue,
            };

            let mut seen_history: Vec<ryot::SeenHistory> = Vec::new();

            let seasons = self.get_items(Some(&show.id)).await?;

            for season in seasons.items.iter() {
                let show_season_number = match season.index_number {
                    Some(i) => i,
                    None => continue,
                };

                let episodes = self.get_items(Some(&season.id)).await?;

                for episode in episodes.items.iter() {
                    match episode.user_data {
                        Some(UserData { played: true }) => (),
                        _ => continue,
                    }

                    let show_episode_number = match episode.index_number {
                        Some(i) => i,
                        None => continue,
                    };

                    seen_history.push(ryot::SeenHistory {
                        show_episode_number: Some(show_episode_number),
                        show_season_number: Some(show_season_number),
                    })
                }
            }

            let ryot_show = ryot::Item {
                identifier: tmdb_id.to_string(),
                collections: vec![],
                lot: "Show".to_string(),
                reviews: vec![],
                seen_history,
                source: "Tmdb".to_string(),
                source_id: show.id.to_string(),
            };

            ryot_items.push(ryot_show);
        }

        let json = serde_json::to_string(&ryot_items)?;
        println!("{}", json);

        Ok(())
    }

    pub async fn get_ryot_movies_json(&self) -> Result<()> {
        let movie_library_id = env::var("JF_MOVIE_LIBRARY_ID")?;
        let movies = self.get_items(Some(&movie_library_id)).await?;
        let mut ryot_items: Vec<ryot::Item> = Vec::new();

        for movie in movies.items.iter() {
            let tmdb_id = match &movie.provider_ids {
                Some(ProviderIds {
                    tmdb: Some(tmdb), ..
                }) => tmdb,
                _ => continue,
            };

            match movie.user_data {
                Some(UserData { played: true }) => (),
                _ => continue,
            }

            let seen_history: Vec<ryot::SeenHistory> = vec![ryot::SeenHistory {
                show_episode_number: None,
                show_season_number: None,
            }];

            let ryot_movie = ryot::Item {
                identifier: tmdb_id.to_string(),
                collections: vec![],
                lot: "Movie".to_string(),
                reviews: vec![],
                seen_history,
                source: "Tmdb".to_string(),
                source_id: movie.id.to_string(),
            };

            ryot_items.push(ryot_movie);
        }

        let json = serde_json::to_string(&ryot_items)?;
        println!("{}", json);

        Ok(())
    }
}
