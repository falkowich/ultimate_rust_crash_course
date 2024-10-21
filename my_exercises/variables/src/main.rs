const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    let (mut m, r): (i32, i32) = (8, 2);

    println!("m = {}, r = {}", m, r);

    m = 7;

    println!("{}", m - r);
}
