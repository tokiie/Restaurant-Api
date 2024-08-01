# Restaurant-Api

## Introduction

This project is a Rust-based web application that uses Axum as its web framework. Axum is designed for building fast and reliable web services with a focus on simplicity and safety.

## Features

- Web Framework: The application uses Axum to handle HTTP requests and responses. Axum provides a modular and composable way to define routes, middlewares, and request handlers.
- Database Integration: The application interacts with a PostgreSQL database, using sqlx for asynchronous SQL queries and migrations.
- Docker Support: The project includes a Docker Compose setup for easy development and testing.

## Prerequisites

Before setting up the database, ensure you have the following:

- Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
- Docker and Docker Compose installed. You can download and install them from [docker.com](https://www.docker.com/products/docker-desktop).

## Setting Up the Database with Docker Compose

1. **Clone the Repository**

   Clone the project repository to your local machine.

   ```bash
   git clone https://github.com/tokiie/Restaurant-Api.git
   cd Restaurant-Api
   ```

2. **Start the Docker Compose Environment**

   Start the PostgreSQL database and any other necessary services using Docker Compose.

   ```bash
   docker-compose up -d
   ```

   This command will start the database in the background.

3. **Configure Environment Variables**

   Check the `.env` file in the project root and set your desired environment variables. These will be used by the application to connect to the database, you can also choose the host and port for the app.

   ```bash
   DATABASE_URL=postgres://username:password@localhost/your_database_name
   HOST=localhost
   PORT=3000
   ```

   Replace `username`, `password`, and `your_database_name` with the credentials specified in the `docker-compose.yml` file.

4. **Install `sqlx-cli`**

   Install the `sqlx-cli` tool, which is necessary for running migrations.

   ```bash
   cargo install sqlx-cli --no-default-features --features rustls,postgres
   ```

5. **Run Migrations**

   With the database running inside Docker and `sqlx-cli` installed, run the migrations to set up the database schema:

   ```bash
   sqlx migrate run
   ```

   Ensure the `DATABASE_URL` environment variable is set correctly to connect to the Dockerized PostgreSQL instance.

6. **Verify Setup**

   Verify that the database schema has been created correctly by inspecting the database or running a simple query.

## Running the Application

With the database set up, you can now run the application:

```bash
cargo run
```

## Running Tests

To run the tests, make sure the test database is set up and configured in Docker, the 5433 port is exposed. Typically, you'll have a separate test database URL:

```bash
TEST_DATABASE_URL=postgres://username:password@localhost:5433/your_test_database_name
```

Run the tests using:

```bash
cargo test
```

Ensure the test database is clean before running tests to avoid conflicts. Some functions are already provided to clean the database.

## Stopping the Docker Containers

To stop the Docker containers, run:

```bash
docker-compose down
```

This command will stop and remove the containers.

## Additional Information

- **Documentation**: For more detailed documentation on `sqlx`, refer to the [official documentation](https://docs.rs/sqlx).

## Troubleshooting

- If you encounter issues with migrations, ensure that the `DATABASE_URL` is correctly configured and that the PostgreSQL container is running.

## License

Include the appropriate license information for your project.
