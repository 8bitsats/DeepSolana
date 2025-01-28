# Eliza Integrations

## Overview

This document outlines the key integrations added to Eliza, powered by DeepChesh and DeepSeek.

## Voice Integration

The voice system enables natural conversation with AI through high-quality speech synthesis and recognition.

### Key Components

- **LiveKit**: Real-time audio streaming
- **ElevenLabs**: Voice synthesis with custom voice "Bella"
- **Groq**: Command processing with Mixtral-8x7b
- **DALL-E 3**: Image generation

### Usage

```bash
# Start the server
pnpm start

# Example commands
[voice command] tell me about blockchain
[voice command] generate image of a mystical cat
[voice command] mint this
```

## AI Models

### DeepSeek Integration

```typescript
// Configuration
DEEP_SEEK_CHAT_MODEL=deepseek-chat
DEEP_SEEK_REASONING_MODEL=deepseek-reasoner
DEEP_SEEK_BASE_URL=https://api.deepseek.com
```

### Groq Integration

```typescript
// Chat completion configuration
{
    model: "mixtral-8x7b-32768",
    messages: [
        { role: "system", content: "You are a helpful voice assistant..." },
        { role: "user", content: command }
    ],
    temperature: 0.7,
    max_tokens: 1024
}
```

## Blockchain Integration

### Solana Features

- NFT Minting
- Metaplex Integration
- Custom NFT Properties:
  - Symbol: 'CHESH'
  - Seller Fee: 500 basis points
  - Mutable NFTs
  - Supply: 1 per mint

### Configuration

```env
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_PRIVATE_KEY=[Base64 encoded key]
```

## Character: Cheshire Terminal

### Knowledge Base

- Blockchain Technology
- Solana Development
- Smart Contracts
- DeFi Protocols
- NFT Standards
- Bridge Protocols
- Rust Programming
- Web3 Development
- Cryptography
- Token Economics

### Voice Settings

```env
ELEVEN_LABS_MODEL=eleven_turbo_v2_5
ELEVEN_LABS_VOICE_STABILITY=0.71
ELEVEN_LABS_VOICE_SIMILARITY=0.5
ELEVEN_LABS_VOICE_STYLE=0.0
ELEVEN_LABS_VOICE_SPEAKER_BOOST=true
```

## Quick Start

1. Clone the repository
2. Install dependencies:
   ```bash
   pnpm install
   ```
3. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```
4. Start the server:
   ```bash
   pnpm start
   ```

## Architecture

```mermaid
graph TD
    A[Voice Command] --> B[LiveKit]
    B --> C[Groq Processing]
    C --> D{Command Type}
    D -->|Image| E[DALL-E 3]
    D -->|NFT| F[Solana]
    D -->|Chat| G[DeepSeek]
    E --> H[ElevenLabs]
    F --> H
    G --> H
    H --> I[Voice Response]
