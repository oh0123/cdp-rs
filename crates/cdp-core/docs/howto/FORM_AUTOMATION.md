# How-To: Form Automation

This guide demonstrates automating web forms with CDP-Core.

## Basic Form Filling

```rust
use cdp_core::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Navigate to form
    page.navigate("https://example.com/contact").await?;
    
    // Wait for form to load
    page.wait_for_selector("form", None).await?;
    
    // Fill text inputs
    if let Some(name_input) = page.query_selector("#name").await? {
        name_input.type_text("John Doe").await?;
    }
    
    if let Some(email_input) = page.query_selector("#email").await? {
        email_input.type_text("john@example.com").await?;
    }
    
    // Fill textarea
    if let Some(message) = page.query_selector("textarea#message").await? {
        message.type_text("This is a test message.").await?;
    }
    
    // Submit form
    if let Some(submit_btn) = page.query_selector("button[type='submit']").await? {
        submit_btn.click().await?;
    }
    
    // Wait for success message
    page.wait_for_selector(".success-message", None).await?;
    
    Ok(())
}
```

## Handling Different Input Types

```rust
async fn fill_complex_form() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate("https://example.com/form").await?;
    page.wait_for_selector("form", None).await?;
    
    // Text input
    if let Some(input) = page.query_selector("input[type='text']").await? {
        input.type_text("value").await?;
    }
    
    // Email input
    if let Some(email) = page.query_selector("input[type='email']").await? {
        email.type_text("test@example.com").await?;
    }
    
    // Password input
    if let Some(password) = page.query_selector("input[type='password']").await? {
        password.type_text_with_delay("secret123", 50, 100).await?;  // Human-like
    }
    
    // Number input
    if let Some(number) = page.query_selector("input[type='number']").await? {
        number.focus().await?;
        number.type_text("42").await?;
    }
    
    // Checkbox
    page.evaluate(r#"
        document.querySelector('#terms').checked = true;
    "#).await?;
    
    // Radio button
    page.evaluate(r#"
        document.querySelector('input[value="option1"]').click();
    "#).await?;
    
    // Select dropdown
    page.evaluate(r#"
        document.querySelector('select#country').value = 'US';
    "#).await?;
    
    Ok(())
}
```

## File Upload

```rust
async fn upload_file() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate("https://example.com/upload").await?;
    
    // Set files to input element
    let file_input = page.query_selector("input[type='file']").await?
        .ok_or(anyhow!("File input not found"))?;
    
    // Use CDP to set files
    page.evaluate(r#"
        const input = document.querySelector('input[type="file"]');
        const dataTransfer = new DataTransfer();
        // Note: Actual file upload requires CDP's Page.setFileInputFiles
    "#).await?;
    
   // Alternative: Use element handle to set files via CDP
    // This requires direct CDP protocol access
    
    Ok(())
}
```

## Dynamic Forms

```rust
async fn handle_dynamic_form() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate("https://example.com/wizard").await?;
    
    // Step 1
    page.wait_for_selector("#step1", None).await?;
    
    if let Some(input) = page.query_selector("#name").await? {
        input.type_text("John").await?;
    }
    
    if let Some(next_btn) = page.query_selector("#next").await? {
        next_btn.click().await?;
    }
    
    // Wait for step 2
    page.wait_for_selector("#step2", None).await?;
    
    if let Some(input) = page.query_selector("#email").await? {
        input.type_text("john@example.com").await?;
    }
    
    if let Some(next_btn) = page.query_selector("#next").await? {
        next_btn.click().await?;
    }
    
    // Step 3...
    page.wait_for_selector("#step3", None).await?;
    
    Ok(())
}
```

## Form Validation Handling

```rust
async fn handle_validation() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate("https://example.com/form").await?;
    
    // Try to submit empty form
    if let Some(submit_btn) = page.query_selector("button[type='submit']").await? {
        submit_btn.click().await?;
    }
    
    // Wait for validation errors
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Check for error messages
    let errors = page.query_selector_all(".error-message").await?;
    
    if !errors.is_empty() {
        println!("Validation errors found:");
        for error in errors {
            let text = error.text_content().await?;
            println!("  - {}", text);
        }
        
        // Fix errors
        if let Some(email) = page.query_selector("#email").await? {
            email.focus().await?;
            // Clear existing value
            page.keyboard().down("Control").await?;
            page.keyboard().press_key("a").await?;
            page.keyboard().up("Control").await?;
            // Type correct value
            email.type_text("valid@example.com").await?;
        }
        
        // Retry submit
        if let Some(submit_btn) = page.query_selector("button[type='submit']").await? {
            submit_btn.click().await?;
        }
    }
    
    Ok(())
}
```

## CAPTCHA Handling

```rust
async fn handle_captcha() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate("https://example.com/form-with-captcha").await?;
    
    // Fill form
    if let Some(name) = page.query_selector("#name").await? {
        name.type_text("John Doe").await?;
    }
    
    // Check for CAPTCHA
    if let Some(_captcha) = page.query_selector(".g-recaptcha").await? {
        println!("CAPTCHA detected!");
        
        // Option 1: Wait for user to solve manually
        println!("Please solve the CAPTCHA...");
        page.wait_for_function(
            r#"() => !!document.querySelector("#g-recaptcha-response").value"#,
            Some(120000),  // 2 minute timeout
            None
        ).await?;
        
        // Option 2: Use CAPTCHA solving service (implemented separately)
        // solve_captcha(&page).await?;
    }
    
    // Submit after CAPTCHA is solved
    if let Some(submit) = page.query_selector("button[type='submit']").await? {
        submit.click().await?;
    }
    
    Ok(())
}
```

## Form Data Extraction

```rust
async fn extract_form_data(url: &str) -> anyhow::Result<serde_json::Value> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    page.navigate(url).await?;
    page.wait_for_selector("form", None).await?;
    
    // Extract all form values using JavaScript
    let form_data = page.evaluate(r#"
        (() => {
            const form = document.querySelector('form');
            const formData = new FormData(form);
            const data = {};
            
            for (let [key, value] of formData.entries()) {
                data[key] = value;
            }
            
            return data;
        })()
    "#).await?;
    
    println!("Form data: {:#?}", form_data);
    
    Ok(form_data)
}
```

## Automated Testing Pattern

```rust
use serde_json::json;

async fn test_form_submission() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Test data
    let test_cases = vec![
        json!({
            "name": "John Doe",
            "email": "john@example.com",
            "message": "Test message",
            "expected_result": "success"
        }),
        json!({
            "name": "",
            "email": "invalid-email",
            "message": "",
            "expected_result": "error"
        }),
    ];
    
    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Running test case {}...", i + 1);
        
        page.navigate("https://example.com/form").await?;
        page.wait_for_selector("form", None).await?;
        
        // Fill form
        if let Some(name_input) = page.query_selector("#name").await? {
            name_input.type_text(test_case["name"].as_str().unwrap()).await?;
        }
        
        if let Some(email_input) = page.query_selector("#email").await? {
            email_input.type_text(test_case["email"].as_str().unwrap()).await?;
        }
        
        if let Some(message_input) = page.query_selector("#message").await? {
            message_input.type_text(test_case["message"].as_str().unwrap()).await?;
        }
        
        // Submit
        if let Some(submit_btn) = page.query_selector("button[type='submit']").await? {
            submit_btn.click().await?;
        }
        
        // Verify result
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let expected = test_case["expected_result"].as_str().unwrap();
        let has_success = page.query_selector(".success-message").await?.is_some();
        let has_error = page.query_selector(".error-message").await?.is_some();
        
        match expected {
            "success" => assert!(has_success, "Expected success message"),
            "error" => assert!(has_error, "Expected error message"),
            _ => {}
        }
        
        println!("✓ Test case {} passed", i + 1);
    }
    
    Ok(())
}
```

## Tips

1. **Wait for elements** - Always use `wait_for_selector`
2. **Human-like typing** - Use `type_text_with_delay` for passwords
3. **Handle validation** - Check for error messages
4. **Test thoroughly** - Try invalid inputs
5. **Add delays** - Between steps for reliability

## See Also

- [Input Guide](../features/INPUT.md)
- [Wait Functions](../features/WAIT_FUNCTIONS.md)
- [Element Interaction](../features/ELEMENT_INTERACTION.md)
- [API Reference](../API_REFERENCE.md)
