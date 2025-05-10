# 🎯 YouTube Comments Spam Filter — Backend Service

## Overview

This is a high-performance backend service written in **Rust** designed to **detect and classify Indonesian YouTube comment spam**, particularly spam linked to **illegal online gambling operations**. It leverages **AI-based inference engines** (DeepSeek / Gemini) and an optional **Redis-based caching layer** for optimal speed and accuracy.

---

## 🔧 Key Features

- ✅ Real-time classification of YouTube comments as **spam / not spam**
- ✅ AI-powered analysis using **DeepSeek** and **Gemini** language models
- ✅ Redis-based **caching layer** for reduced latency and cost
- ✅ Designed with **extensibility** and **horizontal scalability** in mind

---

## 🧱 Technology Stack

| Layer             | Technology        |
|------------------|-------------------|
| Language          | Rust (async runtime) |
| Cache Layer       | Redis (or compatible key-value store) |
| AI Integrations   | DeepSeek, Gemini (via HTTP APIs) |
| API Interface     | JSON over HTTP (REST) |
| Observability     | (TBD) Tracing, Metrics, Logs |

---

## 🏗 Architecture

         +---------------------+
         |     Client API      |
         +---------+-----------+
                   |
                   v
         +---------------------+
         |     API Server      |  <--- Written in Rust
         +---------------------+
                   |
          +--------+--------+
          |                 |
          v                 v
     +--------+       +------------+
     |  Redis |       |  AI Models |
     +--------+       +------------+
                          |
                 DeepSeek / Gemini

- **API Server**: Exposes endpoints to classify comments.
- **Cache Layer**: Speeds up repeated classifications using Redis.
- **AI Module**: Connects to either DeepSeek or Gemini APIs to perform classification.

---

## 📡 API Endpoints (Planned)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST   | `/analyze` | Accepts a YouTube comment and returns spam classification (CSV format) |
| GET    | `/health` | Health check |
| GET    | `/cache/:keyword` | Inspect cache hit for a normalized gambling keyword |

> Note: Full OpenAPI spec will be available post-v1.

---

## 🧪 AI Integration Examples

### 🧠 DeepSeek

```bash
curl --request POST \
  --url https://api.deepseek.com/chat/completions \
  --header 'authorization: Bearer {DEEPSEEK_TOKEN}' \
  --header 'content-type: application/json' \
  --data '{
    "model": "deepseek-chat",
    "messages": [
      {
        "role": "system",
        "content": "You are an AI classifier detecting Indonesian YouTube comment spam related to illegal gambling..."
      },
      {
        "role": "user",
        "content": "Aku adalah pemenang, dan ❄️ KYT4D ❄️ adalah keberuntunganku!"
      }
    ],
    "stream": false,
    "max_tokens": 50
  }'
```

## 🤖 Gemini
```
curl --request POST \
  --url 'https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-lite:generateContent?key={GEMINI_TOKEN}' \
  --header 'content-type: application/json' \
  --data '{
    "system_instruction": {
      "parts": [
        {
          "text": "You are an AI classifier detecting Indonesian YouTube comment spam related to illegal gambling..."
        }
      ]
    },
    "contents": [
      {
        "parts": [
          {
            "text": "Aku adalah pemenang, dan ❄️ KYT4D ❄️ adalah keberuntunganku!"
          }
        ]
      }
    ],
    "generationConfig": {
        "stopSequences": [
          "\n"
        ],
        "temperature": 0.2,
        "maxOutputTokens": 50,
        "topP": 0.5,
        "topK": 3
      }
  }'
```
## 📤 Response Format

The API response (from either DeepSeek or Gemini) will return a CSV-formatted string:

`S,K,C`

### Columns:

- `S`: Spam classification (**1 = SPAM**, **0 = NOT SPAM**)
- `K`: Normalized keyword (e.g., `KYT4D`, `AXL777`, `GenericGambling`, or `N/A`)
- `C`: Confidence score (**0.00–1.00**)

### Sample Responses:

```
1,KYT4D,0.95
0,N/A,0.03
1,GenericGambling,0.80
```

---

## 🚀 Getting Started

1. **Install Rust:**

```bash
curl https://sh.rustup.rs -sSf | sh
```
2. **Install Redis (or a compatible key-value store)**

3. **Clone the Repository:**
```
git clone https://github.com/your-org/yt-comments-filter-be.git
cd yt-comments-filter-be
```
4. **Build & Run the Service:**
```
cargo build --release
./target/release/yt-comments-filter-be

```
## 🤝 Contributing

We welcome external contributions and collaborative improvements. To contribute:

1. **Fork** the repository to your own GitHub account.  
2. **Create a new branch** for your feature or fix.  
3. **Make your changes**, following Rust best practices and existing code structure.  
4. **Test your code** thoroughly.  
5. **Submit a Pull Request** (PR) with a clear explanation of your changes and rationale.

> Note: All contributions must pass formatting (`cargo fmt`) and linting (`cargo clippy`) checks before being reviewed.

---

## 📌 Roadmap

- [ ] Admin dashboard (browser-based) for cache visibility and spam analytics  
- [ ] WebSocket support for real-time comment feed ingestion  
- [ ] Direct integration with YouTube Data API (v3) for automated moderation  
- [ ] Expand language model support beyond Indonesian (e.g., Tagalog, Vietnamese)

---

## 🧠 Strategic Use Cases

This backend is not just a spam filter — it’s a foundation for multiple verticals:

- 🔍 **Brand safety tools** for marketing agencies targeting SEA/Indonesia  
- 🛡 **Anti-abuse layers** for influencer management platforms  
- 📊 **Real-time spam analytics** for YouTube creators and partners  
- 💼 **Licensable SaaS IP** — ready for resale via platforms like [Acquire.com](https://acquire.com)

---

## 📜 License

Distributed under the **MIT License**. See the `LICENSE` file for full legal terms.

---

## 👨‍💻 Maintainer

**Hendri Marcolia**  
LinkedIn: [linkedin.com/in/hendri-marcolia-847ba0190](https://id.linkedin.com/in/hendri-marcolia-847ba0190)

For enterprise collaborations, investment, or acquisition inquiries — connect via LinkedIn or direct business channels.

---
