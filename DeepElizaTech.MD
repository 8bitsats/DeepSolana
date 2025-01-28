# Technical Documentation

## Environment Setup

### Required Dependencies

```json
{
  "dependencies": {
    "@elizaos/core": "0.1.7",
    "@livekit/components-react": "^2.8.1",
    "@metaplex-foundation/js": "^0.20.1",
    "@solana/web3.js": "^1.98.0",
    "livekit-client": "^2.8.1"
  }
}
```

### Environment Variables

```env
# Model Providers
ANTHROPIC_API_KEY=sk-ant-***
GROQ_API_KEY=gsk_***
GROQ_API_URL=https://api.groq.com/openai/v1
OPENAI_API_KEY=sk-***
DEEPSEEK_API_KEY=sk-***

# LiveKit Configuration
LIVEKIT_PROJECT_URL=wss://deepsolana-nski5zjg.livekit.cloud
LIVEKIT_SIR_URL=sip:4jcgaw1fj4w.sip.livekit.cloud
LIVEKIT_WS_URL=wss://deepsolana-nski5zjg.livekit.cloud
LIVEKIT_API_KEY=***
LIVEKIT_API_SECRET=***

# ElevenLabs Configuration
ELEVEN_LABS_API_KEY=sk-***
ELEVEN_LABS_AGENT_ID=***
ELEVEN_LABS_MODEL=eleven_turbo_v2_5
ELEVEN_LABS_VOICE_ID=***
ELEVEN_LABS_VOICE_NAME=Bella
ELEVEN_LABS_VOICE_STABILITY=0.71
ELEVEN_LABS_VOICE_SIMILARITY=0.5
ELEVEN_LABS_VOICE_STYLE=0.0
ELEVEN_LABS_VOICE_SPEAKER_BOOST=true

# DeepSeek Configuration
DEEPSEEK_API_KEY=sk-***
DEEP_SEEK_CHAT_MODEL=deepseek-chat
DEEP_SEEK_REASONING_MODEL=deepseek-reasoner
DEEP_SEEK_BASE_URL=https://api.deepseek.com

# Solana Configuration
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_PRIVATE_KEY=[Base64 encoded key]
```

## Voice Action Implementation

### Core Action Definition

```typescript
const VoiceAction: Action = {
    name: "VOICE_PROCESSING",
    similes: ["HANDLE_VOICE_COMMAND", "PROCESS_AUDIO", "VOICE_INTERACTION"],
    description: "Processes voice commands using LiveKit for real-time audio streaming",
    
    validate: async (runtime: IAgentRuntime, message: Memory) => {
        const isVoiceCommand = message.content.text?.toLowerCase().includes('[voice command]');
        return isVoiceCommand;
    },

    handler: async (runtime: IAgentRuntime, message: Memory, state?: VoiceState) => {
        // Handler implementation
    }
};
```

### Audio Processing Pipeline

```typescript
// LiveKit room configuration
const room = new Room({
    adaptiveStream: true,
    dynacast: true
});

// Audio processing setup
const audioContext = new AudioContext();
const source = audioContext.createMediaStreamSource(track.mediaStream);
const processor = audioContext.createScriptProcessor(1024, 1, 1);

// Audio processing handler
processor.onaudioprocess = (e) => {
    const inputData = e.inputBuffer.getChannelData(0);
    // Process audio data
};
```

### NFT Minting Configuration

```typescript
const nftConfig = {
    name: `Cheshire Voice NFT - ${Date.now()}`,
    uri: imageUrl,
    sellerFeeBasisPoints: 500,
    symbol: 'CHESH',
    isCollection: false,
    isMutable: true,
    maxSupply: 1,
};
```

## TypeScript Configuration

### tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "lib": ["ESNext", "dom"],
    "moduleResolution": "Bundler",
    "outDir": "./dist",
    "rootDir": ".",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "allowJs": true,
    "moduleDetection": "force"
  }
}
```

## Runtime Configuration

### Agent Runtime Setup

```typescript
const runtime = new AgentRuntime({
    token: process.env.ANTHROPIC_API_KEY!,
    modelProvider: ModelProviderName.ANTHROPIC,
    character: characterConfig,
    logging: true,
    agentId: 'cheshire-voice-1234-5678-9abc',
    conversationLength: 10,
    serverUrl: process.env.LIVEKIT_PROJECT_URL,
    databaseAdapter: {
        // Minimal database adapter implementation
    },
    cacheManager: {
        // Minimal cache manager implementation
    }
});
```

## Error Handling

```typescript
// Global error handlers
process.on('SIGINT', () => {
    console.log('Shutting down...');
    process.exit(0);
});

process.on('uncaughtException', (error) => {
    console.error('Uncaught Exception:', error);
    process.exit(1);
});

// Voice processing error handling
try {
    // Process voice command
} catch (error) {
    console.error("Voice processing failed:", error);
    return {
        content: {
            text: "Sorry, I couldn't process that voice command",
            action: "VOICE_PROCESSING_ERROR"
        }
    };
}
```

## Development Commands

```bash
# Install dependencies
pnpm install

# Start development server
pnpm start

# Build project
pnpm build

# Clean build artifacts
pnpm clean
