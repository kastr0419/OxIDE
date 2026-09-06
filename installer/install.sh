#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# ALLoIDE Linux Installer
# Usage: curl -sSf https://raw.githubusercontent.com/kastr0419/ALLoIDE/master/installer/install.sh | bash
# Or:    bash install.sh [--prefix /usr/local] [--no-tools] [--no-rust]

set -euo pipefail

REPO="kastr0419/ALLoIDE"
VERSION="latest"
INSTALL_PREFIX="${INSTALL_PREFIX:-}"
NO_TOOLS=0
NO_RUST=0

# ── Parse args ────────────────────────────────────────────────────────────────
for arg in "$@"; do
  case "$arg" in
    --prefix=*) INSTALL_PREFIX="${arg#*=}" ;;
    --no-tools) NO_TOOLS=1 ;;
    --no-rust)  NO_RUST=1 ;;
    --version=*) VERSION="${arg#*=}" ;;
    -h|--help)
      echo "Usage: install.sh [--prefix=DIR] [--no-tools] [--no-rust] [--version=vX.Y.Z]"
      exit 0 ;;
  esac
done

# ── Colors ────────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'
info()    { echo -e "${CYAN}[ALLoIDE]${NC} $*"; }
success() { echo -e "${GREEN}[ALLoIDE]${NC} ✅ $*"; }
warn()    { echo -e "${YELLOW}[ALLoIDE]${NC} ⚠️  $*"; }
die()     { echo -e "${RED}[ALLoIDE]${NC} ❌ $*"; exit 1; }

# ── Detect OS / arch ──────────────────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"
[[ "$OS" != "Linux" ]] && die "This installer only supports Linux. Use installer/build_installer.ps1 on Windows."
[[ "$ARCH" != "x86_64" ]] && die "Only x86_64 is currently supported (detected: $ARCH)."

# ── Determine install prefix ──────────────────────────────────────────────────
if [[ -z "$INSTALL_PREFIX" ]]; then
  if [[ $EUID -eq 0 ]]; then
    INSTALL_PREFIX="/usr/local"
  else
    INSTALL_PREFIX="$HOME/.local"
  fi
fi
BIN_DIR="$INSTALL_PREFIX/bin"
SHARE_DIR="$INSTALL_PREFIX/share"

info "Installing ALLoIDE to $BIN_DIR"
mkdir -p "$BIN_DIR"

# ── Detect package manager ────────────────────────────────────────────────────
detect_pm() {
  for pm in apt-get dnf pacman zypper; do
    command -v "$pm" &>/dev/null && echo "$pm" && return
  done
  echo "unknown"
}
PM="$(detect_pm)"

# ── Install system deps ───────────────────────────────────────────────────────
install_deps() {
  info "Installing system dependencies..."
  case "$PM" in
    apt-get)
      sudo apt-get update -qq
      sudo apt-get install -y --no-install-recommends \
        libgtk-3-0 libxcb-render0 libxcb-shape0 libxcb-xfixes0 \
        libxkbcommon0 libssl3 libudev1 curl
      ;;
    dnf)
      sudo dnf install -y gtk3 libxcb libxkbcommon openssl libudev curl
      ;;
    pacman)
      sudo pacman -Sy --noconfirm gtk3 libxcb libxkbcommon openssl systemd-libs curl
      ;;
    zypper)
      sudo zypper install -y libgtk-3-0 libxcb1 libxkbcommon0 libopenssl3 libudev1 curl
      ;;
    *)
      warn "Unknown package manager. Make sure GTK3, libxcb, and libxkbcommon are installed."
      ;;
  esac
}

# ── Install avrdude ────────────────────────────────────────────────────────────
install_avrdude() {
  if command -v avrdude &>/dev/null; then
    success "avrdude already installed ($(avrdude -v 2>&1 | head -1))"
    return
  fi
  info "Installing avrdude..."
  case "$PM" in
    apt-get) sudo apt-get install -y avrdude ;;
    dnf)     sudo dnf install -y avrdude ;;
    pacman)  sudo pacman -Sy --noconfirm avrdude ;;
    zypper)  sudo zypper install -y avrdude ;;
    *)       warn "Please install avrdude manually for Arduino support." ;;
  esac
}

# ── Install Rust via rustup ───────────────────────────────────────────────────
install_rust() {
  if command -v rustup &>/dev/null; then
    success "Rust already installed ($(rustc --version))"
  else
    info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env" 2>/dev/null || export PATH="$HOME/.cargo/bin:$PATH"
  fi

  info "Adding embedded targets..."
  rustup target add thumbv7em-none-eabihf 2>/dev/null || true
  rustup target add riscv32imc-unknown-none-elf 2>/dev/null || true
  success "Rust targets installed"
}

# ── Download ALLoIDE binary ─────────────────────────────────────────────────────
download_alloide() {
  if [[ "$VERSION" == "latest" ]]; then
    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/alloide-linux-x86_64.tar.gz"
  else
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/alloide-linux-x86_64.tar.gz"
  fi

  info "Downloading ALLoIDE from $DOWNLOAD_URL ..."
  TMPDIR="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR"' EXIT

  curl -sSfL "$DOWNLOAD_URL" -o "$TMPDIR/alloide.tar.gz" || \
    die "Failed to download. Check your internet connection or visit: https://github.com/${REPO}/releases"

  tar -xzf "$TMPDIR/alloide.tar.gz" -C "$TMPDIR"

  # Find the alloide binary in extracted files
  ALLOIDE_BIN="$(find "$TMPDIR" -name "alloide" -type f | head -1)"
  [[ -z "$ALLOIDE_BIN" ]] && die "alloide binary not found in archive."

  chmod +x "$ALLOIDE_BIN"
  cp "$ALLOIDE_BIN" "$BIN_DIR/alloide"
  success "ALLoIDE installed to $BIN_DIR/alloide"
}

# ── Create .desktop entry ─────────────────────────────────────────────────────
create_desktop_entry() {
  APPS_DIR="$SHARE_DIR/applications"
  mkdir -p "$APPS_DIR"
  cat > "$APPS_DIR/alloide.desktop" <<EOF
[Desktop Entry]
Version=1.0
Name=ALLoIDE
Comment=Rust Embedded Development IDE
Exec=$BIN_DIR/alloide
Icon=alloide
Terminal=false
Type=Application
Categories=Development;IDE;
Keywords=rust;embedded;arduino;esp32;stm32;
StartupNotify=true
EOF
  success "Desktop entry created at $APPS_DIR/alloide.desktop"

  # Update desktop database if available
  command -v update-desktop-database &>/dev/null && \
    update-desktop-database "$APPS_DIR" 2>/dev/null || true
}

# ── Update PATH hint ──────────────────────────────────────────────────────────
check_path() {
  if ! echo "$PATH" | grep -q "$BIN_DIR"; then
    warn "$BIN_DIR is not in your PATH."
    echo ""
    echo "  Add this to your ~/.bashrc or ~/.zshrc:"
    echo -e "  ${CYAN}export PATH=\"\$PATH:$BIN_DIR\"${NC}"
    echo ""
  fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}╔════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     ALLoIDE Linux Installer        ║${NC}"
echo -e "${CYAN}║  Rust Embedded Development IDE     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════╝${NC}"
echo ""

install_deps
download_alloide
create_desktop_entry

[[ $NO_TOOLS -eq 0 ]] && install_avrdude
[[ $NO_RUST -eq 0 ]]  && install_rust

check_path

echo ""
success "Installation complete! Run: alloide"
echo ""
echo -e "  📖 Docs:    ${CYAN}https://github.com/${REPO}${NC}"
echo -e "  🐛 Issues:  ${CYAN}https://github.com/${REPO}/issues${NC}"
echo ""
