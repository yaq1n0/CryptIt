# CryptIt - File Encryption with Shamir Secret Sharing

A secure, cross-platform file encryption application built with Tauri, Vue 3, and Rust. CryptIt uses AES-256-GCM encryption combined with Shamir Secret Sharing to provide enterprise-grade security for your sensitive files.

![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![Vue](https://img.shields.io/badge/Vue-3.0+-green)
![Tauri](https://img.shields.io/badge/Tauri-2.0+-purple)

## 🔐 Features

### Core Functionality

- **File Encryption/Decryption**: Secure AES-256-GCM encryption for any file type
- **Shamir Secret Sharing**: Split encryption keys into k-of-n shares for enhanced security
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux
- **Simple UI**: Intuitive interface for selecting files, configuring shares, and managing encryption

### Security Features

- **AES-256-GCM**: Industry-standard encryption with authenticated encryption
- **Memory Safety**: Rust-powered backend with automatic memory cleanup
- **No Key Persistence**: Keys are never stored permanently
- **Secure Random Generation**: Cryptographically secure random number generation
- **Custom File Format**: Secure `.cryptit` file format with integrity protection

## 🏗️ Architecture & Design Decisions

### Technology Stack

- **Frontend**: Vue 3 + TypeScript + Vite for reactive UI
- **Backend**: Rust + Tauri for native performance and security
- **Crypto**: `aes-gcm` crate for encryption, `shamirs` crate for secret sharing
- **UI**: Native file dialogs via `tauri-plugin-dialog`

### Design Decisions

1. **Single Algorithm Focus**: Started with AES-256-GCM for MVP simplicity
2. **Modular Architecture**: Easy to extend with additional algorithms
3. **Secure by Default**: Tauri v2 permissions model for maximum security
4. **Simple File Format**: Custom `.cryptit` format with nonce and ciphertext
5. **Memory-Safe Operations**: Automatic cleanup of sensitive data

## 🎯 MVP Constraints

This MVP was designed to be functional within ~1 hour of development time:

### What's Included

- ✅ Single encryption algorithm (AES-256-GCM)
- ✅ Shamir Secret Sharing with configurable k-of-n thresholds
- ✅ File and directory selection dialogs
- ✅ Basic encrypt/decrypt workflow
- ✅ Share generation and input handling
- ✅ Cross-platform compatibility

### What's Not Included (Yet)

- ❌ Multiple encryption algorithms
- ❌ Batch file processing
- ❌ Share export to files/QR codes
- ❌ Progress bars for large files
- ❌ File metadata preservation
- ❌ Settings/preferences
- ❌ Advanced error handling UI

## 🚀 Future Extensions

### Planned Features

1. **Multiple Algorithms**: ChaCha20-Poly1305, AES-256-CTR support
2. **Advanced Share Management**:
   - Export shares to separate files
   - QR code generation for shares
   - Share validation and testing
3. **Enhanced UI/UX**:
   - Progress indicators for large files
   - Drag & drop file selection
   - Dark/light theme toggle
   - Batch file processing
4. **Security Enhancements**:
   - Hardware security module (HSM) integration
   - Key derivation from passwords
   - File integrity verification
5. **Enterprise Features**:
   - Audit logging
   - Policy management
   - Centralized share storage

### Potential Integrations

- **Cloud Storage**: Secure cloud backup of encrypted files
- **Password Managers**: Integration with 1Password, Bitwarden
- **Hardware Tokens**: YubiKey support for additional authentication
- **Mobile Apps**: React Native companion for share management

## 🛠️ Development

### Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+
- **System Dependencies**:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libssl-dev`
  - **Windows**: Microsoft Visual Studio C++ Build Tools

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd CryptIt

# Install dependencies
npm install

# Run development server
npm run tauri dev
```

### Development Commands

```bash
# Start development server with hot reload
npm run tauri dev

# Run Rust tests
cd src-tauri && cargo test

# Run frontend linting
npm run lint

# Type checking
npm run type-check

# Build for development (faster, larger)
npm run tauri build -- --debug
```

## 📦 Building & Distribution

### Production Build

```bash
# Build optimized production version
npm run tauri build

# Build with specific target
npm run tauri build -- --target x86_64-pc-windows-msvc

# Build for all platforms (requires setup)
npm run tauri build -- --target universal-apple-darwin
```

### Platform-Specific Builds

```bash
# macOS Universal Binary (Intel + Apple Silicon)
npm run tauri build -- --target universal-apple-darwin

# Windows (x64)
npm run tauri build -- --target x86_64-pc-windows-msvc

# Linux (x64)
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Output Locations

- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## 🧪 Testing

### Automated Testing

```bash
# Run all Rust unit tests
cd src-tauri && cargo test

# Run with verbose output
cd src-tauri && cargo test -- --nocapture

# Test specific module
cd src-tauri && cargo test crypto::tests
```

### Manual Testing Checklist

- [ ] File encryption with various file types and sizes
- [ ] Share generation with different k-of-n configurations
- [ ] Decryption with minimum required shares
- [ ] Error handling for insufficient shares
- [ ] Cross-platform file dialog functionality
- [ ] Memory usage with large files

## 📁 Project Structure

```
CryptIt/
├── src/                          # Vue frontend
│   ├── App.vue                   # Main application component
│   ├── main.ts                   # Application entry point
│   └── assets/                   # Static assets
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs               # Main Tauri application
│   │   ├── crypto.rs            # AES encryption module
│   │   ├── sss.rs              # Shamir Secret Sharing
│   │   └── main.rs              # Application entry point
│   ├── capabilities/
│   │   └── default.json         # Tauri permissions
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── .vscode/                      # VSCode configuration
│   ├── launch.json              # Debug configurations
│   ├── settings.json            # Workspace settings
│   └── extensions.json          # Recommended extensions
├── package.json                  # Node.js dependencies
└── README.md                     # This file
```

## 🔧 Debugging

### Frontend Debugging

```bash
# Open browser dev tools in the app
# Right-click → "Inspect Element" or press F12

# Check console for JavaScript errors
# Network tab for Tauri command failures
```

### Backend Debugging

```bash
# Run with debug logging
RUST_LOG=debug npm run tauri dev

# Run with application-specific logs
RUST_LOG=cryptit=debug npm run tauri dev

# Enable backtraces for panics
RUST_BACKTRACE=1 npm run tauri dev
```

### VSCode Debugging

1. Install recommended extensions (see `.vscode/extensions.json`)
2. Use provided debug configurations in `.vscode/launch.json`
3. Set breakpoints in both Rust and TypeScript code
4. Run "Tauri Development Debug" configuration

## 🔒 Security Considerations

### Implemented Security Measures

- **Memory Safety**: Automatic cleanup of encryption keys
- **Secure Random Generation**: CSPRNG for all cryptographic operations
- **Constant-Time Operations**: Resistant to timing attacks
- **No Key Storage**: Keys exist only in memory during operations
- **Tauri Security Model**: Granular permissions for system access

### Security Best Practices

- Always verify share authenticity before decryption
- Store shares in separate, secure locations
- Use strong physical security for share storage
- Regularly test decryption process with backup shares
- Keep the application updated for security patches

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests for any improvements.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test` and manual testing)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

---

**Built with ❤️ using Rust, Vue, and Tauri**
