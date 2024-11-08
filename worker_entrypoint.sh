#!/bin/bash
set -ex

echo "Creating required directories..."
mkdir -p ~/.config/lxpanel/default/panels
mkdir -p ~/Desktop ~/Documents ~/Downloads
mkdir -p ~/.xsession-errors.log  # Create xsession errors file

# Create and set permissions for attachments directory
echo "Setting up attachments directory..."
sudo mkdir -p /app/attachments/screenshots
sudo chown -R computeruse:computeruse /app/attachments
sudo chmod -R 777 /app/attachments  # Ensure both worker and backend can write

# Configure lxpanel
cat > ~/.config/lxpanel/default/panels/panel << EOF
# lxpanel <profile> config file
Global {
    edge=top
    allign=left
    margin=0
    widthtype=percent
    width=100
    height=26
    transparent=0
    tintcolor=#000000
    alpha=0
    autohide=0
    heightwhenhidden=2
    setdocktype=1
    setpartialstrut=1
    usefontcolor=0
    fontsize=10
    fontcolor=#ffffff
    usefontsize=0
    background=0
    backgroundfile=/usr/share/lxpanel/images/background.png
    iconsize=24
}

Plugin {
    type = menu
    Config {
        image=/usr/share/lxde/images/lxde-icon.png
        system {
        }
        separator {
        }
        item {
            name=Run
            image=system-run
            command=run
        }
        separator {
        }
        item {
            name=Shutdown
            image=system-shutdown
            command=logout
        }
    }
}

Plugin {
    type = taskbar
    expand=1
    Config {
        tooltips=1
        IconsOnly=0
        ShowAllDesks=0
        UseMouseWheel=1
        UseUrgencyHint=1
        FlatButton=0
        MaxTaskWidth=150
        spacing=1
        GroupedTasks=0
    }
}

Plugin {
    type = tray
}
EOF

# Set up D-Bus
echo "Setting up D-Bus..."
if [ ! -d /run/user/$(id -u) ]; then
    sudo mkdir -p /run/user/$(id -u)
    sudo chmod 700 /run/user/$(id -u)
    sudo chown $(id -un):$(id -gn) /run/user/$(id -u)
fi

echo "Starting D-Bus..."
if [ ! -e /run/dbus/pid ]; then
    sudo mkdir -p /run/dbus
    sudo dbus-daemon --system
fi

# Start the session bus
dbus-daemon --session --address=unix:path=/run/user/$(id -u)/bus --nofork --print-address &
sleep 2

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

# Set up the desktop environment manually instead of using autostart
echo "Setting up desktop environment..."
xsetroot -solid "#2E3440"
xsetroot -cursor_name left_ptr

echo "Starting Metacity window manager..."
metacity --display=:$DISPLAY_NUM --no-composite --no-force-fullscreen &
sleep 2

echo "Starting lxpanel..."
lxpanel --profile default &
sleep 2

echo "Starting VNC server..."
x11vnc -display :$DISPLAY_NUM -forever -shared -rfbport 5901 -noxdamage -noxfixes -noxrecord -repeat -auth ~/.Xauthority &
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

# Create some test files
echo "Creating test files..."
mkdir -p ~/Documents/test_folder
echo "Hello World" > ~/Documents/test.txt
echo "Test file 2" > ~/Documents/test2.txt
echo "File 1 content" > ~/Documents/test_folder/file1.txt
echo "File 2 content" > ~/Documents/test_folder/file2.txt

# Create some example files in different formats
echo "Creating example files..."
echo '{"key": "value"}' > ~/Documents/example.json
echo '# Markdown Example' > ~/Documents/example.md
echo '<html><body>Hello</body></html>' > ~/Documents/example.html
echo 'console.log("Hello");' > ~/Documents/example.js

# Create some directories with content
echo "Creating directories with content..."
mkdir -p ~/Documents/projects
mkdir -p ~/Documents/images
mkdir -p ~/Documents/code

# Create some more test files in subdirectories
echo "Creating files in subdirectories..."
echo "Project README" > ~/Documents/projects/README.md
echo "Sample code" > ~/Documents/code/sample.py
touch ~/Documents/images/placeholder.png

# Set permissions
echo "Setting permissions..."
sudo chown -R computeruse:computeruse ~
chmod -R 755 ~/Desktop ~/Documents ~/Downloads ~/.local
chmod 700 ~/.config

echo "Listing created files for verification..."
ls -la ~/Documents

echo "Starting Rust worker application..."
cd ~/worker_app && RUST_LOG=debug ./target/release/worker_app &

# Wait for the worker application to start and become healthy
echo "Waiting for worker application to become healthy..."
for i in {1..30}; do
    if curl -s http://localhost:8081/computer-use/health > /dev/null; then
        echo "Worker application is healthy"
        break
    fi
    echo "Waiting for worker application to start... ($i/30)"
    sleep 1
done

echo "Environment setup complete. Keeping container running..."
# Keep the container running and log X errors
touch ~/.xsession-errors
exec tail -f ~/.xsession-errors
