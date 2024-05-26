# Kanban Board

A simple CRUD for exporing rust ecosystem.

---

## Roadmap

- [ ] Setup basic error handling
- [ ] Setup a health check route
- [ ] Host the app on render.com
- [ ] Add Instrumentation
- [ ] Model Domain
  - [ ] Board
  - [ ] List
  - [ ] Card
  - [ ] Repository Contract
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
