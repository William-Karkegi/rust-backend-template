# RUST-BACKEND-TEMPLATE README

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Code Quality](#code-quality)
- [Database Management](#database-management)
- [Project Structure](#project-structure)
- [Modular Architecture](#modular-architecture)

## Prerequisites

Before you begin, make sure you have the following tools and dependencies installed on your system:

1. **Rust and Cargo**: Install Rust and Cargo using rustup by following the instructions
   at [https://rustup.rs/](https://rustup.rs/).

2. **Clippy**: Clippy is a linter for Rust code. Install it using the following command:

   ```
   rustup component add clippy
   ```

3. **Rustfmt**: Rustfmt is a tool for formatting Rust code. Install it using the following command:

   ```
   rustup component add rustfmt
   ```

4. **Cargo Watch**: Cargo Watch allows automatic recompilation of code when changes are detected. Install it using the
   following command:

   ```
   cargo install cargo-watch
   ```

5. **SeaORM CLI**: SeaORM CLI is used for database migrations and entity generation. Install it using the following
   command:

   ```
   cargo install sea-orm-cli
   ```

## Installation

Follow these steps to set up the project on your local machine:

1. Clone this repository:

   ```
   git clone https://github.com/William-Karkegi/rust-backend-template
   cd rust-backend-template
   ```

2. Build the project using Cargo:

   ```
   cargo build
   ```

3. Run the application:

   ```
   cargo run
   ```

4. Run the application in watch mode:
   ```
   cargo watch -q -c -w src/ -x "run"
   ```

## Code Quality

- Use `clippy` to lint the code and find potential issues:

  ```
  cargo clippy
  ```

- Use `rustfmt` to format the code:
  ```
  cargo fmt
  ```

## Database Management

- Generate a migration using SeaORM CLI:

  ```
  sea-orm-cli migrate generate <name>
  ```

- Apply pending migrations:

  ```
  sea-orm-cli migrate up --database-url 'postgres://backend:backend@localhost:5454/backend'
  ```

- Generate entity files using SeaORM CLI:
  ```
  sea-orm-cli generate entity -o entity/src -l --with-serde both --database-url 'postgres://backend:backend@localhost:5454/backend'
  ```

## Project Structure

The project is structured as follows:

- **docker**: Contains the docker-compose file for running the project in a containerized environment.

- **entity**: Contains the entity files generated by SeaORM CLI.

- **migrations**: Contains the migration files generated by SeaORM CLI.

- **src**: Contains the source code for the project.

  - **modules**: Contains the modules for the project, each of which is a subdirectory containing the following
    folders:

    - **application**: Contains the application logic for the module.

    - **domain**: Contains the domain logic for the module.

    - **web**: Contains the web API for the module.

## Modular Architecture

The project follows a modular architecture, with each module containing its own application, domain, and web layers.
This allows for better separation of concerns and makes the code more maintainable and testable.

- **Application Layer**: The application layer contains the business logic for the module. It is responsible for
  orchestrating the domain logic and the web API.

- **Domain Layer**: The domain layer contains the domain logic for the module. It is responsible for implementing the
  business logic.

- **Web Layer**: The web layer contains the web API for the module. It is responsible for exposing the business logic to
  the outside world.

For example, when a user makes a request to the web API, the web layer will call the application layer, which will in
turn call the domain layer to perform the necessary operations. The domain layer will then return the result to the
application layer, which will return it to the web layer, which will return it to the user.

A more detailed explanation of the modular architecture can be found in the following article:

- [https://softwareontheroad.com/ideal-nodejs-project-structure/](https://softwareontheroad.com/ideal-nodejs-project-structure/)