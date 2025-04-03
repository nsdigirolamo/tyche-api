DATABASE_NAME := tyche-postgres

db-start:
	sudo docker container start \
		$(DATABASE_NAME)

db-stop:
	sudo docker container stop \
		$(DATABASE_NAME)

db-run:
	sudo docker container run \
		--publish 5432:5432 \
		--name $(DATABASE_NAME) \
		--env POSTGRES_PASSWORD=${DATABASE_PASSWORD} \
		--detach \
		postgres

db-rm:
	sudo docker remove \
		$(DATABASE_NAME)

db-migrate-up:
	cargo sqlx migrate run

db-migrate-down:
	cargo sqlx migrate revert

db-enter:
 	sudo docker exec -it tyche-postgres psql -U postgres
