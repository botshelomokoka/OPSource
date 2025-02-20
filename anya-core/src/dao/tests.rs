#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_proposal_submission() {
        block_on(async {
            let dao = AnyaDao::new("http://localhost:20443", test_contract_address());
            let proposal = Proposal::new("Test Proposal", "Description");
            let result = dao.submit_proposal(proposal).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_voting() {
        block_on(async {
            let dao = AnyaDao::new("http://localhost:20443", test_contract_address());
            let vote = Vote::new(true, 100);
            let result = dao.vote("test_proposal_id".to_string(), vote).await;
            assert!(result.is_ok());
        });
    }
}