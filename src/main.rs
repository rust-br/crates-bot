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

fn inline_result(crates: Vec<crates_api::Crate>) -> Vec<Box<Serialize>> {
    crates
        .into_iter()
        .map(|each_crate| {
            let crate_name = each_crate.name;
            let crate_doc = each_crate.documentation.unwrap_or("None".into());
            let crate_desc = each_crate.description.unwrap_or("".into());
            let crate_repo = each_crate.repository.unwrap_or("".into());

            let msg_text =
                format!(
                "*Crate*: {}\n*Description*: {}\n*Repository*: {}\n*Doc*: {}",
                &crate_name,
                &crate_desc,
                &crate_repo,
                &crate_doc,
                );
            let input_message_content = InputMessageContent::Text::new(msg_text);
            let inline_resp = InlineQueryResultArticle::new(
                crate_name.clone().into(),
                Box::new(input_message_content),
            );

            Box::new(inline_resp.description(crate_desc)) as Box<Serialize>
        })
        .collect()
}

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
            let result: Vec<Box<Serialize>> = if crates.is_ok() {
                inline_result(crates.unwrap().crates)
            } else {
                println!("Error: {:?}", crates);
                vec![
                    Box::new(InlineQueryResultArticle::new(
                        "Error fetching results".into(),
                        Box::new(InputMessageContent::Text::new(
                            "There was an error querying crates api".into(),
                        )),
                    )),
                ]
            };

            bot.answer_inline_query(query.id, result).send()
        });

    // enter the main loop
    lp.run(stream.for_each(|_| Ok(())).into_future()).unwrap();
}
