
FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

# Install Desktop-ENV, tools
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install \
	tigervnc-standalone-server \
	tigervnc-xorg-extension \
	dbus \
	dbus-x11 \
	gnome-keyring \
	xfce4 \
	xfce4-terminal \
	xdg-utils \
	x11-xserver-utils \
	g++ \
	pkg-config \
	libx11-dev \
	libasound2-dev \
	libudev-dev \
	python3-numpy


ENV DEBIAN_FRONTEND=dialog

# Install novnc and numpy module for it
RUN git clone --depth 1 https://github.com/novnc/noVNC.git /opt/novnc \
	&& git clone --depth 1 https://github.com/novnc/websockify /opt/novnc/utils/websockify \
	&& find /opt/novnc -type d -name '.git' -exec rm -rf '{}' +


COPY novnc-index.html /opt/novnc/index.html
COPY gp-vncsession /usr/bin/

# Add VNC startup script
RUN echo "\nexport DISPLAY=:0\n" >> /home/vscode/.bashrc

RUN chmod 0755 /usr/bin/gp-vncsession
