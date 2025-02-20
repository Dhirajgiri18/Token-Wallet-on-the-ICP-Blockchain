use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

type AccountId = String;
type Balance = u64;

/// Represents a simple token wallet structure
#[derive(CandidType, Deserialize, Default)]
struct Wallet {
    balances: HashMap<AccountId, Balance>,
}

thread_local! {
    /// Stores the wallet instance and ensures every user starts with 100 tokens
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet {
        balances: HashMap::from([(ic_cdk::caller().to_string(), 100)]),
    });
}

/// Sends tokens from this wallet to another wallet
#[update]
fn send_tokens(amount: Balance, receiver: AccountId) -> String {
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        let sender = ic_cdk::caller().to_string();

        // Prevent self-transfer
        if sender == receiver {
            return "Error: You cannot send tokens to yourself.".to_string();
        }

        // Check sender's balance
        let sender_balance = wallet.balances.entry(sender.clone()).or_insert(100);
        if *sender_balance < amount {
            return format!(
                "Error: Insufficient balance. Your balance is {}, but you tried to send {}.",
                sender_balance, amount
            );
        }

        // Deduct from sender and add to receiver
        *sender_balance -= amount;
        let receiver_balance = wallet.balances.entry(receiver.clone()).or_insert(0);
        *receiver_balance += amount;

        format!("✅ Successfully sent {} tokens to {}", amount, receiver)
    })
}

/// Receives tokens into this wallet
#[update]
fn receive_tokens(amount: Balance) -> String {
    WALLET.with(|wallet| {
        let caller = ic_cdk::caller().to_string();
        let mut wallet = wallet.borrow_mut();
        let user_balance = wallet.balances.entry(caller.clone()).or_insert(0);
        *user_balance += amount;

        format!("✅ Received {} tokens. New balance: {}", amount, user_balance)
    })
}

/// Returns the current balance of the wallet
#[query]
fn get_balance() -> Balance {
    WALLET.with(|wallet| {
        let caller = ic_cdk::caller().to_string();
        let wallet = wallet.borrow();
        *wallet.balances.get(&caller).unwrap_or(&0)
    })
}
