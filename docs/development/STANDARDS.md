# OPSource Development Standards

## Code Style

### Rust Style Guide

#### Formatting

- Use `rustfmt` with default settings
- Maximum line length: 100 characters
- Use 4 spaces for indentation
- No trailing whitespace

#### Naming Conventions

```rust
// Types and Traits (PascalCase)
pub struct UserManager {}
pub trait DataProcessor {}

// Functions and Variables (snake_case)
fn process_data() {}
let user_count = 0;

// Constants (SCREAMING_SNAKE_CASE)
const MAX_CONNECTIONS: u32 = 100;

// Modules (snake_case)
mod authentication {}
mod data_processing {}
```

#### Code Organization

```rust
// File structure
use statements
constants
type definitions
trait definitions
implementations
functions
tests

// Example
use std::error::Error;

const MAX_RETRIES: u32 = 3;

pub struct Config {}

pub trait Processor {}

impl Config {}

pub fn process() {}

#[cfg(test)]
mod tests {}
```

### Documentation Standards

#### Code Documentation

```rust
/// Brief description of the function
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// let result = my_function(arg1, arg2);
/// assert!(result.is_ok());
/// ```
pub fn my_function(arg1: Type1, arg2: Type2) -> Result<Output, Error> {
    // Implementation
}
```

#### Module Documentation

```rust
//! # Module Name
//!
//! Brief description of the module
//!
//! ## Features
//!
//! - Feature 1
//! - Feature 2
//!
//! ## Examples
//!
//! ```
//! use module_name::Feature;
//! ```

pub mod feature1 {}
pub mod feature2 {}
```

## Testing Standards

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = process_data(input);

        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_error_handling() {
        // Arrange
        let invalid_input = setup_invalid_data();

        // Act
        let result = process_data(invalid_input);

        // Assert
        assert!(matches!(result, Err(Error::InvalidInput)));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_system_integration() {
    // Setup
    let system = setup_test_system().await;

    // Execute
    let result = system.process_workflow().await;

    // Verify
    assert_workflow_completed(result);
}
```

### Property Tests

```rust
#[test]
fn property_based_test() {
    proptest!(|(input in any::<TestInput>())| {
        let result = process_input(input);
        prop_assert!(validate_output(result));
    });
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
```

### Error Handling Patterns

```rust
// Use Result for fallible operations
fn process_data() -> Result<Output, ServiceError> {
    // Implementation
}

// Use ? operator for error propagation
fn complex_operation() -> Result<Output, ServiceError> {
    let data = fetch_data()?;
    let processed = process_data(data)?;
    Ok(processed)
}

// Provide context for errors
fn operation_with_context() -> Result<Output, ServiceError> {
    process_data().context("Failed during data processing")?
}
```

## Performance Standards

### Database Access

```rust
// Use connection pooling
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(database_url)
    .await?;

// Use prepared statements
let statement = sqlx::query!(
    "SELECT * FROM users WHERE id = $1",
    user_id
);

// Batch operations
let mut tx = pool.begin().await?;
for item in items {
    sqlx::query!("INSERT INTO items (id, value) VALUES ($1, $2)",
        item.id, item.value
    )
    .execute(&mut tx)
    .await?;
}
tx.commit().await?;
```

### Async Operations

```rust
// Use proper async patterns
async fn process_items(items: Vec<Item>) {
    let futures: Vec<_> = items
        .into_iter()
        .map(|item| process_item(item))
        .collect();
    
    join_all(futures).await
}

// Handle cancellation
async fn cancellable_operation() {
    tokio::select! {
        result = operation() => handle_result(result),
        _ = tokio::signal::ctrl_c() => handle_shutdown(),
    }
}
```

## Security Standards

### Input Validation

```rust
// Validate all inputs
fn process_user_input(input: &str) -> Result<ProcessedInput, ValidationError> {
    // Check length
    if input.len() > MAX_INPUT_LENGTH {
        return Err(ValidationError::TooLong);
    }

    // Validate format
    if !INPUT_REGEX.is_match(input) {
        return Err(ValidationError::InvalidFormat);
    }

    // Process valid input
    Ok(ProcessedInput::new(input))
}
```

### Authentication

```rust
// Always use secure authentication
async fn authenticate_user(credentials: Credentials) -> Result<AuthToken, AuthError> {
    // Verify credentials
    let user = verify_credentials(credentials).await?;

    // Generate secure token
    let token = generate_secure_token();

    // Set proper expiration
    set_token_expiration(token, Duration::hours(1));

    Ok(token)
}
```

## Logging Standards

### Log Levels

```rust
// Error: System failures
error!("Database connection failed: {}", error);

// Warn: Recoverable issues
warn!("Rate limit exceeded for user {}", user_id);

// Info: Important events
info!("User {} logged in successfully", user_id);

// Debug: Development information
debug!("Processing batch {} with {} items", batch_id, count);

// Trace: Detailed debugging
trace!("Function entry: args={:?}", args);
```

### Structured Logging

```rust
// Use structured logging
log::info!(
    target: "api",
    event = "request_processed",
    user_id = %user.id,
    duration_ms = %duration.as_millis(),
    status = ?response.status(),
);
```
