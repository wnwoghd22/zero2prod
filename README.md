# zero2prod

by Jay (wnwoghd22)

## How to Use

I didn't deploy server on digital ocean. Instead, I just connected db with server through docker network.

In `init_db_local.sh`,

```sh

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}:5432" \
        -d --name postgres \
        --network newsletter \
        postgres -N 1000
fi
```

The container name is forced to be `postgres` and has access to network `newsletter`. So, before execute `init` you have to command,

```sh
$ docker network create newsletter
```

