# Aeonium Menu

> [!WARNING]
> This project is under heavy development. Many features are incomplete and things will break often. Use at your own risk.

Aeonium Menu is a ring-style application launcher, inspired by tools like
**rofi** and **dmenu**. Unlike traditional linear menus, Aeonium presents
shortcuts in a circular layout, making it natural to navigate with a **mouse**,
**trackball**, or **encoder knob**.

Keyboard-only usage is also supported for minimal setups, but not recommended.

The name of this project is taken from flowers of the same name.

---

## Planned Features
- [ ] Systray Icon
- [ ] Heavy customisability
- [ ] Configuration menu
- [ ] Integration with systemd and openrc

## Build Instructions

#### Build all binaries:

```bash
cargo build
````

This step ensures that all executables (including those in `src/bin/`) are
compiled and discoverable, only then can the main application be run.

#### Run main application:

```bash
cargo run
```

## Usage
> [!WARNING]
> All key binds are currently hard-coded. Configuration is not yet implemented.

### Requirements
- The user running the application **must be added to the `input` group** to
  allow global key bind and scroll detection through `libinput`.

### Current controls:

#### Modifiers (must be held):
  - <kbd>Ctrl</kbd>
  - <kbd>Shift</kbd>

#### Menu activation:
  - <kbd>F9</kbd>: Move down
  - <kbd>F10</kbd>: Move up
  - Scroll with the mouse wheel or encoder knob
