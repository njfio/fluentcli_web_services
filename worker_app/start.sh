#!/bin/bash

# Start Xvfb
Xvfb :99 -screen 0 ${RESOLUTION:-1024x768x24} &
sleep 1

# Start x11vnc
x11vnc -display :99 -forever -nopw &

# Export display for GUI applications
export DISPLAY=:99

# Set logging level if not already set
export RUST_LOG=${RUST_LOG:-debug}

# Start the worker app
./worker_app
