#!/bin/bash
set -ex

echo "Creating required directories..."
mkdir -p ~/.config/openbox ~/.config/tint2
mkdir -p ~/Desktop ~/Documents ~/Downloads

# Ensure OpenBox config files are in place with correct permissions
echo "Setting up OpenBox configuration..."
cp -f ~/worker_app/desktop/openbox/rc.xml ~/.config/openbox/
cp -f ~/worker_app/desktop/openbox/menu.xml ~/.config/openbox/
chmod 644 ~/.config/openbox/rc.xml ~/.config/openbox/menu.xml

echo "Starting dbus..."
sudo service dbus start || true

echo "Starting X virtual framebuffer..."
Xvfb :$DISPLAY_NUM -screen 0 ${WIDTH}x${HEIGHT}x24 &
sleep 2

echo "Setting DISPLAY variable..."
export DISPLAY=:$DISPLAY_NUM

echo "Checking if Xvfb is running..."
if ! xdpyinfo >/dev/null 2>&1; then
    echo "Xvfb failed to start"
    exit 1
fi

echo "Setting up X authority..."
xauth generate :$DISPLAY_NUM . trusted
xauth add ${HOST}:$DISPLAY_NUM . $(mcookie)

echo "Starting Openbox window manager..."
openbox --config-file ~/.config/openbox/rc.xml &
sleep 2

# Set up the desktop environment manually instead of using autostart
echo "Setting up desktop environment..."
xsetroot -solid "#2E3440"
xsetroot -cursor_name left_ptr

echo "Starting tint2 panel..."
if [ -f ~/.config/tint2/tint2rc ]; then
    echo "Using existing tint2 config"
else
    echo "Creating default tint2 config"
    mkdir -p ~/.config/tint2
    cp /home/computeruse/worker_app/desktop/tint2/tint2rc ~/.config/tint2/
fi

tint2 &
sleep 1

echo "Starting VNC server..."
x11vnc -display :$DISPLAY_NUM -forever -shared -rfbport 5901 -noxdamage -noxfixes -noxrecord -repeat -auth ~/.Xauthority -debug_xdamage -debug_x11 &
sleep 1

echo "Starting noVNC websockify..."
/opt/noVNC/utils/novnc_proxy --vnc localhost:5901 --listen 6080 &
sleep 1

# Generate system menus
echo "Generating system menus..."
sudo update-menus
mkdir -p ~/.local/share/applications
update-menus

echo "Opening a terminal..."
xterm -geometry 80x24+10+10 -fa "DejaVu Sans Mono" -fs 11 -bg "#2E3440" -fg "#D8DEE9" -title "Terminal" &

echo "Starting PCManFM file manager..."
pcmanfm --desktop &

echo "Checking X server status..."
xwininfo -root
echo "Listing windows..."
xwininfo -tree -root

echo "Checking running X clients..."
xlsclients -a

echo "Starting Rust worker application..."
cd $HOME/worker_app && cargo run &

echo "Environment setup complete. Keeping container running..."
# Keep the container running and log X errors
tail -f ~/.xsession-errors /dev/null
