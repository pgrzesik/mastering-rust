#[derive(Debug)]
enum Currency {
    EUR,
    USD,
    PLN,
    CAD
}

#[derive(Debug)]
struct Money {
    amount: u32,
    currency: Currency
}


fn main() {
    let euro = Money {
        amount: 100,
        currency: Currency::EUR
    };

    println!("Euro money struct: {:?}", euro);
}
