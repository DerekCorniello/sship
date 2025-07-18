# 📦 sship – Secure P2P File Transfers Over SSH

**sship** is a developer-focused, zero-setup file transfer tool that uses SSH to securely send files between devices using a short pairing code or optional QR code.

No servers. No setup. Just ship it.

---

## 🔧 Purpose

To create a cross-platform, peer-to-peer file transfer tool that:

* Uses **battle-tested SSH** under the hood
* Requires **zero manual SSH config**
* Makes it **as easy to send a file as `sship send file.png`**
* Works over **LAN**
* Has reliable, production-level behaviors (resumable transfers, hashing, etc.)

---

## 🧠 Use Cases

* Transfer a large file between your **laptop and a server** with one command
* Send a file to a friend on the same **Wi-Fi network**
* Share a file securely with no need to trust the cloud
* Resume a large transfer after a network drop

---

## ✅ MVP (Minimum Viable Product)

> Goal: get a functional prototype that securely transfers files P2P over SSH using code-based pairing.

* [ ] `sship send <file>` command starts SSH listener + prints connection info
* [ ] `sship receive <code>` connects to sender and downloads file
* [ ] `sship scan` - check who is sending
* [ ] Ephemeral SSH key pair generation
* [ ] Short code pairing (e.g., `349-112`)
* [ ] SHA-256 file hashing for integrity check
* [ ] Resumable file transfer support
* [ ] Terminal progress bar
* [ ] Cross-platform support

---

## 🚀 Final (v1.0) Feature Set

> A complete, polished tool ready for real-world use.

* [ ] Automatic port fallback (e.g., use 2222 if 22 in use)
* [ ] Configurable port and timeout
* [ ] Optional passphrase for extra auth
* [ ] Detect if receiver is already connected
* [ ] Support for multiple files / folder zipping
* [ ] Public-facing README, install instructions, and example GIFs
* [ ] `cargo install` path
* [ ] Clean CLI help output (`--help` / `--version`)

---

## 🧱 Stack & Architecture

| Piece        | Tech/Crate       |
| ------------ | ---------------- |
| Language     | Rust             |
| Networking   | SSH (`openssh`)  |
| CLI Parsing  | `clap`           |
| Progress Bar | `indicatif`      |
| Hashing      | `sha2`, `digest` |

---

## 💡 Example Usage

```bash
# On sender
sship send important.pdf

# Output:
🔗 Waiting for connection...
🆔 Code: 349-112
🌐 Host: 192.168.0.24:2222

# On receiver
sship receive 349-112
```

---

## ✨ Philosophy

sship is opinionated:

* No always-running background daemon.
* No accounts, cloud, or tracking.
* No guessing what it's doing.
* No nonsense.

Just a clean CLI that speaks SSH and stays out of your way.
