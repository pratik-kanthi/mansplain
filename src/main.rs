extern crate clap;
extern crate reqwest;
extern crate tokio;
extern crate serde_json;

use clap::{App, Arg};
use std::env;
use std::process::Command;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ManExplainer")
        .version("1.0")
        .arg(
            Arg::with_name("command")
                .help("The command to explain")
                .required(true),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap();

    // Fetch the man page
    let output = Command::new("man")
        .arg(command)
        .output()
        .expect("Failed to fetch man page");

    let full_man_page = String::from_utf8_lossy(&output.stdout);
    let man_page_lines: Vec<&str> = full_man_page.split('\n').collect();
    let man_page: String = man_page_lines.into_iter().take(5).collect::<Vec<&str>>().join("\n");

    // Fetch API Key from environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let prompt_prefix = "A rude summary of this page is:";
    let complete_prompt = format!("{}{}", prompt_prefix, man_page);



    let messages = json!([
        {
            "role": "system",
            "content": "You are a helpful assistant that explains man pages rudely."
        },
        {
            "role": "user",
            "content": complete_prompt
        }
    ]);

    let req_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages
    });


    // Send to OpenAI API
    let client = Client::new();

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&req_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Extract and display the result
    if let Some(explanation) = res["choices"][0]["message"]["content"].as_str() {
        println!("{}", explanation);
    }

    Ok(())
}
