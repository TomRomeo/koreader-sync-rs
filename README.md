# koreader-sync-rs

A KOReader sync server implementation written in Rust.
The server is designed to be lightweight and fast, with a focus on syncing book progress. For the database, postgres is used.

## Features
- Syncs book progress.
- Passwords are hashed using bcrypt.
- Supports multiple users.
- Supports multiple devices.
- Can be deployed with Docker.
- Swagger UI for API documentation.

## How to run
### Environment Variables
- `POSTGRES_CON_STRING`: The URL of the PostgreSQL database.
- `LISTEN_ADDRESS`: The address the server will listen on. Default is `0.0.0.0:3000`.
### Using Docker
1. Build the Docker image:
   ```bash
   docker build -t koreader-sync-rs .
   ```
2. Run the Docker container:
   ```bash
    docker run -d -p 8080:8080 --name koreader-sync-rs koreader-sync-rs
    ```
3. Access the server at `http://localhost:8080`.

### Using Rust
1. Clone the repository:
   ```bash
   git clone
    ```
2. Change to the project directory:
   ```bash
   cd koreader-sync-rs
   ```
3. Build the project:
   ```bash
    cargo build --release
    ```
4. Run the server:
    ```bash
    cargo run --release
    ```