# Mojo language support for Zed

Supercharged Zed extension that brings first-class Mojo language support for `.mojo` and `.🔥` files, updated for **Mojo 1.0 & 1.1+**.

## What is included

- **Language Configuration**: `.mojo` and `.🔥` file association, `#` line comments, triple-quote `"""` block docstrings, smart bracket pairs, and auto-closing.
- **Intelligent LSP Resolution**: Zero-config auto-discovery for `mojo-lsp-server` across project environments (`.magic`, `.pixi`, `.venv`), user installations (`~/.mojo/bin`, `~/.modular`, `~/.magic`), and system paths (`/opt/homebrew/bin`, `/usr/local/bin`), with automatic `PATH` environment augmentation.
- **Modern Syntax Highlighting**: Full tree-sitter highlighting updated for Mojo 1.0/1.1 including `def`, `comptime`, modern stdlib types (`Span`, `Array`, `InlineArray`, `Pointer`, `SIMD`, `Byte`, etc.), builtins (`size_of`, `type_of`, `global_constant`), argument conventions (`mut`, `ref`, `out`, `deinit`), and decorators (`@always_inline`, `@export`, `@value`, `@fieldwise_init`).
- **Editor Queries**:
  - **Indentation**: Python-style indentation with support for `def`, `struct`, `trait`, `class`, `if/elif/else`, `match/case`, `try/except/finally`, and `comptime`.
  - **Outlines**: Symbol outline (`Cmd+Shift+O` / `Ctrl+Shift+O`) indexing `def`, `struct`, `trait`, `class`, `comptime`, and variables.
  - **Runnables**: Inline gutter runnable indicators for both entry points (`def main`) and test functions (`def test_...`).
  - **Vim Text Objects**: Inside and around selections for functions, classes, structs, traits, parameters, and comments.
- **Rich Snippets**: Comprehensive snippets updated for modern Mojo (`comptime`, `struct`, `@value`, `Span`, `Array`, `SIMD`, `std.python`, `std.testing`, etc.).
- **Zed Tasks**: Ready-to-use tasks for running, testing, building, formatting, generating docs, and launching the Mojo REPL.
- **Tree-sitter Grammar**: Pinned to [`vadim-su/tree-sitter-mojo`](https://github.com/vadim-su/tree-sitter-mojo).

## Install locally in Zed

1. Open Zed.
2. Open the command palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) and run `zed: extensions`.
3. Click **Install Dev Extension**.
4. Select this repository directory: `zed_mojo`.
5. Open any `.mojo` or `.🔥` file.

If the extension does not appear immediately, reload Zed with `zed: reload window`.

## Mojo Language Server (LSP)

This extension registers `mojo-lsp-server` as the language server for Mojo files. Auto-completion, diagnostics, hover definitions, go-to-definition, and signature help are provided through the LSP.

### Automatic Binary Resolution

The extension automatically searches for `mojo-lsp-server` in the following locations in order:

1. Active `PATH` environment.
2. Workspace-local virtual environments:
   - `<worktree>/.magic/envs/default/bin/mojo-lsp-server`
   - `<worktree>/.pixi/envs/default/bin/mojo-lsp-server`
   - `<worktree>/.venv/bin/mojo-lsp-server`
3. Standard user home directories:
   - `~/.mojo/bin/mojo-lsp-server` (standard official installer path)
   - `~/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server`
   - `~/.magic/envs/default/bin/mojo-lsp-server`
   - `~/.local/bin/mojo-lsp-server`
4. System package managers:
   - `/opt/homebrew/bin/mojo-lsp-server`
   - `/usr/local/bin/mojo-lsp-server`

When resolved, the binary's directory is automatically prepended to the server's `PATH` environment to ensure companion tools (such as `lldb` or Python runtimes) can be found.

### Custom Configuration

You can override the binary path (with `~` expansion support), extra arguments, or environment variables in your Zed `settings.json`:

```json
{
  "lsp": {
    "mojo-lsp-server": {
      "binary": {
        "path": "~/.mojo/bin/mojo-lsp-server",
        "arguments": ["-I", "/path/to/mojo/packages"],
        "env": {}
      },
      "initialization_options": {},
      "settings": {}
    }
  }
}
```

## Running Code & Zed Tasks

Zed executes code through tasks. This extension provides gutter runnable triggers and pre-configured tasks in `languages/mojo/tasks.json`:

| Task Label | Command | Description | Run Trigger |
| :--- | :--- | :--- | :--- |
| `mojo run current file` | `mojo run $ZED_FILE` | Executes the active Mojo file | Gutter indicator on `def main` (`mojo-main`) |
| `mojo test current file` | `mojo run $ZED_FILE` | Executes test runner on the active file | Gutter indicator on `def test_...` (`mojo-test`) |
| `mojo build current file` | `mojo build $ZED_FILE -o build/$ZED_STEM` | Compiles an executable to the `build/` folder | Command Palette / Tasks |
| `mojo format current file` | `mojo format $ZED_FILE` | Formats current file with official formatter | Command Palette / Tasks |
| `mojo doc current file` | `mojo doc $ZED_FILE` | Compiles docstrings from the active file | Command Palette / Tasks |
| `mojo repl` | `mojo repl` | Launches interactive Mojo REPL in a new terminal | Command Palette / Tasks |

## Modern Snippets

Snippets are updated for Mojo 1.0 & 1.1 (`def` syntax, `comptime`, and modern stdlib):

- **Functions**: `main`, `mainr` (raising main), `def`, `defr`, `method`, `mutmethod`, `staticmethod`
- **Types & Structs**: `struct`, `valuestruct` (`@value`), `fieldwise` (`@fieldwise_init`), `trait`
- **Initializers & Lifecycle**: `init` (`out self`), `copyinit`, `moveinit` (`deinit take: Self`), `del` (`deinit self`)
- **Constants & Bindings**: `comptime`, `alias`, `var`, `ref`
- **Collections & Memory**: `array`, `inlinearray`, `span`, `simd`, `list`, `dict`
- **Interop & Testing**: `pyimport` (`std.python`), `test`, `assert_eq`, `assert_true`
- **Control Flow**: `for`, `while`, `if`, `ifelse`, `try`, `tryfinally`, `raise`

## Development & Verification

Run Rust formatting and compilation checks:

```sh
cargo fmt --check
cargo check
cargo check --target wasm32-wasip1
```

Run query and snippet validation tests:

```sh
bash scripts/check-snippets.sh
bash scripts/check-indents.sh
bash scripts/check-highlight-order.sh
bash scripts/check-runnables.sh
```

