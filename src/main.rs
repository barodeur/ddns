use serde_json::json;
use std::{collections::HashMap, env};

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_ip = fetch_ip()?.unwrap();
    println!("Target IP found: {}", target_ip);
    patch_record(target_ip)?;
    println!("Record patched!");
    Ok(())
}

fn fetch_ip() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    Ok(resp.get("origin").map(|s| s.clone()))
}

fn patch_record(ip: String) -> Result<(), Box<dyn std::error::Error>> {
    let zone_id = env::var("ZONE_ID").unwrap();
    let record_id = env::var("RECORD_ID").unwrap();
    let token = env::var("CF_TOKEN").unwrap();
    let client = reqwest::blocking::Client::new();
    client
        .patch(
            format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                zone_id, record_id
            )
            .as_str(),
        )
        .bearer_auth(token)
        .json(&json!({ "content": ip }))
        .send()?;
    Ok(())
}
