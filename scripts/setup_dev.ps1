param(
    [switch]$Install,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Uso: .\scripts\setup_dev.ps1 [-Install]"
    Write-Host ""
    Write-Host "Sin -Install solo comprueba el sistema y muestra lo que falta."
    Write-Host "Con -Install instala Rust, CMake, NASM y las Visual Studio Build Tools."
    exit 0
}

function Test-Command($Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Install-WingetPackage($Id) {
    Write-Host "+ winget install --id $Id -e"
    if ($Install) {
        & winget install --id $Id -e --accept-source-agreements --accept-package-agreements
        if ($LASTEXITCODE -ne 0) {
            throw "winget no pudo instalar $Id"
        }
    }
}

function Test-MsvcTools {
    if (Test-Command "cl") {
        return $true
    }
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vswhere) {
        $path = & $vswhere -latest -products * `
            -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 `
            -property installationPath
        return [bool]$path
    }
    return $false
}

Write-Host "HUNK Converter - preparacion del entorno"
Write-Host ""

$winget = Test-Command "winget"
if (-not $winget) {
    Write-Host "winget no esta disponible. Instala App Installer desde Microsoft Store." -ForegroundColor Yellow
}

if (Test-Command "rustup") {
    Write-Host "Rustup encontrado."
    if ($Install) {
        & rustup toolchain install 1.99.1 --profile minimal --component clippy --component rustfmt
    } else {
        Write-Host "+ rustup toolchain install 1.99.1 --profile minimal --component clippy --component rustfmt"
    }
} elseif (Test-Command "cargo") {
    Write-Host "cargo esta instalado, pero no se encontro rustup." -ForegroundColor Yellow
} else {
    Write-Host "Rust no encontrado."
    if ($winget) {
        Install-WingetPackage "Rustlang.Rustup"
        Write-Host "Cierra y abre la terminal despues de instalar Rust para actualizar PATH." -ForegroundColor Yellow
    } else {
        Write-Host "Instala rustup desde https://rustup.rs"
    }
}

if (-not (Test-Command "cmake")) {
    Write-Host "CMake no encontrado."
    if ($winget) { Install-WingetPackage "Kitware.CMake" }
} else {
    Write-Host "CMake encontrado."
}

if (-not (Test-Command "nasm")) {
    Write-Host "NASM no encontrado."
    if ($winget) { Install-WingetPackage "NASM.NASM" }
} else {
    Write-Host "NASM encontrado."
}

if (-not (Test-MsvcTools)) {
    Write-Host "Visual Studio Build Tools (C++) no encontrados."
    if ($winget -and $Install) {
        Write-Host "+ winget install --id Microsoft.VisualStudio.2022.BuildTools ..."
        & winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
            --accept-source-agreements --accept-package-agreements `
            --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
        if ($LASTEXITCODE -ne 0) {
            Write-Host "No se pudieron instalar las Build Tools automaticamente." -ForegroundColor Yellow
            Write-Host "Instalalas desde https://visualstudio.microsoft.com/visual-cpp-build-tools/ con el workload 'Desktop development with C++'."
        }
    } else {
        Write-Host "Instalalas desde https://visualstudio.microsoft.com/visual-cpp-build-tools/ con el workload 'Desktop development with C++'."
    }
} else {
    Write-Host "Visual Studio C++ encontrado."
}

Write-Host ""
if ($Install) {
    Write-Host "Entorno preparado. Abre una terminal nueva si se instalo Rust o las Build Tools."
} else {
    Write-Host "Modo comprobacion. Ejecuta de nuevo con -Install para aplicar los cambios."
}
Write-Host ""
Write-Host "Siguiente:"
Write-Host "  cargo test"
Write-Host "  cargo fmt --all -- --check"
Write-Host "  cargo clippy --all-targets -- -D warnings"
Write-Host "  cargo run --release"
