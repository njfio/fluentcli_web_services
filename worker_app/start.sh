#!/bin/bash
set -ex

echo "Starting worker application setup..."

# Ensure directories exist with correct permissions
echo "Setting up directories..."
sudo mkdir -p /repo /app/attachments/screenshots /home/worker/logs
sudo chown -R worker:worker /repo /app/attachments/screenshots /home/worker/logs
sudo chmod -R 755 /repo /app/attachments/screenshots /home/worker/logs

# Create desktop structure
mkdir -p ~/Desktop ~/Documents ~/Downloads
touch ~/Desktop/Terminal.desktop
echo "[Desktop Entry]
Type=Application
Name=Terminal
Exec=xterm
Icon=terminal
Categories=System;TerminalEmulator;" > ~/Desktop/Terminal.desktop
chmod +x ~/Desktop/Terminal.desktop

# Use environment variables for display configuration
DISPLAY_WIDTH=${DISPLAY_WIDTH:-1024}
DISPLAY_HEIGHT=${DISPLAY_HEIGHT:-768}
DISPLAY_NUMBER=${DISPLAY_NUMBER:-99}
DISPLAY_DEPTH=24

# Clean up any existing X lock files
rm -f /tmp/.X${DISPLAY_NUMBER}-lock
rm -f /tmp/.X11-unix/X${DISPLAY_NUMBER}

echo "Starting Xvfb with display ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}..."
Xvfb :${DISPLAY_NUMBER} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x${DISPLAY_DEPTH} -ac +extension GLX +extension RANDR +extension XINERAMA +extension RENDER +extension COMPOSITE -noreset &
XVFB_PID=$!

# Wait for Xvfb to be ready
max_attempts=30
attempt=0
while ! xdpyinfo -display :${DISPLAY_NUMBER} >/dev/null 2>&1; do
    attempt=$((attempt + 1))
    if [ $attempt -ge $max_attempts ]; then
        echo "Failed to start Xvfb after $max_attempts attempts"
        exit 1
    fi
    echo "Waiting for Xvfb to start... (attempt $attempt/$max_attempts)"
    sleep 1
done

echo "Setting initial resolution..."
xrandr --display :${DISPLAY_NUMBER} --fb ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}

# Set background color
xsetroot -solid "#2E3440"

echo "Starting window manager..."
openbox --config-file /etc/xdg/openbox/rc.xml --startup /etc/xdg/openbox/autostart &
sleep 2

echo "Starting tint2 panel..."
tint2 -c /etc/xdg/tint2/tint2rc &
sleep 1

echo "Starting PCManFM file manager..."
pcmanfm --desktop --profile computer-use &
sleep 1

echo "Starting x11vnc..."
x11vnc -display :${DISPLAY_NUMBER} \
    -forever \
    -shared \
    -rfbport 5901 \
    -nopw \
    -repeat \
    -noxdamage \
    -noxrecord \
    -noxfixes \
    -desktop "Computer Use" \
    -env "FD_PROG=xrandr -s %wx%h" \
    -clear_keys \
    -clear_mods \
    -clear_all \
    -nobell \
    -nowf \
    -noscr \
    -threads \
    -xkb \
    -ncache 0 \
    -cursor arrow \
    -viewonly &
X11VNC_PID=$!

# Wait for x11vnc to start
sleep 2

echo "Opening a terminal..."
xterm -geometry 80x24+10+10 \
    -fa "DejaVu Sans Mono" \
    -fs 11 \
    -bg "#2E3440" \
    -fg "#D8DEE9" \
    -title "Terminal" &

# Export display variables for worker app
export DISPLAY=:${DISPLAY_NUMBER}
export DISPLAY_WIDTH
export DISPLAY_HEIGHT
export DISPLAY_NUMBER

# Set logging level if not already set
export RUST_LOG=${RUST_LOG:-debug}

echo "Starting worker app..."
# Start the worker app with output logging
exec ./worker_app 2>&1 | tee -a /home/worker/logs/worker.log
