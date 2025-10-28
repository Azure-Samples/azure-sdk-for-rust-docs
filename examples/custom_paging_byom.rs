use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomItem {
    id: String,
    name: String,
    value: i32,
}

// Example demonstrating the concept of pagination with custom types
// Note: Actual Pager implementation requires proper trait implementations
// and integration with Azure service responses

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create some sample data to demonstrate the concepts
    let sample_items = vec![
        CustomItem {
            id: "1".to_string(),
            name: "First Item".to_string(),
            value: 100,
        },
        CustomItem {
            id: "2".to_string(),
            name: "Second Item".to_string(),
            value: 200,
        },
    ];

    println!("Custom paging demonstration:");
    println!("Sample items created: {} items", sample_items.len());

    // Process items to demonstrate the concept
    for item in sample_items {
        println!("Processing item: {} - {}", item.id, item.name);
    }

    println!("\nIn a real Azure SDK implementation:");
    println!("1. Azure services return a Pager<T> for paginated results");
    println!("2. Use .try_next() to iterate through items across pages");
    println!("3. Or use .into_pages() to work with full pages at a time");
    println!("4. The SDK handles continuation tokens automatically");

    Ok(())
}
