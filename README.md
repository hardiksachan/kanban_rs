# Kanban Board

A simple CRUD for exploring rust ecosystem.

---

## Roadmap

- [x] Setup basic error handling
- [x] Setup a health check route
- [x] Start with a trivial CRUD
  - [x] basic structure for the project
  - [x] fake auth api and middleware
  - [x] implement [C]reate, [R]ead All, and [D]elete for Ticket
  - [x] protect /tickets routes using auth middleware
  - [x] setup logging
  - [x] reorganise and modularize a bit more
- [x] Setup tracing
- [x] Refactor to hexagonal architecture
- [x] Refactor into workspaces
  - [x] Extract `ctx` crate
  - [x] Extract `trace` crate
  - [x] Extract domain logic into `tickets` crate
  - [x] Extract `rest_api` crate responsible to configre rest api structure
  - [x] Refactor `tickets` crate with hexagonal architecture in mind
- [ ] Implement real auth
  - [ ] Users
  - [ ] Authentication
- [ ] Host the app on render.com
- [ ] Model Domain
  - [ ] Board
  - [ ] List
  - [ ] Card
- [ ] Model Repository Contract
- [ ] In memory Repository
  - [ ] Board
  - [ ] List
  - [ ] Card
- [ ] Usecases
  - [ ] CRUD Board
  - [ ] CRUD Lists
  - [ ] CRUD Cards
- [ ] REST API
  - [ ] CRUD Board
  - [ ] CRUD Lists
  - [ ] CRUD Cards
- [ ] CLI

## Tech Stack

- tokio
- axum
- sqlx
- cornocupia (maybe!)
- tracing
