# MyTeams Project

## Overview
MyTeams is a collaborative communication system that follows a client-server architecture. It allows multiple users to communicate through teams, channels, threads, and direct messages, similar to platforms like Microsoft Teams or Slack.

## System Architecture
The project consists of three main components:
- **Server**: A central server that manages all data, handles client connections, and pushes real-time notifications.
- **Client**: A command-line interface (CLI) for users to interact with the server.
- **Logging Library**: A shared library used by both the client and the server to ensure consistent logging and printing formats.

## Features
- **Authentication**: Simple username-based login.
- **Team Organization**: Hierarchical structure: Team > Channel > Thread > Reply.
- **Direct Messaging**: Private one-on-one communication.
- **Persistence**: Server data is saved to disk and reloaded on startup.
- **Real-time Notifications**: Subscribed users receive instant updates on team events.

## Build Instructions
The project uses Cargo (Rust's build system) for all its components. To build the entire project, run the following command in the root directory:
```bash
cargo build --release
```
This will produce the `myteams_server` and `myteams_cli` binaries in the `target/release/` directory.

## Run Instructions
### Start the Server
```bash
./target/release/myteams_server <port>
```
### Start the Client
```bash
./target/release/myteams_cli <ip> <port>
```

## Documentation
Online documentation is available at: [https://akinator31.github.io/My_Teams/](https://akinator31.github.io/My_Teams/)

For more detailed information, please refer to the documentation in the `docs/` directory:
- [Client Documentation](docs/client.md)
- [Server Documentation](docs/server.md)
- [Logging Library Documentation](docs/logging_library.md)
- [Protocol Specification (RFC)](MyTeamRFC.txt)

You can also generate HTML documentation using Doxygen by running `doxygen Doxyfile` in the root directory.
