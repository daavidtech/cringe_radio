# Cringe radio

Plays music in your discord channel.

## How to run

```
cargo run --token YOUR_TOKEN
```

Or you can set CRINGE_RADIO_TOKEN environment variable.

## environment variables

```env
# discord bot token
CRINGE_RADIO_TOKEN
# youtube api key
YOUTUBE_APIKEY
# openai api key
OPENAI_APIKEY
```

## Commands

```
# plays youtube video sound
play YOUTUBE_VIDEO_URL
# stop current music player
stop
```

## Requirements

Requires youtube-dl and ffmpeg to be installed on the system and accessible for this program.

Protip: using yt-dlp also works but it needs to be renamed to youtube-dl.

## Permissions

The bot requires to have message content intent permission which can be enabled from discord bot developer portal.