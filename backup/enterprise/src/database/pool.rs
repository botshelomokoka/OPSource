use sqlx::{Pool, Postgres};
use deadpool_postgres::{Config, Pool as DeadPool, Runtime};
use tokio_postgres::NoTls;

pub struct OptimizedDatabasePool {
    write_pool: Pool<Postgres>,
    read_pool: DeadPool,
    metrics: DatabaseMetrics,
    connection_manager: Arc<ConnectionManager>,
}

impl OptimizedDatabasePool {
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        // Configure write pool with optimal settings
        let write_pool = Pool::builder()
            .max_size(config.write_connections)
            .min_size(config.min_connections)
            .max_lifetime(Some(Duration::from_secs(3600)))
            .idle_timeout(Some(Duration::from_secs(300)))
            .build(&config.database_url)
            .await?;

        // Configure read pool for high throughput
        let mut cfg = Config::new();
        cfg.pool = Some(deadpool::Pool::builder()
            .max_size(config.read_connections)
            .build());

        let read_pool = cfg.create_pool(Runtime::Tokio1)?;

        Ok(Self {
            write_pool,
            read_pool,
            metrics: DatabaseMetrics::new(),
            connection_manager: Arc::new(ConnectionManager::new(config.manager_config)?),
        })
    }

    pub async fn execute_query<T>(
        &self,
        query: &Query<T>,
        context: &QueryContext,
    ) -> Result<T, DatabaseError> {
        let start = Instant::now();
        
        let result = match query.type_ {
            QueryType::Read => {
                self.execute_read_query(query).await?
            },
            QueryType::Write => {
                self.execute_write_query(query).await?
            }
        };

        self.metrics.record_query(start.elapsed(), &query.type_);
        Ok(result)
    }
} 