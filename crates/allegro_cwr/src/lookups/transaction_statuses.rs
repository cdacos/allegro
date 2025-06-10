//! Transaction status lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Transaction status mapping
pub static TRANSACTION_STATUSES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("CO", "Conflict");
    m.insert("DU", "Duplicate");
    m.insert("RA", "Transaction Accepted");
    m.insert("AS", "Registration Accepted");
    m.insert("AC", "Registration Accepted with Changes");
    m.insert("RJ", "Rejected");
    m.insert("NP", "No Participation");
    m.insert("RC", "Claim rejected");
    m
});

/// Validates a transaction status code
#[must_use]
pub fn is_valid_transaction_status(code: &str) -> bool {
    TRANSACTION_STATUSES.contains_key(code)
}

/// Gets the description for a transaction status code
#[must_use]
pub fn get_transaction_status_description(code: &str) -> Option<&'static str> {
    TRANSACTION_STATUSES.get(code).copied()
}

/// Gets all valid transaction status codes
#[must_use]
pub fn get_all_transaction_statuses() -> Vec<&'static str> {
    TRANSACTION_STATUSES.keys().copied().collect()
}
