use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
enum Currency {
    EUR,
    USD,
    PLN,
    CAD
}

#[derive(Debug)]
struct Money {
    amount: i32,
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

impl Sub for Money {
    type Output = Money;

    fn sub(self, other: Money) -> Money {
        assert_eq!(self.currency, other.currency, "Trying to subtract different currencies");

        Money {
            amount: self.amount - other.amount,
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

    let two_hundred_euro = six_hundred_euro - Money { amount: 400, currency: Currency::EUR };

    println!("Result of subtracting two EUR money: {:?}", two_hundred_euro);
}
