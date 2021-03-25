use std::error::Error as StdError;

use cronbot::commands::general::*;
use cronbot::utils;
use serenity::{framework::standard::macros::group, prelude::*};

#[group]
#[commands(tax)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    Ok(())
}
