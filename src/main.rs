use tokio_core::reactor::Core;
use std::env;
use telegram_bot::{Api, UpdateKind};
use telegram_bot::types::{InlineQueryResultArticle, InputTextMessageContent};
use telegram_bot::prelude::*;
use futures::stream::Stream;

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
            match update.kind {
                UpdateKind::InlineQuery(query) => {
                    dbg!(&query);
                    let user_query = query.query.clone();
                    let mut ans = query.answer(vec![]);
                    ans.add_inline_result(InlineQueryResultArticle::new(
                        "id".into(),
                        "title".into(),
                        InputTextMessageContent {
                            message_text: user_query,
                            parse_mode: None,
                            disable_web_page_preview: true
                        }.into()
                    ));
                    api.spawn(ans);
                },
                _ => {}
            }

            Ok(())
        });

    core.run(update_stream).expect("Failed to run react");
}
