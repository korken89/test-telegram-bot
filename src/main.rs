use teloxide::{prelude::*, types::Recipient};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     // log::info!("Info: {bot:?} - {msg:?}");
    //     log::info!("Info: {:?}", msg.chat);
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;

    let chat_id = -975989515;
    bot.send_message(Recipient::Id(ChatId(chat_id)), "`Testing` a message")
        .await
        .unwrap();
}
