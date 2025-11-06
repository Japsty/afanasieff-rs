pub mod ops;

use teloxide::{
    Bot,
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Dispatcher,
    types::{Message, Update},
};

use crate::ops::{
    consts::{STREAM_KEYWORD, VINOGRAD_KEYWORD},
    stream::process_stream_msg,
    vinograd::process_vinograd_msg,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the bot...");
    let bot = Bot::from_env();
    let schema = Update::filter_message()
        .branch(
            dptree::entry()
                .filter(|msg: Message| {
                    msg.text()
                        .map_or(false, |t| t.to_lowercase().contains(STREAM_KEYWORD))
                })
                .endpoint(process_stream_msg),
        )
        .branch(
            dptree::entry()
                .filter(|msg: Message| {
                    msg.text()
                        .map_or(false, |t| t.to_lowercase().contains(VINOGRAD_KEYWORD))
                })
                .endpoint(process_vinograd_msg),
        );
    Dispatcher::builder(bot, schema).build().dispatch().await;
}
