use tokio_core::reactor::Core;
use std::env;
use telegram_bot::{Api, UpdateKind};
use futures::stream::Stream;
use futures::future::Future;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_KEY")
        .expect("TELEGRAM_BOT_KEY not found in environment");

    let api = Api::configure(token)
        .build(core.handle())
        .expect("Failed to spawn bot threads");
    let update_stream = api.stream()
        .map_err(|err| {
            dbg!(&err);
            err
        })
        .for_each(|update| {
            dbg!(&update);
            match update {
                UpdateKind::InlineQuery(InlineQuery) => {

                },
                _ => {}
            }

            Ok(())
        });

    core.run(update_stream).expect("Failed to run react");
}
