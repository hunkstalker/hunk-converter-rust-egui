# CI y releases

## Workflow único

`.github/workflows/ci.yml` concentra todo:

- **Pull requests:** solo formato, clippy y pruebas.
- **Push a `main`:** solo formato, clippy y pruebas.
- **Tags de versión:** pruebas, builds, checksums y GitHub Release.
- **Ejecución manual:** pruebas y builds, sin release.

## Plataformas

| Job | Runner | Artefacto |
| --- | --- | --- |
| Linux Debian x86_64 | `ubuntu-22.04` | `hunk-converter-<versión>-linux-x86_64.tar.gz` |
| Windows 10/11 x86_64 | `windows-latest` | `hunk-converter-<versión>-windows-x86_64.zip` |
| macOS universal | `macos-14` | `hunk-converter-<versión>-macos-universal.tar.gz` |

La build Linux se compila en Ubuntu 22.04 y requiere glibc 2.35 o superior
(Debian 12+, Ubuntu 22.04+). La build de macOS contiene un binario universal
`arm64 + x86_64`; no está firmada ni notarizada. Windows se compila con MSVC y
se distribuye como carpeta portable comprimida. La entrega Linux incluye un lanzador `.desktop` y el icono PNG; la entrega macOS es un `.app` con `icon.icns`.

## Caché

Los jobs de pruebas y de build usan `Swatinem/rust-cache@v2`. La clave se
calcula por job, sistema operativo, versión de Rust y `Cargo.lock`. Se cachean el
registro de Cargo y `target/`, incluidos los objetos generados por las
dependencias C/C++ y los builds de cada target instalado.

Se evaluó `sccache` para las dependencias C/C++ y no se añadió:

- `rust-cache` ya reutiliza `target/` completo cuando la clave coincide.
- Para persistir entre ejecuciones, `sccache` necesitaría un backend remoto o el
  backend de GitHub Actions, con configuración y permisos adicionales.
- NASM, usado por el decodificador AVIF, no pasa por `sccache`.
- Se puede reconsiderar si los tiempos siguen siendo altos tras medir el efecto
  de `rust-cache`.

## Release

El workflow se dispara con tags que encajen con `*.*.*` o `v*.*.*`. Para publicar:

```bash
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

El job `release` solo se ejecuta cuando el evento es un tag. Descarga los tres
artefactos, genera `SHA256SUMS.txt` y crea la GitHub Release con las builds y las
notas automáticas.

También se puede lanzar manualmente desde **Actions → CI y release → Run
workflow**. En ese caso se generan los artefactos, pero no se publica ninguna
release porque no hay tag.

## Requisitos de build

- Rust 1.99.1.
- NASM para el decodificador AVIF.
- `lipo` en macOS (incluido en Xcode Command Line Tools).
- PowerShell y `Compress-Archive` en Windows para el `.zip` portable.
- `lipo` y los targets universales de Rust en macOS.
- `winresource` incrusta `icon.ico` en el `.exe` de Windows.

El script local equivalente es:

```bash
./scripts/build_dist.sh linux
./scripts/build_dist.sh windows
./scripts/build_dist.sh macos
```

## Estado de la release 1.0.0

La release [`v1.0.0`](https://github.com/hunkstalker/hunk-converter-rust-egui/releases/tag/v1.0.0)
se publicó correctamente y se validó en Windows 10/11, incluido el icono del
ejecutable.

`t-34` deja los builds limitados a tags y ejecución manual, con `rust-cache` en
pruebas y builds. `sccache` se evaluó y se descartó por ahora. La próxima
ejecución manual o el tag de 1.1.0 validarán el workflow completo.
