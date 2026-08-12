# dystellar-launcher-rs

A native desktop launcher for the Dystellar Network Minecraft server, written in Rust with a [Slint](https://slint.dev) UI.

The launcher handles the whole path from "user opens the app" to "the game is running": Microsoft/Xbox Live login, fetching the Mojang version manifest, downloading the client, libraries, natives and assets, provisioning a matching Azul Zulu JRE, and finally spawning the game process — all with a live, multi-threaded task manager showing progress.

## Features

- **Microsoft account login** via an OAuth device-style flow brokered by the Dystellar backend (the launcher never sees your Microsoft password).
- **Persistent sessions** — access/refresh tokens are AES-256 encrypted (`magic-crypt`) and stored in an embedded [`sled`](https://docs.rs/sled) database, so you only log in once.
- **Automatic game provisioning** — client jar, libraries, OS-specific natives and all assets are resolved from Mojang's official manifests and downloaded on demand.
- **Automatic JRE management** — the required Java major version is read from the Minecraft manifest and a matching Zulu JRE is fetched from the Azul metadata API for the current OS/arch, then unpacked into the launcher's data directory. No system Java required.
- **Integrity checking** — every download is SHA-1 verified against the manifest; corrupt files are deleted and reported.
- **Parallel task manager** — downloads, verification and unpacking run across a worker pool sized to `available_parallelism()`, with a semaphore-gated queue and a Slint overlay reporting per-task state (`waiting` → `starting` → `in_progress` → `verifying`/`unpacking` → `finished`/`failed`).
- **Configurable launch** — RAM allocation, resolution, fullscreen, extra JVM args, branch and custom game/cache/JDK directories, persisted as JSON.
- **Cross-platform** — Linux (glibc/musl), Windows and macOS, on x86_64 and aarch64.

## Requirements

| | |
|---|---|
| Rust | 1.88+ (edition 2024, let-chains); developed on 1.94 |
| Toolchain | A C/C++ compiler — the Slint Skia renderer builds native code |
| Linux extras | `fontconfig` and standard X11/Wayland dev packages, as required by `winit` |

## Building

The build is driven by `build.rs`, which compiles `ui/app.slint` into `$OUT_DIR/app.rs` and bakes three values into the binary from a `.env` file at the repository root (loaded with `dotenv`). **The build will fail without it.**

Create `.env`:

```dotenv
# Used for every build
CRYPT_PASS=<key used to encrypt stored session tokens>

# Used for `--release` builds
PROD_CLIENT_ID=<Azure application (client) ID>
PROD_BACKEND_URL=https://api.example.com

# Used for debug builds
TEST_CLIENT_ID=<Azure application (client) ID>
TEST_BACKEND_URL=http://localhost:8080
```

`build.rs` selects the `PROD_*` pair when `PROFILE == "release"` and the `TEST_*` pair otherwise, exposing them to the code as the `CLIENT_ID`, `BACKEND_URL` and `CRYPT_PASS` compile-time environment variables.

> `.env` is git-ignored. `CRYPT_PASS` is embedded in the binary, so treat it as an obfuscation key for local session storage, not as a secret.

Then:

```sh
cargo run              # debug build against TEST_BACKEND_URL
cargo build --release  # release build against PROD_BACKEND_URL
```

The release profile is tuned for a small binary: `opt-level = "z"`, fat LTO, one codegen unit, `panic = "abort"`, no debug info.

## Project layout

```
build.rs                    Slint compilation + .env → compile-time env vars
src/
  main.rs                   Entry point; falls back to an error dialog if startup fails
  logic.rs                  Event-loop helper (`safe`) and social link handlers
  api/
    config.rs               Config struct, JSON (de)serialization, defaults
    control/
      http.rs               Backend + Mojang + Azul HTTP calls, login flows
      database.rs           Encrypted sled session storage
      dir_provider.rs       Per-OS data/cache directories
    typedef/
      manifest.rs           Minecraft + Java manifest models
      ms_session.rs         Microsoft session model
      task_manager.rs       Task trait, worker pool, semaphore, UI sync timer
      implementation.rs     HttpDownloadTask + SHA-1 / natives / archive post-scripts
  ui/
    launcher.rs             Main window wiring, session bootstrap, callbacks
    launch.rs               Manifest → download tasks → JVM command construction
    dialogs.rs              Standalone dialogs and the welcome/login window
ui/                         Slint components, fonts (Orbitron) and icons
```

`ui/app.slint` is the single entry point that re-exports every window (`Main`, `WelcomeUI`, `ModsUI`, `ModInfo`, `FallbackDialog`) and the `Callbacks` global; Rust reaches them through the generated `crate::generated` module.

## Runtime data

Paths come from [`directories`](https://docs.rs/directories) using the qualifier `org.dystellar`, organization `mmorpg`, application `DystellarLauncher` — e.g. `~/.local/share/dystellarlauncher` on Linux, `%APPDATA%\mmorpg\DystellarLauncher\data` on Windows, `~/Library/Application Support/org.dystellar.mmorpg.DystellarLauncher` on macOS.

| Path | Contents |
|---|---|
| `<data>/config.json` | User configuration (created with defaults on first run) |
| `<data>/storage/` | sled database holding the encrypted session |
| `<data>/client.jar` | Minecraft client |
| `<data>/libs/` | Libraries, laid out by manifest path |
| `<data>/natives/` | Extracted native libraries (`.so`/`.dll`/`.dylib`) |
| `<data>/assets/objects/` | Hash-addressed asset objects |
| `<data>/jdk/` | Provisioned Zulu JRE (`jdk_dir`) |
| `<data>/dystellar/` | Game directory (`game_dir`) |
| `<cache>/dyst/` | Download scratch space (`cache_dir`) |

### Configuration

`config.json` defaults:

```json
{
  "ram_allocation_mb": 3072,
  "resolution": "854x480",
  "fullscreen": false,
  "branch": "master",
  "jvm_args": "",
  "close_on_launch": false,
  "game_dir": "<data>/dystellar",
  "cache_dir": "<cache>/dyst",
  "jdk_dir": "<data>/jdk"
}
```

`resolution` is parsed from `"<width>x<height>"`. `jvm_args` is split on spaces and appended to the generated JVM arguments.

## Backend contract

The launcher talks to the Dystellar backend at `BACKEND_URL`:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/launcher` | Returns `{ "minecraft_version": "..." }` — the version to launch |
| `POST` | `/api/microsoft/loginsession` | Registers a login attempt keyed by a client-generated UUID |
| `GET` | `/api/microsoft/login?uuid=…` | Polled every 1.5 s until `authenticated` is true; returns the session |
| `POST` | `/api/microsoft/login_existing` | Exchanges stored access/refresh tokens for a fresh session |
| — | `/api/microsoft/callback` | OAuth redirect URI registered with the Azure application |
| — | `/discord` | Social link opened in the system browser |

A successful login response carries `uuid`, `username`, `minecraft_token`, `access_token`, `refresh_token` and `uhs`; any missing field is treated as a fatal session error.

Third-party endpoints used directly: Mojang's `piston-meta` version manifest, `resources.download.minecraft.net` for assets, Azul's Zulu package metadata API, and `login.microsoftonline.com` for the consumer OAuth authorize URL.

## License

GNU General Public License v3.0 — see [LICENSE](LICENSE).
