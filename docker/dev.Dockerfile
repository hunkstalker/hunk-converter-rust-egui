# Entorno de desarrollo reproducible para HUNK Converter.
#
# Construcción:
#   docker build -f docker/dev.Dockerfile -t hunk-dev .
#
# Uso con el repositorio montado:
#   docker run --rm -it -v "$PWD:/workspace" -w /workspace hunk-dev
#
# Si el UID/GID del host no es 1000:
#   docker build -f docker/dev.Dockerfile \
#     --build-arg USER_ID="$(id -u)" --build-arg GROUP_ID="$(id -g)" \
#     -t hunk-dev .
FROM rust:1.99.1-bookworm

ARG USER_ID=1000
ARG GROUP_ID=1000

ENV DEBIAN_FRONTEND=noninteractive

# Dependencias de compilación y ejecución de la GUI en Linux.
# Xvfb y Mesa permiten ejecutar pruebas de interfaz sin pantalla.
RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential \
      cmake \
      nasm \
      perl \
      pkg-config \
      git \
      curl \
      ca-certificates \
      libx11-dev \
      libxcursor-dev \
      libxrandr-dev \
      libxi-dev \
      libxinerama-dev \
      libxkbcommon-dev \
      libxkbcommon-x11-dev \
      libwayland-dev \
      wayland-protocols \
      libxcb1-dev \
      libxcb-render0-dev \
      libxcb-shape0-dev \
      libxcb-xfixes0-dev \
      libgl1-mesa-dev \
      libegl1-mesa-dev \
      libgles2-mesa-dev \
      libfontconfig1-dev \
      libfreetype-dev \
      fonts-dejavu-core \
      xvfb \
      xauth \
      libgl1-mesa-dri \
      mesa-utils \
      dbus-x11 \
 && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt \
 && groupadd --gid "${GROUP_ID}" dev \
 && useradd --uid "${USER_ID}" --gid "${GROUP_ID}" \
      --create-home --shell /bin/bash dev \
 && mkdir -p /workspace /usr/local/cargo/registry /usr/local/cargo/git \
 && chown -R dev:dev /workspace /usr/local/cargo /usr/local/rustup \
 && git config --system --add safe.directory '*'

USER dev
WORKDIR /workspace

CMD ["bash"]
