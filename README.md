# artificial_friend

Simplest possible Telegram bot in Rust that uses OpenAI GPT-4 LLM to participate in discussions. Uses simple rules for determining whether to respond to messages.

## How to run

Set environment variables:

---

| Variable              | Description                                                                                                               | Allowed values                              |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| RUST_LOG              | Logging verbosity. See `tracing::Level` [docs](https://docs.rs/tracing/0.1.37/tracing/struct.Level.html#implementations). | `"ERROR", "WARN", "INFO", "DEBUG", "TRACE"` |
| OPENAI_API_KEY        | OpenAI API key.                                                                                                           | `string`                                    |
| TELOXIDE_TOKEN        | Telegram Bot API key.                                                                                                     | `string`                                    |
| CHARACTER_DESCRIPTION | GPT-4 initial system prompt that should describe the character that the bot will be playing.                              | `string`                                    |
| CHARACTER_NAME        | Name of the character used for detecting mentions of the bot.                                                             | `string`                                    |
| DIALOG_TIMEOUT        | Number of seconds of inactivity in a group chat after which the bot will stop participating in the dialog.                | `int`                                       |

And then just build a Docker image and run it:

```bash
docker build . -t artificial-friend
docker run --rm artificial-friend
```

Or simply use the included `run.sh` script.
