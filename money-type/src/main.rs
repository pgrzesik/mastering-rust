use std::ops::Add;

#[derive(Debug, PartialEq)]
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

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        assert_eq!(self.currency, other.currency, "Trying to add different currencies!");

        Money {
            amount: self.amount + other.amount,
            currency: self.currency
        }
    }
}


fn main() {
    let hundred_euro = Money {
        amount: 100,
        currency: Currency::EUR
    };

    let five_hundred_euro = Money {
        amount: 500,
        currency: Currency::EUR
    };

    let six_hundred_euro = hundred_euro + five_hundred_euro;

    println!("Result of adding two EUR money: {:?}", six_hundred_euro);
}
