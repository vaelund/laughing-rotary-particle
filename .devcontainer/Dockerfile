
FROM mcr.microsoft.com/devcontainers/universal:linux

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get -y install lldb rustup

    && rustup update 2>&1 \
    && rustup component add rls rust-analysis rust-src rustfmt clippy 2>&1 \

    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
    && chmod 0440 /etc/sudoers.d/$USERNAME \

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog