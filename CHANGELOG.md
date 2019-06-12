### Until 0.2.2

A dark time where code was in heavy influx and black metal ruled the world

### 0.3.0

- Start keeping track of the changelog
- Add login endpoint
- Add token-based authentication (plus trimming)
- Black metal still rules the world

### 0.4.0

- Update table in production: `users.is_active` defaults to false
- Add roles: now users can be "admin" or "user"
- Changes in response for signup/login
- Changes in response for user detail
- New `/user/:id` endpoint to modify own user profile (currently only email field is allowed)
- Role enforcement on API routes: `role=user` not allowed anymore to these endpoints:
  - `/users` (list users)
  - `/admin` (admin backoffice)
  - `/doors` (list doors)
- Admin backoffice (https://door.cwrkng.de/admin) to enable/disable users (only accessible to users with role "admin")

### 0.4.1

Service micro release
