# Development Environment Directions

## Prerequisites

1. Install OpenSSL: `sudo apt install openssl`
2. Install OpenSSL Development Tools: `sudo apt install libssl-dev`
3. Install VSCode

## How to set up the Development Environment

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone the Repo: ` git clone git@github.com:skipmcgee/cs-467.git`
3. Install the cross-compilation toolchain: `rustup target install thumbv6m-none-eabi`
4. Install stack overflow protection: `cargo install flip-link`
5. Install the flashing tools: `cargo install --locked probe-rs-tools`
6. (Optional) Install debugger for probe-rs via the VSCode extensions menu (View > Extensions)
7. Set up probe.rs per <https://probe.rs/docs/getting-started/probe-setup/>:

    - Copy Udev Rules to device: `sudo cp files/69-probe-rs.rules  /etc/udev/rules.d/`
    - `sudo udevadm control --reload`
    - `sudo udevadm trigger`
    - `sudo groupadd --system plugdev`
    - `sudo usermod -a -G plugdev $USER`

## Initial template setup

This section documents the commands use to initially set up the repository.

1. Install cargo-generate: `cargo install cargo-generate`
2. Create the template: `cargo generate rp-rs/rp2040-project-template`
    - Name: `humidity-sensor`
    - Flashing method: `probe.rs`
3. Install the cross-compilation toolchain: `rustup target install thumbv6m-none-eabi`
4. Install stack overflow protection: `cargo install flip-link`
5. Install the flashing tools: `cargo install --locked probe-rs-tools`

## Resources

1. Project template: <https://github.com/rp-rs/rp2040-project-template>
