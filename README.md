# Tauri Python POC

## Quick Start

Read the prerequisites at [Tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites).
You also need Python 3.12 or higher installed on your machine.

1. Install dependencies using yarn: `pnpm install`
2. Run development with command: `pnpm tauri dev`
3. Build executable file: `pnpm tauri build`

> NOTICE: Currently, only macOS with Apple Silicon chips is supported.

## Debugging

### Fix Unsigned Warning (macOS)

> Warning: "Tauri Python" is damaged and can't be opened.

This warning is shown because the build is not signed. Run the following command to suppress this warning:

```sh
xattr -r -d com.apple.quarantine "/Applications/Tauri Python.app"
```
