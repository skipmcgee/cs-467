# Windows Installation and Troubleshooting

## Prerequisites:

1.	Install Git: Install via https://git-scm.com/install/windows
   
2.	Install VS Code: Install via https://code.visualstudio.com/
   
3.	Install Rust: Visit https://rustup.rs and download/install via the Windows Installer (rustup-init.exe) - https://rust-lang.org/tools/install/
   
4.	As a note, OpenSSL is not required for building/flashing firmware for Windows setup (as referenced in `DEV_ENV_SETUP.md` and `macOSSetup.md`) as Windows uses its own built-in SSL libraries.

## Setting up the Development Environment:

1.	Clone the Repo: `git clone git@github.com:skipmcgee/cs-467.git`
   
    - Alternatively, the HTTPS URL can be used instead of the SSH URL (if SSH keys are not configured with GitHub): `git clone https://github.com/skipmcgee/cs-467.git`

2.	Install Clippy: `rustup component add clippy`
	
    - As a note: Clippy typically comes installed already!

3.	Install cross-compilation tool chain : `rustup target install thumbv6m-none-eabi`
   
4.	Install stack overflow protection: `cargo install flip-link`
   
5.	Install the flashing tools: `cargo install --locked probe-rs-tools`
   
6.	Install `Picotool`: Install via https://github.com/raspberrypi/picotool
    
    - As a note: udev steps detailed in `DEV_ENV_SETUP.md` and `macOSSetup.md` does not apply to Windows. Please disregard udev steps during Picotool installation.

## Troubleshooting:
1.	`cargo install clippy` might fail with an error “Clippy is no longer available via crates.io” in which case, you can use rustup component add clippy as an alternative.

## Resources:
1. Rust Install: https://rustup.rs/
2. Git Install: https://git-scm.com/install/windows
3. VS Studio Install: https://code.visualstudio.com/
4. Project Template: https://github.com/rp-rs/rp2040-project-template
5. Picotool Install: https://github.com/raspberrypi/picotool

