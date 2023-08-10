use serde::{Deserialize, Serialize};
use csv::{Writer, WriterBuilder};
use std::io;
use regex::Regex;
use structopt::StructOpt;
use reqwest::header::{HeaderMap, HeaderValue};
use chrono::{Local, Datelike};

#[derive(StructOpt)]
struct Cli {
    #[structopt(long, default_value = "")]
    api_key: String,

    #[structopt(long, default_value = "")]
    app_key: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Metric>,
}

#[derive(Debug, Deserialize)]
struct Metric {
    #[serde(rename = "type")]
    metric_type: String,
    id: String,
}

fn validate_keys(api_key: &str, app_key: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    // Check if API key and App key are not empty and contain only alphanumeric characters
    !api_key.is_empty() && !app_key.is_empty() &&
    re.is_match(api_key) && re.is_match(app_key)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get the command line arguments
    let args = Cli::from_args();
    let api_key = &args.api_key;
    let app_key = &args.app_key;

    // Get the date
    let today = Local::today();
    let formatted_date = format!("{:02}-{:02}-{}", today.day(), today.month(), today.year());

    if !validate_keys(api_key, app_key){
        println!("Usage: ./dd-unqueried-metrics --api-key=<API_KEY> --app-key=<APP_KEY>");
        println!("api_key and app-key parameters not provided, prompting user...\n");

        // Prompt the user for the API key
        println!("Please enter your Datadog API Key:");
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).expect("Failed to read line");

        // Prompt the user for the Application key
        println!("Please enter your Datadog Application Key:");
        let mut app_key = String::new();
        io::stdin().read_line(&mut app_key).expect("Failed to read line");

        // Remove any trailing whitespace from the input
        api_key = api_key.trim().to_string();
        app_key = app_key.trim().to_string();
    }
    
    let api_endpoint = "https://api.datadoghq.com/api/v2/metrics";
    let url = format!("{}?window[seconds]=1209600&filter[queried]=true&filter[tags]=source:ephemera-org-example-custom-metric", api_endpoint);

    let mut headers = HeaderMap::new();
    headers.insert("DD-API-KEY", HeaderValue::from_str(&api_key)?);
    headers.insert("DD-APPLICATION-KEY", HeaderValue::from_str(&app_key)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    println!("Loading unqueried metrics...");
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?
        .text()
        .await?;

    let api_response: ApiResponse = serde_json::from_str(&response)?;

    let path = format!("dd-unqueried-metrics-{}.csv", formatted_date);

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    
    let mut metric_count = 0;

    for metric in api_response.data {
        metric_count += 1;
        println!("Writing Metric ID: {:#?}", metric.id);
        writer.serialize(metric.id)?;
    }

    println!("{} metrics found and exported.", metric_count);

    writer.flush()?;

    Ok(())
}