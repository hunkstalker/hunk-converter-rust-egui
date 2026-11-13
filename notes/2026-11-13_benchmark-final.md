# Benchmark final 1.1 — cierre de t-43

Fecha: 2026-11-13 · Estado: completado

## Metodología

- HUNK `release` con `bench_squoosh`, 3 repeticiones y mediana.
- Squoosh `@squoosh/cli 0.7.3` / `@squoosh/lib 0.4.0`, Node 16.20.2,
  3 repeticiones y mediana.
- Corpus: seis fotos Kodak (`kodim01`, `04`, `08`, `13`, `19`, `23`), captura
  de UI de HUNK e icono de 256 px con alfa.
- Calidad: SSIMULACRA2. Para AVIF se interpola el tamaño de HUNK a la misma
  puntuación de Squoosh.
- Hardware: Intel Core i7-14700K, 28 hilos, 31 GiB de RAM, Linux x86_64.

Datos completos:
[`docs/benchmark-squoosh-2026-11-13-final.tsv`](../docs/benchmark-squoosh-2026-11-13-final.tsv).

## Resultados

| Formato | Tamaño HUNK/Squoosh a igual calidad | Velocidad HUNK vs Squoosh |
| --- | --- | --- |
| JPEG q75 | **1,004** mediana (1,000–1,039) | **3,07×** mediana (2,32–3,34×) |
| JPEG q90 | **1,003** mediana (0,999–1,023) | **2,76×** mediana (2,39–3,08×) |
| WebP q75 | **1,000** | **2,84×** mediana (2,68–3,16×) |
| WebP q90 | **1,000** | **2,62×** mediana (2,24–2,89×) |
| PNG | **0,992** mediana (0,955–1,003) | **3,28×** mediana (2,82–3,74×) |
| AVIF | **0,995** mediana (0,705–1,069) | **4,64×** mediana (1,82–7,59×) |

AVIF en la captura de UI: **0,894** mediana (0,705–0,960) y **1,91×** de
velocidad mediana. Peor foto: **+5,8 %** (`kodim04`, calidad media). Peor caso
global: **+6,9 %** en el icono con alfa.

## Lectura

- WebP mantiene paridad exacta de bytes y ventaja de velocidad.
- JPEG queda en paridad práctica; la UI sigue siendo el peor caso (+3,9 %).
- PNG comprime igual o mejor y mantiene más de 3× de velocidad.
- AVIF queda en paridad práctica, con la UI claramente mejor que Squoosh.
- Los resultados confirman las conclusiones de las pasadas de `t-39` y `t-44`;
  no hay regresiones por los cambios de UI de 1.1.

## Comandos

```bash
cargo build --release --bin bench_squoosh
HUNK_BENCH_ONLY=avif target/release/bench_squoosh muestra.png salida 3
node squoosh_bench.js muestra.png salida 3
node squoosh_avif_final.js muestra.png salida 3
```

La comparación y las métricas se calcularon con `ssimulacra2_rs` y, para AVIF,
conversión previa a PNG.
