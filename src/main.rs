use std::env;

use anyhow::Result;
use dotenvy::dotenv;
use futures_util::StreamExt;
use tracing::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::stream::ShardEventStream;
use twilight_http::Client;
use twilight_model::gateway::Intents;

struct Config {
    token: String,
}

impl Config {
    fn new() -> Result<Self> {
        dotenv()?;

        Ok(Self {
            token: env::var("TOKEN")?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    LogTracer::init()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::try_from_default_env()?)
        .try_init()?;

    let token = "FILLME".to_owned();

    let client = Client::new(token.clone());

    let mut shards = twilight_gateway::stream::create_recommended(
        &client,
        twilight_gateway::Config::new(token, Intents::empty()),
        |_, builder| builder.build(),
    )
    .await?
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
                    info!(?event, "event received:");
                })
            }
            Err(err) => {
                error!(?err, "error receiving event");

                if err.is_fatal() {
                    error!("received fatal error while receiving event, exiting");
                    break;
                }

                continue;
            }
        };
    }

    Ok(())
}
