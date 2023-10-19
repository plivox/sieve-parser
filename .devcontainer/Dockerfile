FROM hadolint/hadolint:latest-debian AS hadolint

FROM rust:1-slim-bookworm

LABEL authors="Vincent Lauria"

LABEL maintainer="Vincent Lauria <vincent@lauria.ch>"

# The standard user with which vscode-server will be executed
ARG USERNAME=vscode

ENV USERNAME=${USERNAME}

# On Linux VS Code will automatically update the specified container
# user's UID/GID will be updated to match the local user's UID/GID to
# avoid permission problems with bind mounts
ARG USER_ID=1000 GROUP_ID=1000

ENV DEBIAN_FRONTEND=noninteractive \
    APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=DontWarn \
    HISTFILE_DIRECTORY=/commandhistory \
    WORKSPACE=/workspace

# Set rustup toolchain
ARG RUSTUP_TOOLCHAIN

RUN echo "RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}" > /tmp/.env

# ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}

# Install system dependencies for devcontainer
RUN apt-get update && apt-get install --no-install-recommends -y \
    git \
    curl \
    bash \
    sudo \
    socat \
    procps \
    openssh-client \
    apt-utils \
    bash-completion \
    ca-certificates \
    apt-utils \
    apt-transport-https \
    gnupg \
    gnupg2 \
    locales \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN echo > /etc/apt/sources.list && printf '\
    Enabled: yes\n\
    Types: deb deb-src\n\
    URIs: https://debian.ethz.ch/debian\n\
    Suites: bookworm\n\
    Components: main contrib non-free\n\
    \n\
    Enabled: yes\n\
    Types: deb deb-src\n\
    URIs: https://debian.ethz.ch/debian\n\
    Suites: bookworm-updates\n\
    Components: main contrib non-free\n\
    \n\
    Enabled: yes\n\
    Types: deb deb-src\n\
    URIs: https://security.debian.org/debian-security\n\
    Suites: bookworm-security\n\
    Components: main contrib non-free\n\
    ' | sed 's/^ *//' > /etc/apt/sources.list.d/debian.sources

# System upgrade
RUN apt-get update && apt-get upgrade --no-install-recommends -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Docker repository
RUN printf '\
    Enabled: yes\n\
    Types: deb\n\
    URIs: https://download.docker.com/linux/debian\n\
    Suites: bookworm\n\
    Components: stable\n\
    Signed-By: /usr/share/keyrings/docker-archive-keyring.gpg\n\
    ' | sed 's/^ *//' > /etc/apt/sources.list.d/docker.sources

RUN curl -fsSL https://download.docker.com/linux/debian/gpg | \
    gpg --dearmor 2>/dev/null > /usr/share/keyrings/docker-archive-keyring.gpg

# Install Docker cli
RUN apt-get update && apt-get install --no-install-recommends -y \
    docker-ce-cli \
    docker-compose-plugin \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy hadolint Dockerfile linter
COPY --from=hadolint /bin/hadolint /bin/hadolint

# Configure default locales
RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    dpkg-reconfigure --frontend=noninteractive locales && \
    update-locale LANG=en_US.UTF-8

# Create the group and the user for all operations run as inside the container
RUN groupadd --system --gid "${GROUP_ID}" "${USERNAME}" && \
    useradd --system --create-home --uid "${USER_ID}" --gid "${GROUP_ID}" \
    --groups sudo --shell /bin/bash "${USERNAME}"

# Ensure sudo group users are not asked for a password
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}

# Install system dependencies for Rust
RUN apt-get update && apt-get install --no-install-recommends -y \
    libc6-dev \
    musl-tools \
    build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust components
# RUN rustup component add rust-src clippy rustfmt --toolchain "${RUSTUP_TOOLCHAIN}"

# Copy entrypoint
COPY --chown=${USERNAME}:${USERNAME} container-entrypoint.sh /container-entrypoint

RUN chmod +x /container-entrypoint

ENTRYPOINT ["/container-entrypoint"]

COPY --chown=${USERNAME}:${USERNAME} container-entrypoint.d /container-entrypoint.d

RUN mkdir -p ${WORKSPACE} && chown -R ${USERNAME}:${USERNAME} ${WORKSPACE}

COPY --chown=${USERNAME}:${USERNAME} files/bashrc /home/${USERNAME}/.bashrc

# The next commands run with rootless user
USER ${USERNAME}

WORKDIR ${WORKSPACE}
