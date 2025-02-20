#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        WALLET.with(|wallet| {
            let mut wallet = wallet.borrow_mut();
            wallet.balances.insert("dhiraj".to_string(), 100);
        });

        let result = send_tokens(50, "yogesh".to_string());
        assert_eq!(result, "✅ Successfully sent 50 tokens to bob");

        WALLET.with(|wallet| {
            let wallet = wallet.borrow();
            assert_eq!(*wallet.balances.get("dhiraj").unwrap(), 50);
            assert_eq!(*wallet.balances.get("yogesh").unwrap(), 50);
        });
    }

    #[test]
    fn test_insufficient_balance() {
        let result = send_tokens(200, "dinesh".to_string());
        assert!(result.contains("Error: Insufficient balance"));
    }

    #[test]
    fn test_self_transfer() {
        let result = send_tokens(10, "dhiraj".to_string());
        assert_eq!(result, "Error: You cannot send tokens to yourself.");
    }

    #[test]
    fn test_receive_tokens() {
        let result = receive_tokens(20);
        assert!(result.contains("✅ Received 20 tokens."));
    }
}
