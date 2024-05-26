# Kanban Board

A simple CRUD for exporing rust ecosystem.

---

## Roadmap

- [x] Setup basic error handling
- [x] Setup a health check route
- [ ] Start with a trivial CRUD
  - [x] basic structure for the project
  - [x] fake auth api and middleware
  - [ ] setup loggin/tracing
  - [ ] implement [C]reate and [R]ead for Ticket
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
- [ ] Users
- [ ] Authentication
  - [ ] JWT
  - [ ] Middleware
- [ ] CLI

## Tech Stack

- tokio
- axum
- sqlx
- cornocupia (maybe!)
- tracing

## Project Structure

- Domain
  - Entities
  - Value Objects
  - Repository Contracts
- Application
  - Board
  - List
  - Card
- Infrastructure
  - Repositories
- Presentation (Rest API)
