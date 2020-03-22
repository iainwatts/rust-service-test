# Rocket test app

Basic dockerized Rust web service template with Rocket, Diesel, and postgres

```
docker-compose build
docker-compose run
```

```
docker build -t rocket-tests .
docker run -it --rm -p 8000:8000 --name tests-running rocket-tests
```

```
curl \
--header "Content-Type: application/json" \
--request POST \
--data '{"name": "Fido"}' \
http://localhost:8000/

curl http://localhost:8000/
```

```
docker exec -ti <container_id> psql -h db -d rocket-test -U test-user
```

```
cargo install diesel_cli --no-default-features --features postgres
diesel setup --database-url postgres://test-user:test-pw@localhost:5432/rocket-test

diesel migration run --database-url postgres://test-user:test-pw@localhost:5432/rocket-test
```
