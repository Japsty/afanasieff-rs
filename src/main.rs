use rand::prelude::*;
use teloxide::{Bot, prelude::Requester, types::Message};

const STREAM_KEYWORD: &str = "стрим";

fn random_quote() -> String {
    let pool = vec![
        "Ты сдохнешь в аду урод".to_string(),
        "Я бы тебе просто по твоей лысине вонючей c пыру въебал".to_string(),
        "Да да нет нет, подумать до завтра есть время у тебя".to_string(),
        "Мы на сво пойдем родину защищать ?".to_string(),
        "слава богу мы живем в россии, а не в америке".to_string(),
        "Да, ракурс делает вещи, в жизни ты бы увидел, писе не поднялась бы".to_string(),
        "Девушка бориса".to_string(),
        "ты 1х1 не хочешь пойти попиздиться? чисто за честь владимира путина".to_string(),
        "Я за люля кебаб на пятьсот Рублевой купюре".to_string(),
        "Я бы вмешался, но мне впн дороже".to_string(),
        "Адекватный человек такое читать не будет".to_string(),
        "Ну я понял гуляш говно потому что напоминает ссср".to_string(),
        "Я брал последнее время пикник и там было 2 разновидности, один обычный, другой какой-то нестандартный, я всегда брал дефолт, решил попробовать экзотику, я взял, попробовал и пхуел, я такого говна не пробовал никогда, это видимо от какого-то местного производителя".to_string(),
        "Это 90 айкью юмор я Даун".to_string(),
        "Да, давай с тобой подеремся, если я побеждаю, я тебя из квартиры выписываю, а если ты, я просто заплачу и домой пойду".to_string(),
    ];
    let mut rng = rand::rng();
    match pool.choose(&mut rng) {
        Some(quote) => quote.clone(),
        None => panic!("Can't find quote"),
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the bot...");
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let random_quote = random_quote();
        match msg.text() {
            Some(txt) => {
                if txt.to_lowercase().contains(STREAM_KEYWORD) {
                    let _ = bot.send_message(msg.chat.id, random_quote).await;
                }
            }
            None => log::error!("Can't send quote..."),
        }
        Ok(())
    })
    .await
}
