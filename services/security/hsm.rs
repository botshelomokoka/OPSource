use cron::Schedule;
use std::str::FromStr;

impl HsmKeyManager {
    pub async fn start_rotation_scheduler(&self) {
        let schedule = Schedule::from_str("0 0 0 */90 * *").unwrap(); // Every 90 days
        let mut scheduler = JobScheduler::new().await.unwrap();
        
        scheduler.add(Job::new(schedule, |_| {
            let new_key = self.generate_new_key().await;
            self.rotate_application_keys(new_key).await;
        })).await.unwrap();
        
        scheduler.start().await.unwrap();
    }
    
    async fn generate_new_key(&self) -> HsmKey {
        self.client.generate_key("ECC_SECP256K1")
            .await
            .expect("HSM key generation failed")
    }
} 