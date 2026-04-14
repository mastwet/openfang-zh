<p align="center">
  <img src="public/assets/openfang-logo.png" width="160" alt="OpenFang Logo" />
</p>

<h1 align="center">OpenFang</h1>
<h3 align="center">智能体操作系统</h3>

<p align="center">
  开源智能体操作系统，使用 Rust 构建。137K 行代码。14 个 crate。1,767+ 测试。零 clippy 警告。<br/>
  <strong>单一二进制文件。久经考验。真正为你工作的智能体。</strong>
</p>

<p align="center">
  <a href="https://openfang.sh/docs">文档</a> &bull;
  <a href="https://openfang.sh/docs/getting-started">快速开始</a> &bull;
  <a href="https://x.com/openfangg">Twitter / X</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-Rust-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="MIT" />
  <img src="https://img.shields.io/badge/version-0.5.9-green?style=flat-square" alt="v0.5.9" />
  <img src="https://img.shields.io/badge/tests-1,767%2B%20passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/clippy-0%20warnings-brightgreen?style=flat-square" alt="Clippy" />
  <a href="https://www.buymeacoffee.com/openfang" target="_blank"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=flat-square&logo=buy-me-a-coffee&logoColor=black" alt="Buy Me A Coffee" /></a>
</p>

---

> **v0.5.9 — 中文本地化版本**
>
> 本仓库是 OpenFang 的中文本地化版本。所有 Web UI、CLI 启动菜单和交互界面已翻译为中文。基于上游 v0.5.9 版本。OpenFang 仍处于 pre-1.0 阶段，小版本间可能存在破坏性变更。生产环境请固定到特定 commit。[在这里报告问题](https://github.com/mastwet/openfang-zh/issues)

---

## 🆕 中文本地化更改

本仓库相对于上游 OpenFang 的主要更改：

- **Web UI 全汉化**：`index_body.html` 及所有 JS 页面模块（chat、agents、settings、overview、scheduler、channels、hands、skills、sessions、approvals、comms、logs、runtime、usage、workflows、workflow-builder 等）中的硬编码英文已全部替换为中文
- **i18n 系统**：默认语言设为中文（`zh`），浏览器语言为中文时自动切换；支持中/英/俄三语运行时切换
- **CLI 启动菜单汉化**：`launcher.rs` 中所有交互菜单文本翻译为中文（欢迎信息、菜单项、状态提示等）
- **zh.json 完善**：696 个翻译 key，100% 覆盖 en.json，品牌名保持英文（OpenFang、ClawHub、Telegram、Discord 等）

---

## OpenFang 是什么？

OpenFang 是一个**开源智能体操作系统** — 不是聊天机器人框架，不是 Python LLM 包装器，不是"多智能体编排器"。它是用 Rust 从零构建的自主智能体完整操作系统。

传统智能体框架等待你输入。OpenFang 运行**自主为你工作的智能体** — 按计划执行、7×24 小时、构建知识图谱、监控目标、生成线索、管理社交媒体、并向你的仪表板报告结果。

整个系统编译为**约 32MB 的单一二进制文件**。一次安装，一条命令，智能体即刻上线。

```bash
curl -fsSL https://openfang.sh/install | sh
openfang init
openfang start
# 仪表板运行在 http://localhost:4200
```

<details>
<summary><strong>Windows</strong></summary>

```powershell
irm https://openfang.sh/install.ps1 | iex
openfang init
openfang start
```

</details>

---

## 工具包：真正做事的智能体

<p align="center"><em>"传统智能体等你在键盘上输入。工具包为你<strong>自动工作</strong>。"</em></p>

**工具包（Hands）** 是 OpenFang 的核心创新 — 预构建的自主能力包，按计划独立运行，无需你手动提示。这不是聊天机器人。这是一个早上 6 点自动醒来、研究你的竞争对手、构建知识图谱、评估发现、在你喝咖啡前向你的 Telegram 发送报告的智能体。

每个工具包包含：
- **HAND.toml** — 声明工具、设置、需求和仪表板指标的清单
- **系统提示词** — 多阶段操作手册（不是一句话 — 这些是 500+ 词的专家流程）
- **SKILL.md** — 运行时注入上下文的领域专业知识参考
- **护栏** — 敏感操作的审批门（例如，浏览器工具包在购买前需要审批）

全部编译进二进制文件。无需下载，无需 pip install，无需 Docker pull。

### 7 个内置工具包

| 工具包 | 实际功能 |
|------|----------|
| **Clip** | 接收 YouTube URL，下载视频，识别最佳片段，剪辑成竖屏短视频并添加字幕和缩略图，可选 AI 配音，发布到 Telegram 和 WhatsApp。8 阶段流水线。使用 FFmpeg + yt-dlp + 5 个 STT 后端。 |
| **Lead** | 每日运行。发现匹配 ICP 的潜在客户，通过网络研究丰富信息，0-100 打分，与现有数据库去重，以 CSV/JSON/Markdown 格式交付合格线索。持续构建 ICP 档案。 |
| **Collector** | OSINT 级情报。给定目标（公司、人物、话题），持续监控 — 变更检测、情感追踪、知识图谱构建、关键变化告警。 |
| **Predictor** | 超级预测引擎。从多个来源收集信号，构建校准推理链，带置信区间做出预测，使用 Brier 分数追踪自身准确率。具有故意反对共识的逆向模式。 |
| **Researcher** | 深度自主研究器。交叉引用多个来源，使用 CRAAP 标准（时效性、相关性、权威性、准确性、目的）评估可信度，生成 APA 格式引用报告，支持多语言。 |
| **Twitter** | 自主 Twitter/X 账户管理器。7 种轮换格式创建内容，最佳时间发布推文，回复提及，追踪绩效指标。拥有审批队列 — 没有你的批准不会发任何内容。 |
| **Browser** | Web 自动化智能体。导航网站、填写表单、点击按钮、处理多步骤工作流。使用 Playwright 桥接和会话持久化。**强制购买审批门** — 绝不会在未经明确确认的情况下花费你的钱。 |

```bash
# 激活 Researcher 工具包 — 立即开始工作
openfang hand activate researcher

# 随时检查进度
openfang hand status researcher

# 按每日计划激活线索生成
openfang hand activate lead

# 暂停而不丢失状态
openfang hand pause lead

# 查看所有可用工具包
openfang hand list
```

**构建你自己的。** 定义一个包含工具、设置和系统提示词的 `HAND.toml`。发布到 FangHub。

---

## OpenFang vs 竞品对比

<p align="center">
  <img src="public/assets/openfang-vs-claws.png" width="600" alt="OpenFang vs OpenClaw vs ZeroClaw" />
</p>

### 性能基准：实测数据，非营销数字

所有数据来自官方文档和公共仓库 — 2026 年 2 月。

#### 冷启动时间（越低越好）

```
ZeroClaw   ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10 ms
OpenFang   ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  180 ms    ★
LangGraph  █████████████████░░░░░░░░░░░░░░░░░░░░░░░░░  2.5 sec
CrewAI     ████████████████████░░░░░░░░░░░░░░░░░░░░░░  3.0 sec
AutoGen    ██████████████████████████░░░░░░░░░░░░░░░░░  4.0 sec
OpenClaw   █████████████████████████████████████████░░  5.98 sec
```

#### 空闲内存占用（越低越好）

```
ZeroClaw   █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    5 MB
OpenFang   ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   40 MB    ★
LangGraph  ██████████████████░░░░░░░░░░░░░░░░░░░░░░░░░  180 MB
CrewAI     ████████████████████░░░░░░░░░░░░░░░░░░░░░░░  200 MB
AutoGen    █████████████████████████░░░░░░░░░░░░░░░░░░  250 MB
OpenClaw   ████████████████████████████████████████░░░░  394 MB
```

#### 安装体积（越低越好）

```
ZeroClaw   █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  8.8 MB
OpenFang   ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   32 MB    ★
CrewAI     ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  100 MB
LangGraph  ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  150 MB
AutoGen    ████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░  200 MB
OpenClaw   ████████████████████████████████████████░░░░  500 MB
```

#### 安全系统（越高越好）

```
OpenFang   ████████████████████████████████████████████   16      ★
ZeroClaw   ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░    6
OpenClaw   ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    3
AutoGen    █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    2
LangGraph  █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    2
CrewAI     ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    1
```

#### 渠道适配器（越多越好）

```
OpenFang   ████████████████████████████████████████████   40      ★
ZeroClaw   ███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░   15
OpenClaw   █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   13
CrewAI     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
AutoGen    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
LangGraph  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0
```

#### LLM 供应商（越多越好）

```
ZeroClaw   ████████████████████████████████████████████   28
OpenFang   ██████████████████████████████████████████░░   27      ★
LangGraph  ██████████████████████░░░░░░░░░░░░░░░░░░░░░   15
CrewAI     ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10
OpenClaw   ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░   10
AutoGen    ███████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    8
```

### 逐功能对比

| 功能 | OpenFang | OpenClaw | ZeroClaw | CrewAI | AutoGen | LangGraph |
|------|----------|----------|----------|--------|---------|-----------|
| **语言** | **Rust** | TypeScript | **Rust** | Python | Python | Python |
| **自主工具包** | **7 个内置** | 无 | 无 | 无 | 无 | 无 |
| **安全层级** | **16 层** | 3 层基础 | 6 层 | 1 层基础 | Docker | AES 加密 |
| **智能体沙盒** | **WASM 双计量** | 无 | 白名单 | 无 | Docker | 无 |
| **渠道适配器** | **40** | 13 | 15 | 0 | 0 | 0 |
| **内置工具** | **53 + MCP + A2A** | 50+ | 12 | 插件 | MCP | LC 工具 |
| **记忆** | **SQLite + 向量** | 文件 | SQLite FTS5 | 4 层 | 外部 | 检查点 |
| **桌面应用** | **Tauri 2.0** | 无 | 无 | 无 | Studio | 无 |
| **审计追踪** | **Merkle 哈希链** | 日志 | 日志 | 追踪 | 日志 | 检查点 |
| **冷启动** | **<200ms** | ~6s | ~10ms | ~3s | ~4s | ~2.5s |
| **安装体积** | **~32 MB** | ~500 MB | ~8.8 MB | ~100 MB | ~200 MB | ~150 MB |
| **许可证** | MIT | MIT | MIT | MIT | Apache 2.0 | MIT |

---

## 16 层安全体系 — 纵深防御

OpenFang 不是事后补加安全。每一层都可以独立测试，没有单点故障。

| # | 系统 | 功能 |
|---|------|------|
| 1 | **WASM 双计量沙盒** | 工具代码在 WebAssembly 中运行，带燃料计量 + epoch 中断。看门狗线程会终止失控代码。 |
| 2 | **Merkle 哈希链审计追踪** | 每个操作都与前一个加密链接。篡改一条记录，整条链断裂。 |
| 3 | **信息流污点追踪** | 标签在执行过程中传播 — 从源头到汇点追踪秘密信息。 |
| 4 | **Ed25519 签名智能体清单** | 每个智能体身份和能力集都经过加密签名。 |
| 5 | **SSRF 防护** | 阻止私有 IP、云元数据端点和 DNS 重绑定攻击。 |
| 6 | **秘密归零** | `Zeroizing<String>` 在 API 密钥不再需要时自动从内存中擦除。 |
| 7 | **OFP 互认证** | 基于 HMAC-SHA256 nonce 的常量时间验证，用于 P2P 网络。 |
| 8 | **能力门控** | 基于角色的访问控制 — 智能体声明所需工具，内核强制执行。 |
| 9 | **安全头** | 每个响应都包含 CSP、X-Frame-Options、HSTS、X-Content-Type-Options。 |
| 10 | **健康端点脱敏** | 公共健康检查返回最少信息。完整诊断需要认证。 |
| 11 | **子进程沙盒** | `env_clear()` + 选择性变量传递。进程树隔离，跨平台 kill。 |
| 12 | **提示注入扫描器** | 检测覆盖尝试、数据泄露模式和技能中的 shell 引用注入。 |
| 13 | **循环保护** | 基于 SHA256 的工具调用循环检测，带断路器。处理乒乓模式。 |
| 14 | **会话修复** | 7 阶段消息历史验证和自动损坏恢复。 |
| 15 | **路径遍历防护** | 规范化 + 符号链接逃逸防护。`../` 在这里不起作用。 |
| 16 | **GCRA 限速器** | 带成本意识的令牌桶限速，按 IP 追踪和过期清理。 |

---

## 架构

14 个 Rust crate。137,728 行代码。模块化内核设计。

```
openfang-kernel      编排、工作流、计量、RBAC、调度器、预算追踪
openfang-runtime     智能体循环、3 个 LLM 驱动、53 个工具、WASM 沙盒、MCP、A2A
openfang-api         140+ REST/WS/SSE 端点、OpenAI 兼容 API、仪表板
openfang-channels    40 个消息适配器，带限速、DM/群组策略
openfang-memory      SQLite 持久化、向量嵌入、规范会话、压缩
openfang-types       核心类型、污点追踪、Ed25519 清单签名、模型目录
openfang-skills      60 个内置技能、SKILL.md 解析器、FangHub 市场
openfang-hands       7 个自主工具包、HAND.toml 解析器、生命周期管理
openfang-extensions  25 个 MCP 模板、AES-256-GCM 凭据保管库、OAuth2 PKCE
openfang-wire        OFP P2P 协议，带 HMAC-SHA256 互认证
openfang-cli         带守护进程管理、TUI 仪表板、MCP 服务器模式的 CLI
openfang-desktop     Tauri 2.0 原生应用（系统托盘、通知、全局快捷键）
openfang-migrate     OpenClaw、LangChain、AutoGPT 迁移引擎
xtask                构建自动化
```

---

## 40 个渠道适配器

将你的智能体连接到用户所在的每个平台。

**核心：** Telegram、Discord、Slack、WhatsApp、Signal、Matrix、Email (IMAP/SMTP)
**企业：** Microsoft Teams、Mattermost、Google Chat、Webex、飞书/Lark、Zulip
**社交：** LINE、Viber、Facebook Messenger、Mastodon、Bluesky、Reddit、LinkedIn、Twitch
**社区：** IRC、XMPP、Guilded、Revolt、Keybase、Discourse、Gitter
**隐私：** Threema、Nostr、Mumble、Nextcloud Talk、Rocket.Chat、Ntfy、Gotify
**办公：** Pumble、Flock、Twist、钉钉、Zalo、Webhooks

每个适配器支持按渠道模型覆盖、DM/群组策略、限速和输出格式化。

---

## WhatsApp Web 网关（二维码）

通过二维码将你的个人 WhatsApp 账户连接到 OpenFang — 就像 WhatsApp Web 一样。不需要 Meta Business 账户。

### 前提条件

- **Node.js >= 18** 已安装（[下载](https://nodejs.org/)）
- OpenFang 已安装并初始化

### 设置

**1. 安装网关依赖：**

```bash
cd packages/whatsapp-gateway
npm install
```

**2. 配置 `config.toml`：**

```toml
[channels.whatsapp]
mode = "web"
default_agent = "assistant"
```

**3. 设置网关 URL（选择一种）：**

添加到 shell 配置文件以持久化：

```bash
# macOS / Linux
echo 'export WHATSAPP_WEB_GATEWAY_URL="http://127.0.0.1:3009"' >> ~/.zshrc
source ~/.zshrc
```

或启动网关时内联设置：

```bash
export WHATSAPP_WEB_GATEWAY_URL="http://127.0.0.1:3009"
```

**4. 启动网关：**

```bash
node packages/whatsapp-gateway/index.js
```

网关默认监听端口 `3009`。可通过 `WHATSAPP_GATEWAY_PORT` 覆盖。

**5. 启动 OpenFang：**

```bash
openfang start
# 仪表板运行在 http://localhost:4200
```

**6. 扫描二维码：**

打开仪表板 → **渠道** → **WhatsApp**。会显示一个二维码。用手机扫描：

> **WhatsApp** → **设置** → **已关联设备** → **关联设备**

扫描后，状态变为 `已连接`，收到的消息会路由到你配置的智能体。

### 网关环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `WHATSAPP_WEB_GATEWAY_URL` | OpenFang 连接的网关 URL | _(空 = 禁用)_ |
| `WHATSAPP_GATEWAY_PORT` | 网关监听端口 | `3009` |
| `OPENFANG_URL` | 网关报告的 OpenFang API URL | `http://127.0.0.1:4200` |
| `OPENFANG_DEFAULT_AGENT` | 处理输入消息的智能体 | `assistant` |

### 网关 API 端点

| 方法 | 路由 | 描述 |
|------|------|------|
| `POST` | `/login/start` | 生成二维码（返回 base64 PNG） |
| `GET` | `/login/status` | 连接状态（`disconnected`、`qr_ready`、`connected`） |
| `POST` | `/message/send` | 发送消息（`{ "to": "5511999999999", "text": "你好" }`） |
| `GET` | `/health` | 健康检查 |

### 备选：WhatsApp Cloud API

对于生产工作负载，使用 [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api) 和 Meta Business 账户。参见 [Cloud API 配置文档](https://openfang.sh/docs/channels/whatsapp)。

---

## 27 个 LLM 供应商 — 123+ 模型

3 个原生驱动（Anthropic、Gemini、OpenAI 兼容）路由到 27 个供应商：

Anthropic、Gemini、OpenAI、Groq、DeepSeek、OpenRouter、Together、Mistral、Fireworks、Cohere、Perplexity、xAI、AI21、Cerebras、SambaNova、HuggingFace、Replicate、Ollama、vLLM、LM Studio、Qwen、MiniMax、智谱、Moonshot、千帆、Bedrock 等更多。

智能路由，带任务复杂度评分、自动回退、成本追踪和按模型定价。

---

## 从 OpenClaw 迁移

已经在运行 OpenClaw？一条命令：

```bash
# 迁移一切 — 智能体、记忆、技能、配置
openfang migrate --from openclaw

# 从特定路径迁移
openfang migrate --from openclaw --path ~/.openclaw

# 先试运行查看会变更什么
openfang migrate --from openclaw --dry-run
```

迁移引擎导入你的智能体、对话历史、技能和配置。OpenFang 原生读取 SKILL.md，并与 ClawHub 市场兼容。

---

## OpenAI 兼容 API

直接替换。将你现有的工具指向 OpenFang：

```bash
curl -X POST localhost:4200/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "researcher",
    "messages": [{"role": "user", "content": "分析 Q4 市场趋势"}],
    "stream": true
  }'
```

140+ REST/WS/SSE 端点，覆盖智能体、记忆、工作流、渠道、模型、技能、A2A、工具包等。

---

## 快速开始

```bash
# 1. 安装（macOS/Linux）
curl -fsSL https://openfang.sh/install | sh

# 2. 初始化 — 引导你完成供应商设置
openfang init

# 3. 启动守护进程
openfang start

# 4. 仪表板运行在 http://localhost:4200

# 5. 激活一个工具包 — 它开始为你工作
openfang hand activate researcher

# 6. 与智能体对话
openfang chat researcher
> "AI 智能体框架的新兴趋势是什么？"

# 7. 派生一个预构建智能体
openfang agent spawn coder
```

<details>
<summary><strong>Windows (PowerShell)</strong></summary>

```powershell
irm https://openfang.sh/install.ps1 | iex
openfang init
openfang start
```

</details>

---

## 开发

```bash
# 构建工作区
cargo build --workspace --lib

# 运行所有测试（1,767+）
cargo test --workspace

# 代码检查（必须 0 警告）
cargo clippy --workspace --all-targets -- -D warnings

# 格式化
cargo fmt --all -- --check
```

---

## 稳定性说明

OpenFang v0.5.9 是 pre-1.0。架构稳固，测试套件全面，安全模型完善。但：

- **小版本之间**可能存在破坏性变更，直到 v1.0
- **部分工具包**成熟度不一（Browser 和 Researcher 最为成熟）
- **边缘情况**存在 — 如果发现，请[提交 issue](https://github.com/mastwet/openfang-zh/issues)
- **生产部署**请固定到特定 commit，直到 v1.0

我们快速迭代，快速修复。目标是在 2026 年中发布稳固的 v1.0。

---

## 安全

报告安全漏洞，请发邮件至 **jaber@rightnowai.co**。我们认真对待所有报告，并在 48 小时内回复。

---

## 许可证

MIT — 随意使用。

---

## 链接

- [网站和文档](https://openfang.sh)
- [快速开始指南](https://openfang.sh/docs/getting-started)
- [GitHub](https://github.com/mastwet/openfang-zh)
- [Discord](https://discord.gg/sSJqgNnq6X)
- [Twitter / X](https://x.com/openfangg)

---

## 由 RightNow 构建

<p align="center">
  <a href="https://www.rightnowai.co/">
    <img src="public/assets/rightnow-logo.webp" width="60" alt="RightNow Logo" />
  </a>
</p>

<p align="center">
  OpenFang 由 <a href="https://x.com/Akashi203"><strong>Jaber</strong></a>（<a href="https://www.rightnowai.co/"><strong>RightNow</strong></a> 创始人）构建和维护。
</p>

<p align="center">
  <a href="https://www.rightnowai.co/">网站</a> &bull;
  <a href="https://x.com/Akashi203">Twitter / X</a> &bull;
  <a href="https://www.buymeacoffee.com/openfang" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me a Coffee" style="height: 60px !important;width: 217px !important;" ></a>
</p>

---

<p align="center">
  <strong>用 Rust 构建。16 层安全防护。真正为你工作的智能体。</strong>
</p>