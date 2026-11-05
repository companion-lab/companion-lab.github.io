# Installing Companion

Companion puts a 3D VRM avatar on your desktop that acts as the visual body of
your AI agent. It runs as a transparent overlay — no window chrome, no
background — just your character floating on top of your workspace.

## Requirements

- A desktop running **Windows**, **macOS**, or **Linux**
- An API key from an AI provider (OpenAI-compatible, Anthropic, or Gemini) —
  see [First launch](#3-first-launch) below
- An internet connection to talk to your AI provider

## 1. Download

Grab the latest build from the
[GitHub releases page](https://github.com/companion/companion/releases/latest).
The download buttons on the landing page point at the same place and detect
your platform automatically.

## 2. Run the app

Install instructions differ slightly per platform:

**macOS** — open the downloaded `.dmg` (or `.app`), and drag Companion into
`Applications`. If macOS warns that the app is from an unidentified
developer, **right-click** the app and choose **Open**, then confirm. This
appears for builds that aren't signed with an Apple developer certificate.

**Windows** — run the downloaded installer or executable. If Windows
SmartScreen shows a warning for an unsigned build, click **More info → Run
anyway**.

**Linux** — extract or install the package for your distribution, then launch
the binary. If it isn't executable after download, make it one:

```bash
chmod +x companion
./companion
```

## 3. First launch

On the first run Companion:

1. Shows your avatar as a transparent overlay on the desktop.
2. Seeds the bundled feature plugins (chat, voice, alarms, calendar, MCP
   tools, VRM avatar) into the app data folder — they ship pre-installed and
   can be removed any time in **Settings → Plugins**.
3. Opens the settings window so you can connect an AI provider.

## 4. Connect an AI provider

Head to **Settings → Connection** and pick a provider:

- **OpenAI-compatible** — any endpoint that speaks the OpenAI chat API
  (OpenAI, OpenRouter, a local `llama.cpp` server, …)
- **Anthropic** — Claude models
- **Gemini** — Google's models

Paste your API key and save. Keys are stored in the **operating system
keychain**, not in a plain file.

## Updating

Download the new build from the
[releases page](https://github.com/companion/companion/releases/latest) and
replace the old app. Your settings, plugins, and avatar data live in the app
data folder and survive updates.

## Uninstalling

Delete the app itself. To remove everything (plugins, imported avatars,
settings), also delete the app data directory for Companion.
