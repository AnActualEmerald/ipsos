use failure;
use myanimelist_rs::{api::anime, api_objects, api_objects::Anime};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
	AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
	Scope, TokenResponse, TokenUrl,
};
use url::Url;
//I just wanted to track my anime man why is security so hard

pub fn search_show(title: &str) -> Option<Anime> {
	let q = anime::GetAnimeListQuery {
		q: title.to_owned(),
		limit: 1,
		offset: 0,
		nsfw: true,
		fields: None,
	};

	//let ret = anime::get_anime_list(q, auth: &Auth)
}

fn get_auth() {
	//TODO: Figure out how authentication works
	let redirect_url = "http://ipsos/oauth";
	let auth_url = "https://myanimelist.net/v1/oauth2/authorize";
	let token_url = "https://myanimelist.net/v1/oauth2/token";
	let client_id = include_str!("secret"); //if you cloned this repo you'll need to create this

	let auth_client = BasicClient::new(
		ClientId::new(client_id.to_owned()),
		None,
		AuthUrl::new(
			auth_url.to_owned(),
			Some(TokenUrl::new(token_url.to_owned())),
		),
	)
	.set_redirect_url(RedirectUrl::new(redirect_url.to_owned()));

	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	let (auth_url, csrf_token) = auth_client
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(pkce_challenge)
		.url();

	let token_result = client
		.exchange_code(AuthorizationCode::new(
			"some authorization code".to_string(),
		))
		// Set the PKCE code verifier.
		.set_pkce_verifier(pkce_verifier)
		.request(http_client)?;
}
