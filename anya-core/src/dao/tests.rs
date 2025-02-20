#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    Add unit tests for AnyaDao methods using mock DAO client

    - Added tests for proposal submission, voting, proposal status retrieval, proposal execution, and voting power retrieval.
    - Utilized `mockall` to create a mock `DaoClient` for testing.
    - Used `tokio_test::block_on` to run async tests.
    - Ensured each test sets up expectations and verifies the results correctly.
    use tokio_test::block_on;

    // Mock DAO client for testing
    mock! {
        DaoClient {
            fn submit_proposal(&self, proposal: Proposal) -> Result<String, Error>;
            fn vote(&self, proposal_id: String, vote: Vote) -> Result<(), Error>;
            fn get_proposal_status(&self, proposal_id: String) -> Result<ProposalStatus, Error>;
            fn execute_proposal(&self, proposal_id: String) -> Result<(), Error>;
            fn get_voting_power(&self, address: String) -> Result<u64, Error>;
        }
    }

    fn setup_test_dao() -> (MockDaoClient, AnyaDao) {
        let mock_client = MockDaoClient::new();
        let dao = AnyaDao::new_with_client(mock_client.clone());
        (mock_client, dao)
    }

    #[test]
    fn test_proposal_submission() {
        block_on(async {
            let (mut mock_client, dao) = setup_test_dao();
            
            // Setup expectations
            mock_client
                .expect_submit_proposal()
                .with(predicate::always())
                .times(1)
                .returning(|_| Ok("test_id".to_string()));

            let proposal = Proposal::new(
                "Test Proposal".to_string(),
                "Description".to_string(),
            );

            let result = dao.submit_proposal(proposal).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "test_id");
        });
    }

    #[test]
    fn test_voting() {
        block_on(async {
            let (mut mock_client, dao) = setup_test_dao();
            
            // Setup expectations
            mock_client
                .expect_vote()
                .with(predicate::eq("test_proposal_id".to_string()), predicate::always())
                .times(1)
                .returning(|_, _| Ok(()));

            let vote = Vote::new(true, 100);
            let result = dao.vote("test_proposal_id".to_string(), vote).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_proposal_status() {
        block_on(async {
            let (mut mock_client, dao) = setup_test_dao();
            
            // Setup expectations
            mock_client
                .expect_get_proposal_status()
                .with(predicate::eq("test_proposal_id".to_string()))
                .times(1)
                .returning(|_| Ok(ProposalStatus::Active));

            let status = dao.get_proposal_status("test_proposal_id".to_string()).await;
            assert!(status.is_ok());
            assert_eq!(status.unwrap(), ProposalStatus::Active);
        });
    }

    #[test]
    fn test_execute_proposal() {
        block_on(async {
            let (mut mock_client, dao) = setup_test_dao();
            
            // Setup expectations
            mock_client
                .expect_execute_proposal()
                .with(predicate::eq("test_proposal_id".to_string()))
                .times(1)
                .returning(|_| Ok(()));

            let result = dao.execute_proposal("test_proposal_id".to_string()).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_get_voting_power() {
        block_on(async {
            let (mut mock_client, dao) = setup_test_dao();
            
            // Setup expectations
            mock_client
                .expect_get_voting_power()
                .with(predicate::eq("test_address".to_string()))
                .times(1)
                .returning(|_| Ok(100));

            let result = dao.get_voting_power("test_address".to_string()).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 100);
        });
    }
}