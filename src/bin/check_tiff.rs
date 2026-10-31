use std::path::{Path, PathBuf};

use hunk_converter::codec;
use hunk_converter::domain::LoadError;

struct Expectation {
    file: &'static str,
    dimensions: (u32, u32),
    alpha: bool,
}

fn main() {
    let directory = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist/pruebas-tiff"));
    let expectations = [
        Expectation {
            file: "foto-rgb-sin-compresion.tiff",
            dimensions: (1024, 683),
            alpha: false,
        },
        Expectation {
            file: "foto-rgb-lzw.tiff",
            dimensions: (1024, 683),
            alpha: false,
        },
        Expectation {
            file: "foto-rgb-deflate.tiff",
            dimensions: (1024, 683),
            alpha: false,
        },
        Expectation {
            file: "foto-rgb-packbits.tiff",
            dimensions: (1024, 683),
            alpha: false,
        },
        Expectation {
            file: "gris-packbits.tiff",
            dimensions: (640, 360),
            alpha: false,
        },
        Expectation {
            file: "icono-rgba-deflate.tiff",
            dimensions: (256, 256),
            alpha: true,
        },
        Expectation {
            file: "multipagina.tiff",
            dimensions: (512, 341),
            alpha: false,
        },
        Expectation {
            file: "orientacion-6.tiff",
            dimensions: (720, 480),
            alpha: false,
        },
    ];

    let mut failures = 0;
    for expectation in expectations {
        let path = directory.join(expectation.file);
        match check(&path, expectation.dimensions, expectation.alpha) {
            Ok(message) => println!("PASS  {:<34} {message}", expectation.file),
            Err(message) => {
                failures += 1;
                println!("FAIL  {:<34} {message}", expectation.file);
            }
        }
    }

    let rejected = directory.join("rgba-16bits.tiff");
    match codec::load_image_file(&rejected) {
        Err(LoadError::UnsupportedTiff) => {
            println!(
                "PASS  {:<34} rechazo controlado de 16 bits",
                "rgba-16bits.tiff"
            );
        }
        Ok(_) => {
            failures += 1;
            println!(
                "FAIL  {:<34} se esperaba rechazo de 16 bits",
                "rgba-16bits.tiff"
            );
        }
        Err(error) => {
            failures += 1;
            println!("FAIL  {:<34} error inesperado: {error}", "rgba-16bits.tiff");
        }
    }

    if failures > 0 {
        eprintln!("{failures} comprobaciones fallidas");
        std::process::exit(1);
    }
    println!("Pack TIFF validado");
}

fn check(path: &Path, dimensions: (u32, u32), expect_alpha: bool) -> Result<String, String> {
    let loaded = codec::load_image_file(path).map_err(|error| error.to_string())?;
    if loaded.format != "TIFF" {
        return Err(format!("formato inesperado: {}", loaded.format));
    }
    let actual = loaded.pixels.dimensions();
    if actual != dimensions {
        return Err(format!("dimensiones {actual:?}, esperadas {dimensions:?}"));
    }
    let has_alpha = loaded.pixels.pixels().any(|pixel| pixel.0[3] < 255);
    if expect_alpha && !has_alpha {
        return Err("se esperaba alfa y todos los píxeles son opacos".to_owned());
    }
    Ok(format!(
        "{}×{} · {} · {}",
        actual.0,
        actual.1,
        if has_alpha { "alfa" } else { "opaco" },
        hunk_converter::codec::format_bytes(loaded.file_bytes)
    ))
}
