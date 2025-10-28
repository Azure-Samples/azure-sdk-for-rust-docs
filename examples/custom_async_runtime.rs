// Example: Demonstrating the concept of custom async runtime integration
// The Azure SDK allows using different async runtimes (tokio, async-std, etc.)
// through the AsyncRuntime trait

// Note: This is a conceptual example showing the pattern
// In practice, you would implement the full AsyncRuntime trait

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Custom Async Runtime Example");
    println!("============================");
    println!();
    println!(
        "The Azure SDK for Rust supports custom async runtimes through the AsyncRuntime trait."
    );
    println!("This allows you to use different async executors like:");
    println!("  - Tokio (most common)");
    println!("  - async-std");
    println!("  - smol");
    println!("  - or your own custom runtime");
    println!();
    println!("To implement a custom runtime:");
    println!("  1. Implement the AsyncRuntime trait");
    println!("  2. Provide spawn() and sleep() implementations");
    println!("  3. Call set_async_runtime() at application startup");
    println!();

    // Test the runtime with a simple sleep
    let start = std::time::Instant::now();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let elapsed = start.elapsed();

    println!("Sleep completed in {:?}", elapsed);
    println!("This example is running on the Tokio runtime");

    Ok(())
}
