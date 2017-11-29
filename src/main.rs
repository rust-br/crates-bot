extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate erased_serde;
extern crate crates_api;

use telebot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use std::env;
use futures::IntoFuture;

use erased_serde::Serialize;

use telebot::functions::*;
use telebot::objects::*;

fn main() {
    // Create a new tokio core
    let mut lp = Core::new().unwrap();

    // Create the bot
    let bot = RcBot::new(lp.handle(), &env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    let stream = bot.get_stream()
        .filter_map(|(bot, msg)| {
            println!("{:?}", msg);
            msg.inline_query.map(|query| (bot, query))
        })
        .and_then(|(bot, query)| {
            let crates = crates_api::query(query.query);
            let result: Vec<Box<Serialize>> = crates
                .crates
                .into_iter()
                .map(|each_crate| {
                    let inline_resp = InlineQueryResultArticle::new(
                        each_crate.name.into(),
                        Box::new(InputMessageContent::Text::new(
                            each_crate.documentation.unwrap_or("None".into()).into(),
                        )),
                    );

                    Box::new(inline_resp.description(
                        each_crate.description.unwrap_or("".into()),
                    )) as Box<Serialize>
                })
                .collect();

            bot.answer_inline_query(query.id, result).send()
        });

    // enter the main loop
    lp.run(stream.for_each(|_| Ok(())).into_future()).unwrap();
}
