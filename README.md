# Monster Gaming SDK for Rust

Official Rust client for [Monster Gaming](https://monstergaming.ai) — an AI-powered game development platform for Unreal Engine, Unity, Godot, and bespoke engines.

## Installation

```toml
[dependencies]
monstergaming = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use monstergaming::{MonsterGaming, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), monstergaming::Error> {
    let client = MonsterGaming::new("mg_your_api_key");

    let response = client
        .chat_completion("monster-gpt", vec![
            ChatMessage::user("Generate a UE5 C++ character controller with double jump"),
        ])
        .await?;

    println!("{}", response.choices[0].message.content);
    Ok(())
}
```

## Monster-GPT

`monster-gpt` is Monster Gaming's flagship model. It auto-detects your game engine and routes your query to a specialist agent — shader programming, networking, animation, level design, QA, and 25+ other disciplines.

```rust
let response = client
    .chat_completion("monster-gpt", vec![
        ChatMessage::system("Engine: Unity 6. Language: C#."),
        ChatMessage::user("Implement object pooling for projectiles"),
    ])
    .await?;
```

## Available Models

```rust
let models = client.list_models().await?;
for model in &models.data {
    println!("{}", model.id);
}
```

Budget models (Free tier): `monster-gpt`, `claude-haiku`, `deepseek-chat`, `codestral`, `gemini-3-flash`

Standard models (Starter+): `claude-sonnet`, `gpt-4o`, `gemini-3.1-pro`, `mistral-large`

Premium models (Pro+): `claude-opus`, `o3`, `gpt-5.4`

## Pricing

Free tier available — no credit card required. See [monstergaming.ai/pricing](https://monstergaming.ai/pricing) for details.

## Documentation

Full documentation at [monstergaming.ai/quickstart](https://monstergaming.ai/quickstart).

## License

MIT
