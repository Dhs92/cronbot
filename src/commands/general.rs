use crate::utils::misc::calc_tax;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use std::error::Error as StdError;

pub struct TaxToken {
    pub(crate) family: f32,
    pub(crate) price: u64,
    pub(crate) qty: u64,
}

// TODO if price is 0, inform user of constraints and stop
#[command]
pub async fn tax(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut total = 0;
    // if args fails to parse into a string, I have no idea what to do..
    for tax in args.iter::<String>().map(|t| t.unwrap()) {
        let tok = match parse_tax(&tax) {
            Ok(tok) => tok,
            Err(_e) => unimplemented!(), // TODO print help message for command
        };
        total += calc_tax(tok.family, tok.price, tok.qty)
    }

    // TODO print `total` to Discord
    log::info!("Tax total: {}", total);

    Ok(())
}

pub fn parse_tax(input: &str) -> Result<TaxToken, Box<dyn StdError>> {
    let mut toks = input.splitn(3, ' ');

    let family = toks.next().unwrap_or("1").parse::<f32>()?;
    let price = toks.next().unwrap_or("0").parse::<u64>()?;
    let qty = toks.next().unwrap_or("1").parse::<u64>()?;

    Ok(TaxToken { family, price, qty })
}
