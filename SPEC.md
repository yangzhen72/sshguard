# SSHGuard Specification

## Overview
SSHGuard is a Windows desktop application for managing SSH connections with built-in terminal and SFTP support.

## Features
- Multi-server connection management with groups and tags
- Encrypted local storage for credentials
- Multi-tab terminal with xterm.js
- SFTP file browser with upload/download
- Terminal-follows-SFTP navigation
- Folder packaging for batch uploads/downloads

## Technical Stack
- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust with Tauri 2
- **Terminal**: xterm.js + ssh2 crate
- **SFTP**: ssh2 crate (Sftp type)
- **Storage**: Encrypted SQLite

## UI Layout
- Single-window tabbed interface
- Left sidebar: Server list with groups
- Center: Terminal tabs
- Right panel: SFTP file browser
- Bottom: Status bar

## Security
- SQLite database encrypted with age/AES
- User master password for key derivation (PBKDF2)
- SSH credentials never stored in plaintext
- Support for SSH agent authentication

## Keyboard Shortcuts
- `Ctrl+T`: New terminal tab
- `Ctrl+W`: Close current tab
- `Ctrl+Tab`: Next tab
- `Ctrl+Shift+Tab`: Previous tab
- `Ctrl+L`: Clear terminal
- `F5`: Refresh SFTP directory

## File Structure
```
sshguard/
├── src/                              # Vue frontend
│   ├── main.ts                        # Vue app entry
│   ├── App.vue                        # Root component
│   ├── components/
│   │   ├── Sidebar.vue               # Server list sidebar
│   │   ├── ServerTree.vue            # Tree view for servers
│   │   ├── TabBar.vue                # Tab navigation
│   │   ├── TerminalPanel.vue         # Terminal container
│   │   ├── SftpPanel.vue             # SFTP file browser
│   │   ├── FileEditor.vue            # Code editor component
│   │   ├── ServerForm.vue            # Add/edit server form
│   │   └── StatusBar.vue             # Bottom status bar
│   ├── stores/
│   │   ├── servers.ts                # Server config state (Pinia)
│   │   ├── terminals.ts               # Terminal sessions state
│   │   └── sftp.ts                   # SFTP state
│   ├── views/
│   │   ├── MainLayout.vue            # Main window layout
│   │   └── Settings.vue               # Settings page
│   ├── styles/
│   │   └── main.css                  # Global styles
│   └── types/
│       └── index.ts                  # TypeScript types
├── src-tauri/                        # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/
│       ├── main.rs                   # Tauri entry
│       ├── lib.rs                    # Library root
│       ├── commands/                  # Tauri commands
│       │   ├── servers.rs            # Server CRUD commands
│       │   ├── ssh.rs                # SSH connection commands
│       │   └── sftp.rs               # SFTP commands
│       ├── storage/                  # Storage modules
│       │   ├── database.rs           # SQLite operations
│       │   └── encryption.rs         # Age encryption
│       ├── ssh/                      # SSH modules
│       │   ├── session.rs            # SSH session management
│       │   └── pty.rs               # PTY handling
│       └── sftp/                     # SFTP modules
│           └── client.rs             # SFTP client wrapper
├── package.json
├── vite.config.ts
├── tsconfig.json
└── SPEC.md
```

## Build Commands
- `npm run dev` - Start development server
- `npm run build` - Build frontend
- `npm run tauri dev` - Start Tauri development
- `npm run tauri build` - Build Tauri application
