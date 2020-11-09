use myanimelist_rs::{api::anime, api_objects, api_objects::Anime};
use myanimelist_rs::auth::Auth;
use myanimelist_rs::api_objects::ALL_ANIME_AND_MANGA_FIELDS;
use std::env;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
	AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl,
	StandardTokenResponse, TokenUrl, url::Url, url::ParseError, EmptyExtraTokenFields, basic::BasicTokenType
};

use crate::auth;

//I just wanted to track my anime man why is security so hard

pub fn search_show(title: &str) -> Option<Anime> {
	let q = anime::GetAnimeListQuery {
		q: title.to_owned(),
		limit: 1,
		offset: 0,
		nsfw: true,
		fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_owned()),
	};

	let ret = anime::get_anime_list(q, auth: )

	None
}

async fn get_auth() -> Result</*StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>*/Auth, ParseError>{
	//TODO: Figure out how authentication works
	let redirect_url = "http://localhost/oauth";
	let auth_url = "https://myanimelist.net/v1/oauth2/authorize";
	let token_url = "https://myanimelist.net/v1/oauth2/token";
	let client_id = include_str!("secret"); //if you cloned this repo you'll need to create this

	// let mal_auth

	let auth_client = BasicClient::new(
		ClientId::new(client_id.to_owned()),
		None,
		AuthUrl::new(
			auth_url.to_owned())?,
			Some(TokenUrl::new(token_url.to_owned())?),
		
	)
	.set_redirect_url(RedirectUrl::new(redirect_url.to_owned())?);

	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	let (full_auth_url, csrf_token) = auth_client
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(pkce_challenge)
		.url();

	if let Err(_) = env::var("MAL_CODE"){
		if let Err(_) = myanimelist_rs::auth::open_in_browser(&full_auth_url) {
			println!("Couldn't open the broswer, go here to authenticate: {}", full_auth_url);
		}
	}

	auth::listen_for_code().await;

	let token_result = auth_client
		.exchange_code(AuthorizationCode::new(
			env::var("MAL_CODE").unwrap_or_else(|o| {
				panic!("didn't find the environment variable MAL_CODE: {}", o)
			})
		))
		// Set the PKCE code verifier.
		.set_pkce_verifier(pkce_verifier)
		.add_extra_param("grant_type", "authorization_code")
		.request(http_client);
	
	match token_result{
		Ok(t) => {
			let a 
			return Ok(t)
		},
		Err(e) => panic!("Invalid token: {}", e)
	}
}
