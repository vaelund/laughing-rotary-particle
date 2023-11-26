
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# Install Desktop-ENV, tools
RUN install-packages \
	tigervnc-standalone-server tigervnc-xorg-extension \
	dbus dbus-x11 gnome-keyring xfce4 xfce4-terminal \
	xdg-utils x11-xserver-utils pip

# Install novnc and numpy module for it
RUN git clone --depth 1 https://github.com/novnc/noVNC.git /opt/novnc \
	&& git clone --depth 1 https://github.com/novnc/websockify /opt/novnc/utils/websockify \
	&& find /opt/novnc -type d -name '.git' -exec rm -rf '{}' + \
	&& sudo -H pip3 install numpy
COPY .devcontainer/novnc-index.html /opt/novnc/index.html
COPY .devcontainer/gp-vncsession /usr/bin/

# Add VNC startup script
RUN echo -e "\nexport DISPLAY=:0\n" >> /home/vscode/.bashrc

RUN chmod 0755 "$(which gp-vncsession)"