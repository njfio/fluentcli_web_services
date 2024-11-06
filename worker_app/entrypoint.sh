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
chmod -R 755 ~/Desktop ~/Documents ~/Downloads
chmod 700 ~/.config

echo "Listing created files for verification..."
ls -la ~/Documents

echo "Starting Rust worker application..."
cd ~/worker_app && RUST_LOG=debug cargo run &

echo "Environment setup complete. Keeping container running..."
# Keep the container running and log X errors
tail -f ~/.xsession-errors /dev/null
