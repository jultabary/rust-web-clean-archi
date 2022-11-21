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

To compile project:
```
make release
```
<br/>Warning if you use docker instead of podman, update line 7 of Makefile with:
```
DOCKER := podman -> DOCKER := docker
```
<br/>To start rust server with its database:<br/>
```
make start-db
make start-releaase
```

### API
- POST http://localhost:8000/foo -> body : { "name": string, "number_oy_years": int }
  - API to instantiate a new Foo Object from body
- GET http://localhost:8000/foo
  - Return all existing Foo instances
- PUT http://localhost:8000/foo/{id}
  - increment by one the `number_of_years` of Foo
