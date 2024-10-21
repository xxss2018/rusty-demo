docker run -d \
	--name metahumanpg \
	-e POSTGRES_PASSWORD=mysecretpassword \
	-e PGDATA=/var/lib/postgresql/data/pgdata \
    -p 5432:5432 \
    -e POSTGRES_USER=postgres \
	--restart always \
	postgres