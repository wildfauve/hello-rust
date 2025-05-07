enum Payment {
    Cash(f32),               // but we can define that data should be provided...
    CreditCard(String, f32), // Also using a tuple as the data value
    Debitcard(Card),         // or even a struct
    Salt,                    // or just a common enum
    Crypto { wallet_uuid: String, amount: f32 }, // or even a struct like thing, notice the {} instead of the ().
}

struct Card {
    card_number: String,
    expiry_epoch: i32,
}

pub fn play() {
    let cash_payment_type = Payment::Cash(100.00);
    let cc_payment_type = Payment::CreditCard("1111-1111-1111-1111".to_string(), 100.00);
    let dc_payment_type = Payment::Debitcard(Card {
        card_number: "2222-2222".to_string(),
        expiry_epoch: 1,
    });
    let crypto_payment_type = Payment::Crypto {
        wallet_uuid: "123".to_string(),
        amount: 100.00,
    };
    let salt_payment_type = Payment::Salt;
    process_payment(cash_payment_type);
    process_payment(cc_payment_type);
    process_payment(dc_payment_type);
    process_payment(crypto_payment_type);
    process_payment(salt_payment_type);
}

fn process_payment(payment: Payment) {
    match payment {
        Payment::Cash(amt) => println!("Loads of money, or at least {}", amt),
        Payment::CreditCard(number, _amt) => println!("Beholding to the man {}", number),
        Payment::Debitcard(card) => println!("{}, {}", card.card_number, card.expiry_epoch),
        Payment::Crypto {
            wallet_uuid,
            amount,
        } => println!("{} {}", wallet_uuid, amount),
        _ => println!("Just take it"),
    }
}
