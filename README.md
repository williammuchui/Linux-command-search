# Commands

```
██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
```

Commands is a simple command-line tool that lists and searches available Linux
commands directly in your terminal. It's designed to help users quickly discover
and explore the commands they can use in their Linux environment.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Uninstall](Uninstall)
- [Usage](#usage)
- [Customization](#customization)
- [Contributing](#contributing)
- [License](#license)

## Features

- Displays a comprehensive list of all available Linux commands
- Simple and intuitive command-line interface
- Supports searching for specific commands
- Gradual display of commands for better readability

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Building from Source

```
git clone https://github.com/juanmilkah/commands.git
cd commands
bash build.sh
```

## Uninstallation
May require privilege escalation

```bash
rm -rf /usr/local/bin/commands 
rm -rf ~/.commands
```

## Usage

To use the Commands tool:

1. List all available commands:

```bash
commands
```

2. Search for a specific command:

```bash
commands search <search_term>
```

3. Ignorecase

```bash
commands search <search_term> -i
```

4. Display help information:

```bash
commands --help
```

5. Show the Version

```bash
commands --version
```

### Example Output

```bash
Available Linux Commands:
1. ls - List directory contents
2. cd - Change directory
3. cp - Copy files and directories
...
```

## Customization

You can easily add or modify commands by editing the `linux` file in the `~/.commands` directory. Each line should follow this format:

```
[command_name]  Command description
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, please follow these steps:

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
