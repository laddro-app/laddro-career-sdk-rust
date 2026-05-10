# laddro-career

Rust SDK for the [Laddro Career API](https://api.laddro.com/reference).

## Install

```toml
[dependencies]
laddro-career = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use laddro_career::{Client, TailorRequest};

#[tokio::main]
async fn main() -> Result<(), laddro_career::Error> {
    let client = Client::new("laddro_live_...");

    // List resumes
    let list = client.list_resumes(20, 0).await?;
    for resume in &list.items {
        println!("{}: {}", resume.resume_id, resume.title);
    }

    // Tailor a resume
    let pdf = client.tailor(&TailorRequest {
        position_name: "Senior Frontend Engineer".into(),
        job_url: Some("https://jobs.example.com/sfe".into()),
        ..Default::default()
    }).await?;
    std::fs::write("tailored.pdf", &pdf)?;

    // Browse templates (works without auth too)
    let templates = client.list_templates().await?;
    for t in &templates {
        println!("{} (ATS: {})", t.name, t.ats_score);
    }

    Ok(())
}
```

## Links

- [laddro.com](https://laddro.com)
- [API Reference](https://api.laddro.com/reference)
- [Docs](https://docs.laddro.com)
- [GitHub](https://github.com/laddro-app)

## License

MIT
