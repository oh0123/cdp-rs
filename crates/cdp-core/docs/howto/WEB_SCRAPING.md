# How-To: Web Scraping with CDP-Core

This guide shows practical patterns for extracting data from websites.

## Basic Page Scraping

```rust
use cdp_core::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Navigate to target page
    page.navigate("https://example.com/products").await?;
    
    // Wait for content
    page.wait_for_selector(".product", None).await?;
    
    // Extract data
    let products = page.query_selector_all(".product").await?;
    
    for (i, product) in products.iter().enumerate() {
        let name = product.get_attribute("data-name").await?;
        let price = product.get_attribute("data-price").await?;
        
        println!("Product {}: {:?} - ${:?}", i + 1, name, price);
    }
    
    Ok(())
}
```

## Scraping with Pagination

```rust
async fn scrape_all_pages(initial_url: &str) -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    let mut current_page = 1;
    let mut url = initial_url.to_string();
    
    loop {
        println!("Scraping page {}...", current_page);
        
        // Navigate
        page.navigate(&url).await?;
        page.wait_for_selector(".items", None).await?;
        
        // Extract items
        let items = page.query_selector_all(".item").await?;
        for item in items {
            let title = item.text_content().await?;
            println!("  - {}", title);
        }
        
        // Check for next page
        let next_button = page.query_selector("a.next-page").await?;
        match next_button {
            Some(btn) => {
                if let Some(href) = btn.get_attribute("href").await? {
                    url = href;
                    current_page += 1;
                } else {
                    break;
                }
            }
            None => break,
        }
        
        // Be nice to the server
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Ok(())
}
```

## Handling Dynamic Content

```rust
async fn scrape_spa(url: &str) -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate(url).await?;
    
    // Wait for SPA to load
    page.wait_for_navigation(Some(WaitForNavigationOptions {
        timeout_ms: Some(30000),
        wait_until: Some(WaitUntil::NetworkIdle2),
    })).await?;
    
    // Scroll to load lazy content
    for _ in 0..5 {
        page.evaluate("window.scrollBy(0, window.innerHeight)").await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Now scrape
    let items = page.query_selector_all(".lazy-item").await?;
    println!("Found {} items", items.len());
    
    Ok(())
}
```

## Scraping Tables

```rust
async fn scrape_table(url: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate(url).await?;
    page.wait_for_selector("table", None).await?;
    
    // Get table using JavaScript evaluation
    let result = page.evaluate(r#"
        Array.from(document.querySelectorAll('table tr')).map(row => 
            Array.from(row.querySelectorAll('td, th')).map(cell => cell.textContent.trim())
        )
    "#).await?;
    
    let table: Vec<Vec<String>> = serde_json::from_value(result)?;
    
    // Print table
    for row in &table {
        println!("{}", row.join(" | "));
    }
    
    Ok(table)
}
```

## Handling Authentication

```rust
async fn scrape_authenticated_page() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Login
    page.navigate("https://example.com/login").await?;
    
    if let Some(username_input) = page.query_selector("#username").await? {
        username_input.type_text("myusername").await?;
    }
    
    if let Some(password_input) = page.query_selector("#password").await? {
        password_input.type_text("mypassword").await?;
    }
    
    if let Some(submit_btn) = page.query_selector("button[type='submit']").await? {
        submit_btn.click().await?;
    }
    
    // Wait for login to complete
    page.wait_for_navigation(None).await?;
    
    // Save cookies for future use
    let cookies = page.cookies().await?;
    let cookies_json = serde_json::to_string(&cookies)?;
    std::fs::write("cookies.json", cookies_json)?;
    
    // Now scrape authenticated content
    page.navigate("https://example.com/dashboard").await?;
    // ... scrape ...
    
    Ok(())
}
```

## Scraping with Rate Limiting

```rust
use tokio::time::{sleep, Duration};

struct RateLimiter {
    min_delay_ms: u64,
    last_request: Option<std::time::Instant>,
}

impl RateLimiter {
    fn new(min_delay_ms: u64) -> Self {
        Self {
            min_delay_ms,
            last_request: None,
        }
    }
    
    async fn wait(&mut self) {
        if let Some(last) = self.last_request {
            let elapsed = last.elapsed().as_millis() as u64;
            if elapsed < self.min_delay_ms {
                sleep(Duration::from_millis(self.min_delay_ms - elapsed)).await;
            }
        }
        self.last_request = Some(std::time::Instant::now());
    }
}

async fn scrape_with_rate_limiting() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    let mut rate_limiter = RateLimiter::new(2000);  // 2 seconds between requests
    
    let urls = vec!["https://example.com/page1", "https://example.com/page2"];
    
    for url in urls {
        rate_limiter.wait().await;
        
        page.navigate(url).await?;
        // ... scrape ...
    }
    
    Ok(())
}
```

## Extracting Structured Data

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
    rating: Option<f64>,
    url: String,
}

async fn extract_products(url: &str) -> anyhow::Result<Vec<Product>> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate(url).await?;
    page.wait_for_selector(".product", None).await?;
    
    let product_elements = page.query_selector_all(".product").await?;
    let mut products = Vec::new();
    
    for element in product_elements {
        let name = element
            .get_attribute("data-name").await?
            .unwrap_or_default();
        
        let price_str = element
            .get_attribute("data-price").await?
            .unwrap_or_default();
        let price = price_str.parse::<f64>().unwrap_or(0.0);
        
        let rating = element
            .get_attribute("data-rating").await?
            .and_then(|s| s.parse::<f64>().ok());
        
        let url = element
            .get_attribute("href").await?
            .unwrap_or_default();
        
        products.push(Product {
            name,
            price,
            rating,
            url,
        });
    }
    
    // Save to JSON
    let json = serde_json::to_string_pretty(&products)?;
    std::fs::write("products.json", json)?;
    
    Ok(products)
}
```

## Complete Example with Error Handling

```rust
use cdp_core::Browser;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher()
        .launch()
        .await
        .context("Failed to launch browser")?;
    
    let page = browser.new_page()
        .await
        .context("Failed to create page")?;
    
    // Navigate with error handling
    page.navigate("https://example.com")
        .await
        .context("Failed to navigate")?;
    
    // Wait with timeout
    let element = tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        page.wait_for_selector(".content", None)
    )
    .await
    .context("Timeout waiting for selector")?
    .context("Failed to find selector")?;
    
    // Extract with fallback
    let text = element.text_content().await
        .unwrap_or_else(|_| "N/A".to_string());
    
    println!("Content: {}", text);
    
    Ok(())
}
```

## Tips

1. **Be respectful** - Add delays between requests
2. **Check robots.txt** - Respect site policies
3. **Handle errors** - Sites change, selectors break
4. **Save progress** - Write data incrementally
5. **Use headless mode** - Faster for production

## See Also

- [Network Guide](../features/NETWORK.md) - For interception
- [Wait Functions](../features/WAIT_FUNCTIONS.md) - For dynamic content
- [API Reference](../API_REFERENCE.md)
