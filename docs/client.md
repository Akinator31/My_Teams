# MyTeams Client Documentation

## Overview
The MyTeams client is a command-line interface (CLI) that allows users to interact with the MyTeams server. It provides a way to manage teams, channels, threads, and direct messages.

## Features
- **Authentication**: Connect to the server using a simple username.
- **Team Management**: Create, list, and subscribe to teams.
- **Channel & Thread Management**: Create and list channels within teams and threads within channels.
- **Messaging**: Send and receive direct messages between users.
- **Real-time Notifications**: Receive updates on new teams, channels, threads, and messages.

## Usage
To start the client, use the following command:
```bash
./myteams_cli <ip> <port>
```
- `<ip>`: The IP address of the server.
- `<port>`: The port number the server is listening on.

## Commands
The client supports various commands, most of which are contextual:
- `/help`: Display the list of available commands.
- `/login ["username"]`: Log in to the server.
- `/logout`: Log out from the server.
- `/users`: List all registered users.
- `/user ["user_uuid"]`: Get details about a specific user.
- `/send ["user_uuid"] ["message_body"]`: Send a direct message.
- `/messages ["user_uuid"]`: List direct messages with a user.
- `/subscribe ["team_uuid"]`: Subscribe to a team.
- `/subscribed ["team_uuid"]`: List users subscribed to a team or teams you are subscribed to.
- `/unsubscribe ["team_uuid"]`: Unsubscribe from a team.
- `/use ["team_uuid"] ["channel_uuid"] ["thread_uuid"]`: Set the command context.
- `/create`: Create a team, channel, thread, or reply based on the current context.
- `/list`: List items based on the current context.
- `/info`: Get information about the current context or user.

## Implementation Details
The client is built using Rust and uses a non-blocking I/O approach to handle server messages and user input simultaneously. It relies on a transport layer for communication and a command manager to parse and execute user commands.
