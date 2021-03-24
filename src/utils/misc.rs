// return u64 since you shouldn't be able to go negative post-tax in BDO
pub fn calc_tax(fame_haven: f32, price: u64, qty: u64) -> u64 {
    let qty = if qty == 0 { 1 } else { qty };

    let raw: u64 = price * qty;
    let post_tax = raw - (raw as f32 * 0.35).round() as u64;
    post_tax + (post_tax as f32 * fame_haven).round() as u64
}
