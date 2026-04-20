# 🤖 my-chatbot

A native conversational chatbot built from scratch in **Rust** — no external AI APIs, no machine learning frameworks. Just pure Rust and a handcrafted brain.

---

## 💡 What Is This?

`my-chatbot` is a terminal-based chatbot that simulates natural human conversation using a rule-based **Natural Language Understanding (NLU)** and **Natural Language Generation (NLG)** pipeline — all written natively in Rust.

It can recognize what you mean, remember your name, detect the topic you're talking about, and respond with relevant advice or knowledge — all without any internet connection or external dependencies.

---

## ✨ Features

- 🧠 **Multi-intent detection** — understands multiple meanings in a single message
- 💬 **Topic-aware responses** — handles Career, Technology, Finance, Mental Health, Relationships, and more
- 📚 **Built-in knowledge base** — gives real, useful advice per topic
- 🧵 **Contextual memory** — remembers your name and tracks conversation history
- 😐 **Sentiment detection** — recognizes positive, negative, curious, and neutral tones
- 🔁 **Follow-up questions** — keeps the conversation going naturally
- ⚡ **Zero dependencies** — no external crates required for core logic

---

## 🗂️ Project Structure

```
my-chatbot/
├── src/
│   ├── main.rs       # Entry point — handles the conversation loop
│   └── brain.rs      # The core NLU/NLG engine (intents, topics, responses)
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.65 or later)

### Build & Run

```bash
git clone https://github.com/COSTA300/my-chatbot.git
cd my-chatbot
cargo run
```

### Example Conversation

```
Bot: Hello! I'm here to chat, listen, and help where I can.

You: Hey, my name is Calvin
Bot: Hey Calvin! Good to see you again. It's great to properly meet you, Calvin!

You: I'm struggling with my career lately
Bot: I hear you, Calvin. That sounds genuinely tough. Here's something
     that might help: Networking is underrated — 70% of jobs are filled
     through connections, not applications. Would you like to talk more
     about what's going on?

You: How do I get better at coding?
Bot: Great question. Here's how I'd think about it: The best way to learn
     a technology is to build something real with it — tutorials only get
     you so far.
```

---

## 🧱 How It Works

The bot's intelligence is split into three layers inside `brain.rs`:

| Layer | Description |
|---|---|
| **NLU** | Parses user input → extracts intents (Greeting, AskAdvice, ExpressDistress, etc.) |
| **Topic Engine** | Classifies the subject (Career, Tech, Finance, etc.) from keywords |
| **NLG** | Maps intents + topics → human-like, contextual responses |

There is no black-box model here. Every decision is traceable in the source code.

---

## 🛣️ Roadmap

- [ ] Persist memory between sessions (file-based storage)
- [ ] Randomized responses using the `rand` crate
- [ ] Expand the knowledge base with more topics and tips
- [ ] Optional: Anthropic API fallback for unknown inputs
- [ ] Optional: Web frontend (WASM + Rust)

---

## 👨‍💻 Author

**COSTA300**  
Building natively. One crate at a time.

---

## 📄 License

MIT License — free to use, modify, and share.
