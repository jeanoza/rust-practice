## Stacks
- rust
- actix
- diesel


## Util Commands

### Diesel cli
```bash
# Setup and generate diesel.toml and migration directory
diesel setup
# Connect DB and generate schema diesel::table! including relation between tables from the DB
diesel print-schema > src/schema.rs

```

### Diesel cli ext
```bash
# generate model rust struct that can be used in rust code
diesel_ext > src/db_models.rs

```

### etc
```bash
#r2d2 to pull db connections
cargo add diesel chrono --features "diesel/postgres diesel/r2d2"


```