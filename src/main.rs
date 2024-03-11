use futures_util::StreamExt;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::stream::ShardEventStream;
use twilight_http::Client;
use twilight_model::gateway::Intents;

#[tokio::main]
async fn main() {
    let token = "FILLME".to_owned();

    let client = Client::new(token.clone());

    let mut shards = twilight_gateway::stream::create_recommended(
        &client,
        twilight_gateway::Config::new(token, Intents::empty()),
        |_, builder| builder.build(),
    )
    .await
    .unwrap()
    .collect::<Vec<_>>();

    let mut event_stream = ShardEventStream::new(shards.iter_mut());

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::empty())
        .build();

    while let Some((_, event_res)) = event_stream.next().await {
        match event_res {
            Ok(event) => {
                cache.update(&event);
                tokio::spawn(async move {
                    println!("{event:#?}");
                })
            }
            Err(err) => {
                eprintln!("error receiving event: {err}");

                if err.is_fatal() {
                    eprintln!("received fatal error, exiting");
                    break;
                }

                continue;
            }
        };
    }
}
