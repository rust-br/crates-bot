use std::env;
use telegram_bot::types::*;
use telegram_bot::{Api, InlineKeyboardButton};

use futures::StreamExt;

async fn handle_inline_query(
    crates: Vec<crates_bot::Crate>,
    query: InlineQuery,
    api: &Api,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ans = query.answer(vec![]);
    for c in crates {
        let message_text = format!(
                                    "<b>Crate</b>: {}\n<b>Description</b>: {}\n<b>Total downloads</b>: {}, <b>Recent downloads</b>: {}",
                                    &c.name,
                                    &c.description.clone().unwrap_or("".into()),
                                    &c.downloads,
                                    &c.recent_downloads
                                );

        let input_text_message_content = InputTextMessageContent {
            message_text,
            parse_mode: Some(telegram_bot::ParseMode::Html),
            disable_web_page_preview: true,
        };

        let mut article =
            InlineQueryResultArticle::new(c.name.clone(), c.name, input_text_message_content);

        if let Some(description) = c.description {
            article.description(description);
        }

        let mut inline_keyboard_row = vec![];
        if let Some(repository) = c.repository {
            inline_keyboard_row.push(InlineKeyboardButton::url("Repository", &repository));
        }

        if let Some(crates_doc) = c.documentation {
            inline_keyboard_row.push(InlineKeyboardButton::url("Documentation", &crates_doc));
        }

        article.reply_markup(vec![inline_keyboard_row]);
        ans.add_inline_result(article);
    }

    api.spawn(ans);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_KEY").expect("TELEGRAM_BOT_KEY not found in environment");

    let api = Api::new(token);
    let client = reqwest::ClientBuilder::new()
        .user_agent("crates-bot (https://github.com/rust-br/crates-bot)")
        .build()?;

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        match update {
            Ok(Update {
                kind: UpdateKind::InlineQuery(query),
                id: _,
            }) => {
                let crates_bot::Crates { crates } = crates_bot::search(&client, &query.query).await?;
                let _ignore = dbg!("handle result = {}", handle_inline_query(crates, query, &api).await);
            }
            Ok(update_kind) => {
                dbg!("received non supported update_kind = {:?}", update_kind);
            }
            Err(err) => {
                dbg!("update error = {}", err);
            }
        }
    }

    Ok(())
}
