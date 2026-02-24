# Projekt

Productivity API built with Rust, Axum, and PostgreSQL. Features daily tasks, habits, goals, and pomodoro session tracking with JWT auth.

## Tech stack

- Rust 2024 (Axum 0.8, Tokio)
- PostgreSQL + sqlx
- JWT auth + Argon2 password hashing
- Askama templates for basic HTML views

## Local development

### 1) Start Postgres

Use Docker (recommended):

```bash
docker-compose up -d
```

This uses `user/postgres` on database `db`. If you use a local Postgres instead, make sure your `DATABASE_URL` matches.

### 2) Configure environment

Create a `.env` file in the repo root:

```env
DATABASE_URL=postgres://user:postgres@localhost:5432/db
JWT_SECRET=replace_with_a_long_random_secret
```

### 3) Run migrations

Install the sqlx CLI once:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Run the migrations:

```bash
sqlx migrate run
```

### 4) Run the server

```bash
cargo run
```

Server starts on `http://127.0.0.1:3000`.

## API overview

All routes except `/auth/register` and `/auth/login` require:

```
Authorization: Bearer <token>
```

Auth

- `POST /auth/register` `{ username, email, password }`
- `POST /auth/login` `{ email, password }` -> `{ token }`

Tasks

- `GET /tasks`
- `POST /tasks` `{ title, notes?, priority?, due_date? }`
- `PATCH /tasks/{id}` `{ title?, notes?, priority?, due_date?, completed? }`
- `DELETE /tasks/{id}`
- `POST /tasks/{id}/complete`

Goals

- `GET /goals`
- `POST /goals` `{ title, description?, deadline? }`
- `PATCH /goals/{id}` `{ title?, description?, deadline?, status? }`
- `DELETE /goals/{id}`
- `POST /goals/{id}/complete`

Habits

- `GET /habits`
- `POST /habits` `{ name, frequency? }`
- `PATCH /habits/{id}` `{ name?, frequency? }`
- `DELETE /habits/{id}`
- `POST /habits/{id}/complete` `{ completed_on? }`

Pomodoro

- `GET /pomodoro`
- `POST /pomodoro/start` `{ task_id?, session_type?, duration_minutes? }`
- `POST /pomodoro/{id}/end` `{ notes? }`
- `DELETE /pomodoro/{id}`

Views

- `GET /`
- `GET /auth/login`
- `GET /auth/register`

## Project structure

```
src/
├── main.rs              # Entry point + router setup
├── state.rs             # AppState (db pool + jwt secret)
├── models/              # DB row structs + request/response types
├── auth/                # Auth handlers, JWT, middleware
├── goals/               # Goal CRUD handlers
├── tasks/               # Task CRUD handlers
├── habits/              # Habit CRUD handlers
├── pomodoro/            # Pomodoro session tracking handlers
└── views/               # Askama templates + routes
```

## Common commands

```bash
sqlx migrate run
cargo run
cargo test
```
