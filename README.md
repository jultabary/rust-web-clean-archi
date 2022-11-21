# Rust Clean Architecture Proposal

## Libraries used
- <a href="https://rocket.rs">rocket</a> for exposing HTTP API
- <a href="https://github.com/launchbadge/sqlx">SQLx</a> to query DB (not an ORM)

## Requirements
- Rust be installed <a href="https://www.rust-lang.org/tools/install">here</a>
- Podman or Docker to use Makefile
- Makefile

## Use project

### Launch project
```
make release
make start-db
make start-releaase
```

### API
- POST http://localhost:8000/foo -> body : { "name": string, "number_oy_years": int }
- GET http://localhost:8000/foo
- PUT http://localhost:8000/foo/{id}
