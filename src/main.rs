use rspotify::{
    model::{album, artist, AlbumId, AlbumType, ArtistId, Market},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
    // enabling the `env-file` feature) or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    // export RSPOTIFY_REDIRECT_URI="your redirect uri"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // ```
    // let creds = Credentials {
    //     id: "this-is-my-client-id".to_string(),
    //     secret: Some("this-is-my-client-secret".to_string())
    // };
    // ```
    let creds = Credentials::from_env().unwrap();

    let spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token().await.unwrap();

    let id = "7Eu1txygG6nJttLHbZdQOh";
    let artist_id = ArtistId::from_id(id).unwrap();
    let artist_albums = spotify
        .artist_albums_manual(
            artist_id.clone_static(),
            [AlbumType::Album],
            None,
            None,
            None,
        )
        .await
        .unwrap();
    //let artist_albums_stream = spotify.artist_albums(artist_id, [AlbumType::Album], None);

    //println!("Response: {albums:#?}");
    for item in artist_albums.items.iter() {
        println!("artist album {} id{:?}\n", item.name, item.id);
        let tracks = spotify
            .album_track_manual(item.id.clone().unwrap(), None, None, None)
            .await
            .unwrap();
        for track in tracks.items.iter() {
            println!("tracks{:?}\n", track.name);
            println!("link{:?}\n", track.external_urls);
        }
    }
}
