#!/bin/bash

# Enable error handling and debugging
set -e
set -x

echo "Starting worker application setup..."

# Ensure /repo directory exists with correct permissions
if [ ! -d /repo ]; then
    echo "Creating /repo directory..."
    sudo mkdir -p /repo
fi

echo "Setting permissions for /repo..."
sudo chown -R worker:worker /repo
sudo chmod -R 755 /repo

echo "Setting up logging directory..."
if [ ! -d /home/worker/logs ]; then
    mkdir -p /home/worker/logs
fi
chmod 755 /home/worker/logs

# Use environment variables for display configuration
DISPLAY_WIDTH=${DISPLAY_WIDTH:-1024}
DISPLAY_HEIGHT=${DISPLAY_HEIGHT:-768}
DISPLAY_NUMBER=${DISPLAY_NUMBER:-99}
DISPLAY_DEPTH=24

# Clean up any existing X lock files
rm -f /tmp/.X${DISPLAY_NUMBER}-lock
rm -f /tmp/.X11-unix/X${DISPLAY_NUMBER}

echo "Starting Xvfb with display ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}..."
Xvfb :${DISPLAY_NUMBER} -screen 0 ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}x${DISPLAY_DEPTH} -ac +extension GLX +render -noreset &
XVFB_PID=$!

# Wait for Xvfb to be ready with more detailed error reporting
max_attempts=30
attempt=0
while ! xdpyinfo -display :${DISPLAY_NUMBER} >/dev/null 2>&1; do
    attempt=$((attempt + 1))
    if [ $attempt -ge $max_attempts ]; then
        echo "Failed to start Xvfb after $max_attempts attempts"
        echo "Checking Xvfb process status:"
        ps -p $XVFB_PID || true
        echo "Checking X11 socket:"
        ls -la /tmp/.X11-unix/ || true
        echo "Checking X lock files:"
        ls -la /tmp/.X* || true
        echo "Checking system logs:"
        tail -n 50 /var/log/syslog 2>/dev/null || true
        if ps -p $XVFB_PID > /dev/null; then
            kill $XVFB_PID
        fi
        exit 1
    fi
    echo "Waiting for Xvfb to start... (attempt $attempt/$max_attempts)"
    sleep 1
done

# Verify display dimensions match exactly
actual_dimensions=$(xdpyinfo -display :${DISPLAY_NUMBER} | grep dimensions | awk '{print $2}')
expected_dimensions="${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}"
if [ "$actual_dimensions" != "$expected_dimensions" ]; then
    echo "Error: Display dimensions mismatch"
    echo "Expected: $expected_dimensions"
    echo "Actual: $actual_dimensions"
    echo "Full xdpyinfo output:"
    xdpyinfo -display :${DISPLAY_NUMBER}
    if ps -p $XVFB_PID > /dev/null; then
        kill $XVFB_PID
    fi
    exit 1
fi

echo "Xvfb started successfully with resolution ${DISPLAY_WIDTH}x${DISPLAY_HEIGHT}"

# Try to kill any existing x11vnc processes
pkill x11vnc || true
sleep 1

echo "Starting x11vnc..."
x11vnc -display :${DISPLAY_NUMBER} -forever -nopw -rfbport 5901 -shared &
X11VNC_PID=$!

# Wait for x11vnc to start and verify it's running
max_attempts=10
attempt=0
while ! ps -p $X11VNC_PID > /dev/null 2>&1; do
    attempt=$((attempt + 1))
    if [ $attempt -ge $max_attempts ]; then
        echo "Failed to start x11vnc after $max_attempts attempts"
        echo "Checking x11vnc process:"
        ps aux | grep x11vnc || true
        echo "Checking VNC port:"
        netstat -tuln | grep 5901 || true
        if ps -p $XVFB_PID > /dev/null; then
            kill $XVFB_PID
        fi
        exit 1
    fi
    echo "Waiting for x11vnc to start... (attempt $attempt/$max_attempts)"
    sleep 1
done

echo "x11vnc started successfully"

# Verify xdotool is available and working
if ! command -v xdotool > /dev/null; then
    echo "Error: xdotool is not installed"
    if ps -p $XVFB_PID > /dev/null; then
        kill $XVFB_PID
    fi
    if ps -p $X11VNC_PID > /dev/null; then
        kill $X11VNC_PID
    fi
    exit 1
fi

# Test xdotool with exact coordinates
echo "Testing xdotool with display bounds..."
if ! xdotool mousemove 0 0; then
    echo "Error: Failed to move mouse to origin (0,0)"
    echo "xdotool error output:"
    DISPLAY=:${DISPLAY_NUMBER} xdotool mousemove 0 0 2>&1 || true
    if ps -p $XVFB_PID > /dev/null; then
        kill $XVFB_PID
    fi
    if ps -p $X11VNC_PID > /dev/null; then
        kill $X11VNC_PID
    fi
    exit 1
fi

if ! xdotool mousemove $((DISPLAY_WIDTH-1)) $((DISPLAY_HEIGHT-1)); then
    echo "Error: Failed to move mouse to bottom-right corner (${DISPLAY_WIDTH}-1, ${DISPLAY_HEIGHT}-1)"
    echo "xdotool error output:"
    DISPLAY=:${DISPLAY_NUMBER} xdotool mousemove $((DISPLAY_WIDTH-1)) $((DISPLAY_HEIGHT-1)) 2>&1 || true
    if ps -p $XVFB_PID > /dev/null; then
        kill $XVFB_PID
    fi
    if ps -p $X11VNC_PID > /dev/null; then
        kill $X11VNC_PID
    fi
    exit 1
fi

echo "xdotool verified successfully with display bounds"

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