# Ravalink Example Bot

**A Discord Bot Demonstrating Ravalink Integration with Kafka**

[![Rust Version](https://img.shields.io/badge/rust-1.65%2B-blue)](https://www.rust-lang.org/)
[![Dependencies](https://img.shields.io/badge/dependencies-ravalink--lib%2Cravalink--interconnect-brightgreen)](https://github.com/yourorg)

## Overview
This example bot demonstrates how to build a Discord music bot using:
- **Ravalink**: Kafka-based audio service
- **Ravalink-Lib**: Kafka client library for bot interactions
- **Ravalink-Interconnect**: Shared message schema

The bot handles music commands through Kafka and responds to audio events from the Ravalink server.

## Prerequisites
- Rust 1.65+
- Docker (for Kafka setup)
- [Ravalink Server](https://github.com/CrySteRz/ravalink) running
- Discord Bot Token ([create one](https://discord.com/developers/applications))

## Running the Bot
1. Start Ravalink server
2. Start Kafka
3. Run the bot:
   ```bash
   cargo run --release
   ```

## Architecture Flow
```
Discord User
  │
  ├──▶ !play command
  │     │
  │     └──▶ Bot (ravalink-lib)
  │           │ Produces PlayTrack command
  │           ▼
  │         Kafka
  │           │
  │           └──▶ Ravalink Server
  │                  │ Processes audio
  │                  ▼
  │                Kafka
  │                  │
  │                  └──▶ Bot (ravalink-lib)
  │                        ▼
  └────────────────────── AudioEvent (e.g., TrackStart)
```

