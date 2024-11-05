#!/bin/bash
set -ex

echo "Starting worker application setup..."

# Ensure directories exist with correct permissions
echo "Setting up directories..."
mkdir -p /repo /app/attachments/screenshots /home/worker/logs

# Create desktop structure
mkdir -p ~/Desktop ~/Documents ~/Downloads
touch ~/Desktop/Terminal.desktop
echo "[Desktop Entry]
Type=Application
Name=Terminal
Exec=gnome-terminal
Icon=terminal
Categories=System;TerminalEmulator;" > ~/Desktop/Terminal.desktop
chmod +x ~/Desktop/Terminal.desktop

# Use environment variables for display configuration
DISPLAY_WIDTH=${DISPLAY_WIDTH:-1280}
DISPLAY_HEIGHT=${DISPLAY_HEIGHT:-1024}
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

# Start D-Bus
echo "Starting D-Bus..."
sudo service dbus start

# Set background color
xsetroot -solid "#2E3440"

# Set up GNOME Session environment
export XDG_SESSION_TYPE=x11
export XDG_SESSION_CLASS=user
export XDG_SESSION_DESKTOP=gnome-classic
export XDG_CURRENT_DESKTOP=GNOME-Classic:GNOME
export GNOME_SHELL_SESSION_MODE=classic
export DESKTOP_SESSION=gnome-classic

# Start GNOME Session
echo "Starting GNOME Session..."
dbus-launch --exit-with-session gnome-session --session=gnome-classic &
sleep 5

echo "Starting window manager..."
mutter --replace --sm-disable --display=:${DISPLAY_NUMBER} --wayland --no-x11 &
sleep 2

echo "Starting GNOME Panel..."
gnome-panel &
sleep 2

echo "Starting PCManFM file manager..."
pcmanfm --desktop --profile computer-use &
sleep 1

echo "Starting x11vnc..."
x11vnc -display :${DISPLAY_NUMBER} \
    -forever \
    -shared \
    -rfbport 5901 \
    -nopw \
    -xkb \
    -noxrecord \
    -noxfixes \
    -noxdamage \
    -wait 5 \
    -desktop "Computer Use" \
    -cursor arrow &
X11VNC_PID=$!

# Wait for x11vnc to start and verify it's running
sleep 2
if ! ps -p $X11VNC_PID > /dev/null; then
    echo "x11vnc failed to start or crashed"
    exit 1
fi

# Start noVNC
websockify --web=/usr/share/novnc/ 6080 localhost:5901 &
NOVNC_PID=$!

# Wait for websockify to start and verify it's running
sleep 2
if ! ps -p $NOVNC_PID > /dev/null; then
    echo "websockify failed to start or crashed"
    exit 1
fi

echo "Opening a terminal..."
gnome-terminal &

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
