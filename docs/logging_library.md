# Logging Library Documentation

## Overview
The MyTeams logging library provides standardized logging functions for both the client and the server. It is built as a shared library (`.so`) with Rust bindings.

## Role
The library's primary role is to provide a unified way to log events and print system state. It ensures consistency between different components and facilitates debugging and monitoring.

## Features
- **Client Logging**: Log user login/logout, message reception, and item creation (teams, channels, threads, replies).
- **Server Logging**: Log team creation, channel/thread creation, user subscription, and persistence events.
- **Unified Printing**: Provide standardized output formats for users, teams, channels, threads, and messages.

## Usage
The library is automatically linked by the client and server binaries. In the Rust code, you can call the `ClientLog` functions to perform logging operations.

### Key Functions
- `client_event_logged_in(user_uuid, user_name)`: Logs a user login.
- `client_print_users(user_uuid, user_name, user_status)`: Prints details about a user.
- `server_event_team_created(team_uuid, team_name, user_uuid)`: Logs the creation of a new team by the server.

## Integration
The library is implemented in the `libs/` directory. It uses `bindgen` (implicit or explicit) to create bindings for the C headers `logging_client.h` and `logging_server.h`.
The shared library files (`libmyteams.so`) must be present in the linker's path for the project to run.
