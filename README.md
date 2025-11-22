<div align="center">
   <img src="assets/logo.png" width="120" alt="Aeonium Logo" />
</div>

<h1 align="center">Aeonium Menu</h1>

> [!WARNING]
> This project is under heavy development. Many features are incomplete and things will break often. Use at your own risk.

Aeonium Menu is a ring-style application launcher, inspired by tools like
**rofi** and **dmenu**. Unlike traditional linear menus, Aeonium presents
shortcuts in a circular layout, making it natural to navigate with a **mouse**,
**trackball**, or **encoder knob**.

Keyboard-only usage is also supported for minimal setups, but not recommended.

The name of this project is taken from flowers of the same name.

---

## Build Instructions
> [!Note]
> The main binary expects the GUI binary to be at the path `~/.local/share/aeonium` which is why installation is required before usage and testing.
> If you wish to clean everything simply delete `$(BIN_INSTALL_DIR)/aeonium-menu,` `~/.local/share/aeonium/` and `~/.config/aeonium/` if any of those were created.

Ensure `BIN_INSTALL_DIR` in the Makefile points to a value in `$PATH`.

#### Build all binaries:
```bash
make build
````

#### Install and run:
```bash
make install && aeonium-menu
```

## Usage
> [!WARNING]
> All key binds are currently hard-coded. Configuration is not yet implemented.

### Requirements
- The user running the application must be added to the `input` group to
  allow global key bind and scroll detection through `libinput`.
- Add your `.desktop` shortcuts to `~/.config/aeonium-menu/shortcuts/` (Fallback to `~/.local/share/applications`)

### Current controls:

#### Modifiers (must be held):
  - <kbd>Ctrl</kbd>
  - <kbd>Shift</kbd>

#### Menu activation:
  - <kbd>F9</kbd>: Move down
  - <kbd>F10</kbd>: Move up
  - Scroll with the mouse wheel or encoder knob

## Logo Credits
#### Designed by somdu77a:
- [behance.net/somdu77a](behance.net/somdu77a)
- [x.com/somdu77a](x.com/somdu77a)
