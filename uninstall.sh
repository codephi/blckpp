#!/bin/bash

# Service name
service_name="blckpp"

# Stop and disable the service
sudo systemctl stop $service_name
sudo systemctl disable $service_name

# Remove the service file
sudo rm /etc/systemd/system/$service_name.service

# Reload systemd to reflect the changes
sudo systemctl daemon-reload

# Remove the application binary from /usr/bin
sudo rm /usr/bin/blckpp

# Remove the extracted directory if it exists
extract_dir="blckpp-extracted"
if [ -d "$extract_dir" ]; then
    rm -r "$extract_dir"
fi

echo "Uninstallation completed."
