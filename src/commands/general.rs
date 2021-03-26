use crate::utils::misc::calc_tax;
use serenity::{client::Context, framework::standard::{macros::command, Args, CommandResult}, http::CacheHttp, model::channel::Message};
use std::error::Error as StdError;
use num_format::{Locale, ToFormattedString};

pub struct TaxToken {
    pub(crate) family: f32,
    pub(crate) price: u128,
    pub(crate) qty: u128,
}

// TODO if price is 0, inform user of constraints and stop
#[command]
pub async fn tax(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut total = 0;
    // if args fails to parse into a string, I have no idea what to do..
    for tax in args.iter::<String>().map(|t| t.unwrap()) {
        println!("Token: {}", tax);
        let tax = tax.replace(",", "");
        let tok = match parse_tax(&tax) {
            Ok(tok) => tok,
            Err(_e) => unimplemented!(), // TODO print help message for command
        };
        total += calc_tax(tok.family, tok.price, tok.qty)
    }

    // TODO print `total` to Discord
    log::info!("Tax total: {}", total);
    println!("Tax total: {}", total);

    msg.channel_id.say(&ctx.http(), total.to_formatted_string(&Locale::en)).await?;

    Ok(())
}

pub fn parse_tax(input: &str) -> Result<TaxToken, Box<dyn StdError>> {
    let mut toks = input.splitn(3, ' ');

    let family = toks.next().unwrap_or("1").parse::<f32>()?;
    let price = toks.next().unwrap_or("0").parse::<u128>()?;
    let qty = toks.next().unwrap_or("1").parse::<u128>()?;

    Ok(TaxToken { family, price, qty })
}
