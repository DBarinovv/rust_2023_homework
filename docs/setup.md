# Workspace Setup

This document is to help you to set up your course workspace.

## Operating System

Linux and MacOS support Rust and Cargo. For Windows, you will have to install [Wsl](https://learn.microsoft.com/ru-ru/windows/wsl/install-manual).

## Setup process

### Step 1 - Installing Rust and C linker

Install `rustup` either [using the official guide](https://www.rust-lang.org/tools/install) or using your package manager such as `apt`, `pacman` or `brew`.

On Linux, you probably have the C language linker, but make sure you already have it by installing `build-essential` using your package manager.

On MacOS, users have to install XCode tools to get a C language linker.

```shell
xcode-select --install
```

Run this command to get the stable Rust compiler:

```shell
rustup update stable
```

### Step 2 - VS Code and plugins

The only supported editor in the course is [Visual Studio Code](https://code.visualstudio.com). Install it. Then, install the following plugins:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - language server, your best friend.
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - syntax highlight for `.toml` files.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - optional, needed for debugging.

_IDE such as CLion or editors with plugins such ad Vim will work perfectly because of Rust's nature, but the lecturer doesn't use them and won't support them officially._

### Step 3 - Cloning repository

Clone the repository:

```shell
git clone https://github.com/DBarinovv/rust_2023_homework.git
cd rust_2023_homework
```

### Step 4 - First solution

Read the document about [solving and submitting problems](solving.md). Solve the [add](https://github.com/DBarinovv/rust_2023_homework/tree/master/problems/tutorial/add) problem.
