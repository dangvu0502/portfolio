use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<PullRequest>,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    number: u32,
    title: String,
    html_url: String,
    state: String,
    created_at: String,
    updated_at: String,
    repository_url: String,
}

#[derive(Debug, Serialize)]
struct OSSContribution {
    repository: String,
    pr_number: String,
    link: String,
    title: String,
    state: String,
    created_at: String,
    updated_at: String,
}

fn get_github_token() -> Result<String> {
    env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable is required")
}

fn get_username(client: &reqwest::blocking::Client, token: &str) -> Result<String> {
    if let Ok(username) = env::var("GITHUB_USERNAME") {
        return Ok(username);
    }

    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "fetch-oss-contributions")
        .send()
        .context("Failed to fetch GitHub user")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "GitHub API request failed: {} {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let user: GitHubUser = response.json().context("Failed to parse user response")?;
    Ok(user.login)
}

fn fetch_user_pull_requests(
    client: &reqwest::blocking::Client,
    token: &str,
    username: &str,
) -> Result<Vec<PullRequest>> {
    let query = format!("is:pr author:{} is:public", username);
    let url = format!(
        "https://api.github.com/search/issues?q={}&sort=created&order=desc&per_page=7",
        urlencoding::encode(&query)
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "fetch-oss-contributions")
        .send()
        .context("Failed to fetch pull requests")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "GitHub API request failed: {} {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let search_response: SearchResponse = response
        .json()
        .context("Failed to parse search response")?;
    Ok(search_response.items)
}

fn transform_pr_data(prs: Vec<PullRequest>) -> Vec<OSSContribution> {
    prs.into_iter()
        .map(|pr| {
            let repository = pr
                .repository_url
                .strip_prefix("https://api.github.com/repos/")
                .unwrap_or("unknown")
                .to_string();

            OSSContribution {
                repository,
                pr_number: pr.number.to_string(),
                link: pr.html_url,
                title: pr.title,
                state: pr.state,
                created_at: pr.created_at,
                updated_at: pr.updated_at,
            }
        })
        .collect()
}

fn main() -> Result<()> {
    println!("Fetching OSS contributions...");

    let token = get_github_token()?;
    let client = reqwest::blocking::Client::new();

    let username = get_username(&client, &token)?;
    println!("Fetching PRs for user: {}", username);

    let prs = fetch_user_pull_requests(&client, &token, &username)?;
    println!("Found {} pull requests", prs.len());

    let contributions = transform_pr_data(prs);

    // Determine output path relative to project root
    let output_path = PathBuf::from(env::current_dir()?)
        .join("data")
        .join("oss-contributions.json");

    // Ensure data directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).context("Failed to create data directory")?;
    }

    let json = serde_json::to_string_pretty(&contributions)
        .context("Failed to serialize contributions")?;

    fs::write(&output_path, json).context("Failed to write output file")?;

    println!(
        "Successfully saved {} contributions to {}",
        contributions.len(),
        output_path.display()
    );

    // Print summary
    let open_prs = contributions.iter().filter(|pr| pr.state == "open").count();
    let closed_prs = contributions
        .iter()
        .filter(|pr| pr.state == "closed")
        .count();
    println!("Summary: {} open, {} closed/merged", open_prs, closed_prs);

    Ok(())
}
