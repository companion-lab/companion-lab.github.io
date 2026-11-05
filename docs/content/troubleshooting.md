# Troubleshooting

Answers to the most common questions about Companion.

## The app warns it's from an "unidentified developer"

Builds aren't signed with Apple/Windows developer certificates, so the OS
shows a one-time warning:

- **macOS** — right-click the app and choose **Open**, then confirm.
- **Windows** — SmartScreen: click **More info → Run anyway**.

This only appears on first launch.

## The microphone isn't working

Speech-to-text runs **on your device** — no audio is uploaded — but the OS
still needs to grant microphone access to the app. Check your system privacy
settings and allow microphone access for Companion, then restart the app.

## The avatar isn't speaking

Check the voice settings in **Settings → Connection** (speech section): the
active TTS engine and voice. If the voice plugin was uninstalled, the avatar
stays silent — reinstall it in **Settings → Plugins**.

## My API key was rejected

- Keys live in the OS **keychain**; if you changed providers, update the key
  in **Settings → Connection**.
- For OpenAI-compatible endpoints, double-check the base URL and that the
  endpoint speaks the OpenAI chat API.

## A plugin won't load

- Plugins built against an older SDK are refused at load time — rebuild the
  plugin against the current SDK (`abi: 2` in the manifest).
- A backend that keeps erroring is wrapped in `catch_unwind`: it degrades to
  "every call errors" instead of crashing the app. Restart the app after
  updating a plugin — backends are never unloaded while the app runs.
- Uninstalled plugins stay uninstalled across restarts (tombstoned); use
  **Restore bundled features** in **Settings → Plugins** to bring bundled
  ones back.

## The avatar feels slow

Open **Settings → Performance** and lower the graphics settings or adjust the
idle mixer. The avatar is a real-time 3D scene, so frame budget scales with
model complexity and window size.

## Something is broken — where do I report it?

Open an issue on
[GitHub](https://github.com/companion-vr) with your OS, the app version, and
what you did right before the problem. Include any red text from the error
overlay if one appeared.
