use moka::future::Cache;
use tokio::sync::RwLock;

pub struct CacheManager {
    transaction_cache: Cache<String, Transaction>,
    block_cache: Cache<String, Block>,
    connection_pool: Pool<Postgres>,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            transaction_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600))
                .build(),
            block_cache: Cache::builder()
                .time_to_live(Duration::from_secs(7200))
                .build(),
            connection_pool: PgPoolOptions::new()
                .max_connections(32)
                .connect_timeout(Duration::from_secs(30))
                .build(&config.database_url)
                .await?,
        }
    }
    
    pub async fn get_transaction(&self, txid: &str) -> Result<Option<Transaction>, CacheError> {
        if let Some(tx) = self.transaction_cache.get(txid).await {
            return Ok(Some(tx));
        }
        
        // Fetch from database if not in cache
        if let Some(tx) = self.fetch_from_db(txid).await? {
            self.transaction_cache.insert(txid.to_string(), tx.clone()).await;
            Ok(Some(tx))
        } else {
            Ok(None)
        }
    }
} 