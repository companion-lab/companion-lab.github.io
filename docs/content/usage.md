# Using Companion

Your companion is an AI agent with a visual body: it listens, thinks, speaks
out loud with lip sync, and acts with motions and expressions.

## Talk to your companion

- **Speak** — the voice plugin listens on-device (no audio leaves your
  machine) and transcribes your speech. Use the **push-to-talk hotkey** or the
  **wake word** to start a turn.
- **Type** — open the chat panel to send messages and watch replies stream in
  token by token.
- Replies are spoken aloud with on-device text-to-speech, and the mouth moves
  in sync via viseme lip sync.

Configure providers, voices, and the wake word in **Settings** (the voice and
connection tabs).

## What you can ask

Companion exposes your installed features as tools the model can call, so you
just talk naturally:

- **Alarms** — "set an alarm for 7am", "remind me every weekday at 9",
  "remind me in 10 minutes", "list all my alarms", "pause that alarm". Alarms
  persist to disk and are also editable in **Settings → Alarms**. When one
  rings, a banner appears with snooze controls.
- **Calendar** — "add my dentist appointment to Friday", "when is mom's
  birthday?" Dated and repeating events, birthday reminders, and month/agenda
  views live in **Settings → Calendar**.
- **MCP tools** — add any MCP server (stdio or streamable HTTP) in
  **Settings → MCP** and its tools become available to the model. Playwright
  works out of the box for browser tasks. Tools are only offered when a
  message plausibly needs them.

## Make the avatar act

Every reply carries a motion and an expression — you can request them
directly:

- "do a nod" → plays the built-in `nod`
- "do the reading loop motion" → plays a clip you imported
- "look angry" → sets the `angry` expression

Motion and expression names come from the built-ins plus every clip you
import, and the model is told the full list on every request — so custom
names work as soon as they're imported. If nothing matches, the avatar says
so instead of inventing a name.

## Customize the avatar

- **Import a model** — load your own `.vrm` character instead of the bundled
  one.
- **Import motions** — add `.vrma` motion clips; they appear in the idle
  mixer and are callable by name.
- **Emotion presets** — define your own expressions.

All of this lives in the model and motion sections of the settings window.

## Manage plugins

Companion is plugin-based. In **Settings → Plugins** you can:

- Disable or uninstall any bundled feature you don't use
- Restore removed bundled features with the **Restore bundled features**
  button
- Import third-party plugins packaged as zip files — see
  [Creating a plugin](/docs/creating-plugins)

## Tuning performance

If the avatar feels heavy, **Settings → Performance** exposes graphics and
idle-mixer options to trade polish for frame budget.
