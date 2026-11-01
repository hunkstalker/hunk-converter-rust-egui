#!/usr/bin/env bash
# Prepara un entorno local de desarrollo en Linux o macOS.
#
# Sin argumentos muestra los comandos que faltan. Con --install los ejecuta.
# El script no instala Homebrew ni Xcode Command Line Tools automáticamente:
# solo muestra cómo hacerlo si no están presentes.

set -euo pipefail

INSTALL=0

usage() {
	cat <<'TXT'
Uso: ./scripts/setup_dev.sh [--install]

Sin --install solo comprueba el sistema y muestra los comandos necesarios.
Con --install instala las dependencias del sistema y Rust 1.99.1.
TXT
}

for argument in "$@"; do
	case "$argument" in
		--install|-i) INSTALL=1 ;;
		--help|-h)
			usage
			exit 0
			;;
		*)
			echo "Argumento no reconocido: $argument" >&2
			usage >&2
			exit 2
			;;
	esac
done

log() {
	printf '  %s\n' "$*"
}

run() {
	printf '+ %s\n' "$*"
	if [[ "$INSTALL" -eq 1 ]]; then
		"$@"
	fi
}

as_root() {
	if [[ "$(id -u)" -eq 0 ]]; then
		run "$@"
	else
		run sudo "$@"
	fi
}

ensure_rust() {
	export PATH="$HOME/.cargo/bin:$PATH"
	if command -v rustup >/dev/null 2>&1; then
		run rustup toolchain install 1.99.1 --profile minimal \
			--component clippy --component rustfmt
	elif command -v cargo >/dev/null 2>&1; then
		log "cargo está instalado, pero no se encontró rustup; se usará el toolchain disponible."
	else
		log "Rust no encontrado. Instala rustup y el toolchain 1.99.1:"
		log "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.99.1 --profile minimal"
		if [[ "$INSTALL" -eq 1 ]]; then
			run sh -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.99.1 --profile minimal"
			export PATH="$HOME/.cargo/bin:$PATH"
			run rustup component add clippy rustfmt
		fi
	fi
}

install_debian() {
	as_root apt-get update
	as_root apt-get install -y --no-install-recommends \
		build-essential cmake nasm pkg-config git curl ca-certificates \
		libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxinerama-dev \
		libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev wayland-protocols \
		libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
		libgl1-mesa-dev libegl1-mesa-dev libgles2-mesa-dev \
		libfontconfig1-dev libfreetype-dev fonts-dejavu-core \
		xvfb xauth libgl1-mesa-dri mesa-utils dbus-x11
}

install_fedora() {
	as_root dnf install -y \
		gcc gcc-c++ make cmake nasm pkgconf-pkg-config git curl \
		libX11-devel libXcursor-devel libXrandr-devel libXi-devel libXinerama-devel \
		libxkbcommon-devel libxkbcommon-x11-devel wayland-devel wayland-protocols-devel \
		mesa-libGL-devel mesa-libEGL-devel fontconfig-devel freetype-devel dejavu-sans-fonts \
		xorg-x11-server-Xvfb xauth mesa-dri-drivers
}

install_arch() {
	as_root pacman -S --needed --noconfirm \
		base-devel cmake nasm pkgconf git curl \
		libx11 libxcursor libxrandr libxi libxinerama \
		libxkbcommon libxkbcommon-x11 wayland wayland-protocols mesa \
		fontconfig freetype2 ttf-dejavu xorg-server-xvfb xauth
}

install_macos() {
	if ! xcode-select -p >/dev/null 2>&1; then
		log "Xcode Command Line Tools no encontrados. Ejecuta:"
		log "xcode-select --install"
	fi
	if ! command -v brew >/dev/null 2>&1; then
		log "Homebrew no encontrado. Instálalo desde https://brew.sh y vuelve a ejecutar el script."
		return
	fi
	run brew install cmake nasm pkg-config
}

echo "HUNK Converter · preparación del entorno"
echo

case "$(uname -s)" in
	Darwin)
		echo "Sistema: macOS"
		install_macos
		;;
	Linux)
		if [[ ! -r /etc/os-release ]]; then
			echo "No se pudo detectar la distribución de Linux." >&2
			exit 2
		fi
		. /etc/os-release
		case "${ID:-} ${ID_LIKE:-}" in
			*debian*|*ubuntu*|*linuxmint*|*pop*)
				echo "Sistema: ${PRETTY_NAME:-Debian/Ubuntu}"
				install_debian
				;;
			*fedora*)
				echo "Sistema: ${PRETTY_NAME:-Fedora}"
				install_fedora
				;;
			*arch*)
				echo "Sistema: ${PRETTY_NAME:-Arch}"
				install_arch
				;;
			*)
				echo "Distribución no reconocida: ${PRETTY_NAME:-desconocida}" >&2
				echo "Instala manualmente: compilador de C, CMake, NASM, pkg-config y librerías X11/GL/Wayland." >&2
				exit 2
				;;
		esac
		;;
	*)
		echo "Sistema no soportado por este script: $(uname -s)" >&2
		echo "En Windows usa: .\\scripts\\setup_dev.ps1" >&2
		exit 2
		;;
esac

echo
ensure_rust

echo
if [[ "$INSTALL" -eq 0 ]]; then
	echo "Modo comprobación. Ejecuta de nuevo con --install para aplicar los comandos."
else
	echo "Entorno preparado. Abre una terminal nueva si Rust se acaba de instalar."
fi
echo
echo "Siguiente:"
echo "  cargo test"
echo "  cargo fmt --all -- --check"
echo "  cargo clippy --all-targets -- -D warnings"
echo "  cargo run --release"
