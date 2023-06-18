1. pull docker image`docker pull postgis/postgis`
2. Spin up the DB
```docker run -e POSTGRES_PASSWORD=mysecretpassword -p 5431:5432 -d postgis/postgis```
3. cargo install sqlx-cli
4. sqlx migrate run
