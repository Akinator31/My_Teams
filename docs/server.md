# MyTeams Server Documentation

## Overview
The MyTeams server is the central hub for the collaborative communication system. It handles all data management, including users, teams, channels, threads, and messages, and push updates to connected clients.

## Key Responsibilities
- **Client Management**: Handle multiple simultaneous connections using a non-blocking, event-driven architecture.
- **Data Persistence**: Store and reload all system data (users, teams, messages, etc.) from disk.
- **Request Handling**: Parse and execute client commands according to the MYTP (MyTeams Protocol).
- **Event Push**: Notify connected and subscribed users when relevant events occur (new messages, team creation, etc.).
- **Access Control**: Enforce rules for team subscriptions and user authentication.

## Usage
To start the server, use the following command:
```bash
./myteams_server <port>
```
- `<port>`: The port number the server will listen on.

## Protocol: MYTP/1.0
The server communicates with clients using the MyTeams Protocol (MYTP/1.0), which is a line-based, text protocol.
- **Messages**: CRLF-terminated lines.
- **Codes**: Uses numeric reply codes (e.g., 200 OK, 404 Not Found, etc.).
- **Quoted Strings**: All string arguments in commands must be enclosed in double quotes.

## Data Model
- **User**: Identified by a UUID and username.
- **Team**: Organizational unit containing channels.
- **Channel**: A subset of a team containing threads.
- **Thread**: A specific discussion containing replies.
- **Comment/Reply**: A single post in a thread.
- **Direct Message**: Private communication between two users.

## Persistence
On shutdown (SIGINT), the server persists all current state to files. Upon startup, it reloads the state if save files are present, ensuring no data loss between sessions.
