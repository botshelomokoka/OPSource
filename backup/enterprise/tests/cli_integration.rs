#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    #[test]
    fn test_list_transactions() {
        let output = cli.get_matches_from(vec![
            "anya-cli",
            "transact",
            "list"
        ]);
        assert!(output.is_ok());
    }

    #[test]
    fn test_update_transaction_status() {
        let output = cli.get_matches_from(vec![
            "anya-cli",
            "transact",
            "update",
            "--TXID",
            "123",
            "--STATUS",
            "confirmed"
        ]);
        assert!(output.is_ok());
    }
} 