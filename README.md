# applications-starter

Solve the TODOs from the [ls.rs](src/ls.rs) and [ui.rs](src/ui.rs)

Open Codespaces:
- Code
- Codespaces
- Create Codespace

Rust and ncurses should already be installed in your container, if not,
the steps on how to do it are described below:

Install Rust on the codespace

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Install ncurses

```bash
sudo apt-get install libncurses5-dev libncursesw5-dev
```

Install *rust-analyzer* for VSCode
