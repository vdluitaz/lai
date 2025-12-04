use std::env;
use std::io::{self, Read};

use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::json;

fn show_help() {
    eprintln!("Usage: lai -p <prompt>");
    eprintln!("       command | lai -p <prompt>");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -p <prompt>    User prompt to include with the piped/inline data");
    eprintln!("  -h             Show help");
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut prompt = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-p" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: -p requires an argument");
                    show_help();
                    std::process::exit(1);
                }
                prompt = args[i].clone();
            }
            "-h" => {
                show_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Invalid option {}", args[i]);
                show_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Read from stdin if piped
    let input = if !atty::is(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        String::new()
    };

    if prompt.is_empty() {
        eprintln!("Error: Prompt (-p) is required.");
        show_help();
        std::process::exit(1);
    }

    // Build context-boosting prompt template
    let user_content = if input.trim().is_empty() {
        format!("TASK:\n{prompt}\n\nRules:\n- If no context is provided, use your general knowledge to complete the task.")
    } else {
        format!("You are an assistant that *must* use the CONTEXT provided below.\n\nCONTEXT:\n\"\"\"\n{input}\n\"\"\"\n\nTASK:\n{prompt}\n\nRules:\n- Base your answer ONLY on the context unless the task requires outside knowledge.\n- If the context is unclear, say so and ask a clarifying question.\n- Do not fabricate details not present in the context.")
    };

    // Allow overriding the model with OLLAMA_MODEL env var
    let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string());

    let json_payload = json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are a concise, context-grounded assistant. You always prioritize analyzing piped input above general knowledge."
            },
            {
                "role": "user",
                "content": user_content
            }
        ],
        "stream": false
    });

    let client = Client::new();
    let resp = client
        .post("http://localhost:11434/v1/chat/completions")
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .json(&json_payload)
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        eprintln!("Request failed with status {status}: {body}");
        std::process::exit(1);
    }

    let json_resp: serde_json::Value = resp.json()?;

    if let Some(content) = json_resp["choices"][0]["message"]["content"].as_str() {
        println!("{content}");
    } else {
        eprintln!("Unexpected response format: {json_resp}");
        std::process::exit(1);
    }

    Ok(())
}
