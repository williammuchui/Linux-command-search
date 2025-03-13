#! /bin/bash

echo "Building the project..."

cargo build --release 

sudo cp target/release/commands /usr/local/bin/

mkdir -p ~/.commands

sudo cp linux ~/.commands/

echo "Executable installed at '/usr/local/bin'"
echo "Installation complete!"
