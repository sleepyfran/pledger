use crate::parser::ast::{Amount, CurrencyCode};

/// Transforms a given amount to the specified currency.
pub fn convert_to(currency: &CurrencyCode, origin: &Amount) -> Amount {
    Amount {
        // TODO: Implement actual currency conversion.
        quantity: origin.quantity.clone(),
        currency: currency.clone(),
    }
}
