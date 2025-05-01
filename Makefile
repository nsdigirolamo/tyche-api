DATABASE_NAME := tyche-postgres

db-create:
	sudo docker container run \
		--publish 5432:5432 \
		--name $(DATABASE_NAME) \
		--env POSTGRES_PASSWORD=${DATABASE_PASSWORD} \
		--detach \
		postgres

db-remove:
	sudo docker remove \
		$(DATABASE_NAME)

db-start:
	sudo docker container start \
		$(DATABASE_NAME)

db-stop:
	sudo docker container stop \
		$(DATABASE_NAME)

db-migrate-run:
	cargo sqlx migrate run

db-migrate-revert:
	cargo sqlx migrate revert

db-enter:
	sudo docker exec -it tyche-postgres psql -U postgres

docker-build:
	cargo sqlx prepare && \
	sudo docker build -t nsdigirolamo/tyche:api . && \
	cd migrations && sudo docker build -t nsdigirolamo/tyche:db .

docker-push:
	sudo docker push nsdigirolamo/tyche:api && \
	sudo docker push nsdigirolamo/tyche:db
