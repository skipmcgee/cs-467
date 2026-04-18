# MacOS Installation and Troubleshooting
## Rust and Cargo Installation: 
### 1.	Rust Installation
a.	Full instructions are detailed in: https://rust-lang.org/tools/install/ 

b.	Use Rustup tool to install and manage Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

c.	Rustup also automatically installs Cargo. 

## Development Environment Prerequisites: 
### 1.	`sudo apt install openssl`: 
**MacOS Modification**: 

a.  Package manager apt not available. Instead, use Homebrew as package manager. 

b.	If Homebrew has not been set up, install using: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"` 

c.	Once Homebrew is installed, run: `brew install openssl` 

d.	MacOS has some components that depend on LibreSSL. You have to specify that OpenSSL will be the default, adding it to PATH for shell session: 
`/usr/local/opt/openssl/bin/openssl version` 
`export PATH="/opt/homebrew/opt/openssl/bin:$PATH"`

### 2.	`sudo apt install libssl-dev`: 
**MacOS Modification**: 

a. Equivalent library is already installed by Homebrew when openssl is installed 

### 3.	Install VS Code:
**MacOS Installation**:  

a. https://code.visualstudio.com/docs/setup/mac#_install-vs-code-on-macos


## Set Up the Development Environment:
### 1.	Clone Repo: `git clone git@github.com:skipmcgee/cs-467.git` 

a.	Note: if SSH key is not in place to connect Github account, follow instructions detailed here: https://github.com/rp-rs/rp2040-project-template 

### 2.	Install Clippy: cargo install clippy
a.	MacOS Modification: If you used rustup, Clippy is usually already installed. 
b.	To confirm or update, you can run: `rustup component add clippy-preview`

### 3.	Install the cross-compilation toolchain: `rustup target install thumbv6m-none-eabi`

a.	No modification needed for MacOS

### 4.	Install stack overflow protection: `cargo install flip-link`

a.	No modification needed for MacOS

### 5.	Install the flashing tools: `cargo install --locked probe-rs-tools`

a.	No modification needed for MacOS

### 6.	Install picotool via the repository directions:

a.  https://github.com/raspberrypi/picotool

## Resources: 
1.	https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent 
2.	https://fixmycert.com/guides/openssl-installation#macos 
3.	https://code.visualstudio.com/docs/setup/mac#_install-vs-code-on-macos 
4.	https://rust-lang.org/tools/install/ 
5.	https://github.com/rp-rs/rp2040-project-template
