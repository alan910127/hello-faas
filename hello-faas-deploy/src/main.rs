use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let binary_path = match std::env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!(
                "Usage: {} <path-to-binary> [gateway-url]",
                std::env::args()
                    .next()
                    .unwrap_or("hello-faas-deploy".into())
            );
            std::process::exit(1);
        }
    };
    let gateway_url = std::env::args()
        .nth(2)
        .unwrap_or("http://localhost:3000/deploy".into());

    let binary_content = match std::fs::read(binary_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read binary: {}", e);
            std::process::exit(1);
        }
    };
    let form_data = reqwest::multipart::Form::new().part(
        "binary",
        reqwest::multipart::Part::bytes(binary_content).file_name("bootstrap"),
    );
    let client = reqwest::Client::new();
    match client.post(gateway_url).multipart(form_data).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Deployed successfully: {}",
                    response.text().await.unwrap_or_default()
                );
            } else {
                eprintln!("Failed to deploy: {}", response.status());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to deploy: {}", e);
            std::process::exit(1);
        }
    }
}
