# Theasus — Native Windows Keyboard Control Center

**Theasus** is an independent, high-performance native Windows desktop application built in **Rust** for peripheral configuration, low-latency software key remapping, real-time input diagnostics, and HID inspection. 

Engineered with a focus on hardware transparency, low overhead, and safety, Theasus targets the **Cosmic Byte Pandora CBG K26** (`VID: 0x258A`, `PID: 0x002A`) while dynamically detecting and supporting standard USB/HID keyboards.

---

## Key Features

- **Dynamic Identify-by-Keystroke Auto-Detection**:
  - Automatically identifies which physical keyboard you are actively typing on using Windows Raw Input (`WM_INPUT` with `RAWINPUTHEADER.hDevice`).
  - Queries `GetRawInputDeviceInfoW(hDevice, RIDI_DEVICENAME)` to resolve the full NT device path and parse VID/PID.
  - Cross-references against `config/known_devices.json`: confirmed hardware is labeled `Cosmic Byte Pandora CBG K26 (confirmed)`, while unrecognized devices display raw HID parameters without guesswork.
  - Live hotplug detection via `WM_INPUT_DEVICE_CHANGE` updates device state without restarting.

- **Microsecond Low-Level Input Pipeline**:
  - Dedicated background Win32 worker thread running `WH_KEYBOARD_LL`.
  - Non-blocking `crossbeam_channel` streaming keystroke events at microsecond latency.
  - Output injection powered by native `SendInput`.

- **Anti-Recursion Safety Engine**:
  - Injected keystrokes carry a magic signature tag (`dwExtraInfo = 0x4B434331` / `"KCC1"`) and test `LLKHF_INJECTED`.
  - The remapping engine processes only `InputOrigin::Physical` events, preventing recursive input loops and key storms.

- **Observed Host Latency & Jitter Diagnostics**:
  - Sliding-window delta calculation measuring **Median**, **Average**, **Jitter** (Standard Deviation), **Min/Max**, and **Estimated Frequency (Hz)**.
  - Hardware honesty: clearly disclaimed as *Host-Side Latency Measurement (Hardware Polling Rate: Not exposed by this device)* — zero fake polling-rate sliders.
  - 1,000-event circular ring buffer monitor with live filter, pause, clear, and clipboard export.

- **Interactive 68-Key (65% ANSI) Keyboard Visualizer**:
  - 1:1 physical layout matching the Cosmic Byte CBG K26 form factor across 5 rows.
  - Inset 3D keycaps with real-time amber glow on physical press (`#F59E0B`), cyan selection border (`#06B6D4`), and remap destination badges.

- **Profiles & Atomic Persistence**:
  - Shipped with **Default** (1:1 pass-through), **Gaming** (WinKey blocked, CapsLock → Ctrl), and **Work** (CapsLock → Esc) profiles.
  - Create and edit custom profiles with atomic file persistence (`.tmp` write followed by atomic rename) to `%APPDATA%\Theasus\profiles\`.

- **Windows Auto-Startup (Boot, Wake & Restart)**:
  - Full native Windows autorun integration via `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
  - Non-elevated standard user execution without irritating UAC Administrator prompts.
  - One-click configuration directly in the **Settings** view with status indicators and mode selection (**Normal Window** vs **Start Minimized**).
  - Standalone double-clickable scripts (`Enable_AutoStartup.bat` and `Disable_AutoStartup.bat`) and CLI flags (`--register-startup`, `--unregister-startup`, `--minimized`).
  - Seamless Sleep/Wake and Restart handling: low-level input hooks and profile remapping activate immediately as soon as your Windows user session starts.

- **Windows Start Menu & Search Integration**:
  - Automatically installed into `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Theasus.lnk`.
  - Full Windows Search indexing: search "Theasus" or "thesus" directly from the Windows taskbar or Start Menu to launch immediately.
  - Custom high-resolution icon (`assets/theasus.ico`) embedded for Start Menu tiles and search previews.
  - Windows `App Paths` registered (`HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths`): supports launching via `Win + R` Run dialog.
  - One-click setup in **Settings** or via `Install_StartMenu_Shortcut.bat` and CLI flag `--install-shortcut`.

- **Firmware & Protocol Isolation**:
  - SinoWealth MCU vendor protocol writes are quarantined (`src/hid_protocol/mod.rs`) until verified via USB capture, protecting onboard controller EEPROM from bricking.

---

## Application Navigation

| View | Icon | Purpose |
|:---|:---:|:---|
| **Keyboard** | ⌨ | Interactive 68-key visual layout, real-time keypress highlighting, and quick remap assignment |
| **Profiles** | 📁 | Create, duplicate, activate, and delete software profiles |
| **Remap** | 🔀 | Matrix of active key remappings and key blocking rules with interactive key capture mode |
| **Diagnostics** | 📊 | Real-time observed input rate (median/avg/jitter) and live 1000-event logging ring buffer |
| **Device** | 🔌 | HID inspection, VID/PID verification, interface collection enumeration, and capability matrix |
| **Settings** | ⚙ | Runtime architecture details, configuration directory paths, and profile reset options |

---

## Project Structure

```text
theasus/
├── Cargo.toml                # Package configuration & dependencies
├── config/
│   └── known_devices.json    # Confirmed hardware identity database
├── src/
│   ├── main.rs               # Win32 entry point, worker thread supervisor, eframe loop
│   ├── lib.rs                # Crate root exposing all subsystems
│   ├── app/                  # eframe::App implementation, UI state, and theme
│   ├── device/               # Phase 1: hidapi discovery, raw input identity, reconnect
│   ├── input/                # Win32 Raw Input, WH_KEYBOARD_LL hook, injector, normalization
│   ├── remap/                # Remapping engine, rules, and anti-recursion resolver
│   ├── profiles/             # Profile data structures and atomic JSON storage
│   ├── diagnostics/          # Input rate calculator and live event ring buffer monitor
│   ├── keyboard_visual/      # 68-key CBG K26 physical layout and egui canvas renderer
│   ├── hid_protocol/         # Isolated placeholder for future verified vendor reports
│   └── ui/                   # Status bar, sidebar navigation, and modular views
└── tests/
    ├── profile_test.rs       # Profile serialization and factory unit tests
    ├── rate_calc_test.rs     # Rate calculation and jitter sliding window tests
    └── remapper_test.rs      # Anti-recursion and key blocking verification tests
```

---

## Building and Running

### Prerequisites
- **Operating System**: Windows 10 or Windows 11 (x86_64)
- **Toolchain**: [Rust 1.80+](https://www.rust-lang.org/) with MSVC toolchain (`x86_64-pc-windows-msvc`)
- **C++ Build Tools**: Visual Studio Build Tools with C++ workload (required for `windows-sys` / `windows` crates)

### Run Unit Tests
```powershell
cargo test -- --nocapture
```

### Run in Debug Mode
```powershell
cargo run
```

### Build Optimized Release Binary
```powershell
cargo build --release
```
The compiled native executable will be available at:
```text
target/release/theasus.exe
```

---

## Architecture & Safety Guarantees

1. **Zero Browser Wrapping**: Built with 100% native Rust using `eframe` and `egui 0.31` directly rendering via DirectX/OpenGL. No Electron, no Chromium, no localhost HTTP servers.
2. **Infinite Loop Prevention**: Keystrokes injected by Theasus carry `0x4B434331` in `dwExtraInfo`. The hook thread and remapping engine verify `InputOrigin::Physical`, preventing injected keys from re-triggering remapping rules.
3. **Hardware Transparency**: Unverified devices always display raw HID metadata with an `UNKNOWN` status until verified and added to `config/known_devices.json`.
