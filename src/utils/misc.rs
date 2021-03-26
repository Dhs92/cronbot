// return u64 since you shouldn't be able to go negative post-tax in BDO
pub fn calc_tax(fame_haven: f32, price: u128, qty: u128) -> u128 {
    let qty = if qty == 0 { 1 } else { qty };

    let raw: u128 = price * qty;
    let post_tax = raw - (raw as f32 * 0.35).round() as u128;
    post_tax + (post_tax as f32 * fame_haven).round() as u128
}
