use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use hunk_converter::codec::{self, EncodeOptions, OutputFormat};

struct Case {
    label: &'static str,
    format: OutputFormat,
    quality: u8,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(input) = args.next().map(PathBuf::from) else {
        eprintln!("Uso: bench_squoosh <entrada> <directorio-salida> [repeticiones]");
        std::process::exit(2);
    };
    let Some(output_dir) = args.next().map(PathBuf::from) else {
        eprintln!("Uso: bench_squoosh <entrada> <directorio-salida> [repeticiones]");
        std::process::exit(2);
    };
    let runs = args
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(3)
        .max(1);

    let loaded = match codec::load_image_file(&input) {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("Error cargando {}: {error}", input.display());
            std::process::exit(1);
        }
    };
    if let Err(error) = std::fs::create_dir_all(&output_dir) {
        eprintln!("Error creando {}: {error}", output_dir.display());
        std::process::exit(1);
    }

    let stem = input
        .file_stem()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "imagen".to_owned());
    let cases = [
        Case {
            label: "jpg-q75",
            format: OutputFormat::Jpg,
            quality: 75,
        },
        Case {
            label: "jpg-q90",
            format: OutputFormat::Jpg,
            quality: 90,
        },
        Case {
            label: "webp-q75",
            format: OutputFormat::Webp,
            quality: 75,
        },
        Case {
            label: "webp-q90",
            format: OutputFormat::Webp,
            quality: 90,
        },
        Case {
            label: "avif-q40",
            format: OutputFormat::Avif,
            quality: 40,
        },
        Case {
            label: "avif-q50",
            format: OutputFormat::Avif,
            quality: 50,
        },
        Case {
            label: "avif-q60",
            format: OutputFormat::Avif,
            quality: 60,
        },
        Case {
            label: "avif-q70",
            format: OutputFormat::Avif,
            quality: 70,
        },
        Case {
            label: "avif-q80",
            format: OutputFormat::Avif,
            quality: 80,
        },
        Case {
            label: "avif-q90",
            format: OutputFormat::Avif,
            quality: 90,
        },
        Case {
            label: "png",
            format: OutputFormat::Png,
            quality: 0,
        },
    ];
    let only = std::env::var("HUNK_BENCH_ONLY").ok();

    for case in cases {
        if let Some(prefix) = only.as_deref()
            && !case.label.starts_with(prefix)
        {
            continue;
        }
        let options = EncodeOptions {
            format: case.format,
            quality: case.quality,
            ico_sizes: Vec::new(),
            background: [255, 255, 255, 255],
        };
        let mut bytes_len = 0;
        let mut timings = Vec::with_capacity(runs);
        let mut output_path = None;
        for _ in 0..runs {
            let started = Instant::now();
            match codec::encode(&loaded.pixels, &options) {
                Ok(output) => {
                    timings.push(started.elapsed());
                    bytes_len = output.bytes.len();
                    let path = output_dir.join(format!(
                        "{stem}-hunk-{}.{}",
                        case.label,
                        case.format.extension()
                    ));
                    if let Err(error) = std::fs::write(&path, &output.bytes) {
                        eprintln!("Error escribiendo {}: {error}", path.display());
                        std::process::exit(1);
                    }
                    output_path = Some(path);
                }
                Err(error) => {
                    eprintln!(
                        "Error codificando {} {}: {error}",
                        input.display(),
                        case.label
                    );
                    std::process::exit(1);
                }
            }
        }
        timings.sort_unstable();
        let median = median(&timings);
        let path = output_path.unwrap_or_else(|| Path::new("").to_path_buf());
        println!(
            "hunk\t{}\t{}\t{}\t{:.3}\t{}",
            input.file_name().unwrap_or_default().to_string_lossy(),
            case.label,
            bytes_len,
            median.as_secs_f64() * 1000.0,
            path.display()
        );
    }
}

fn median(values: &[Duration]) -> Duration {
    if values.len().is_multiple_of(2) {
        let a = values[values.len() / 2 - 1];
        let b = values[values.len() / 2];
        (a + b) / 2
    } else {
        values[values.len() / 2]
    }
}
