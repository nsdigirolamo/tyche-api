POSTGRES_CONTAINER_NAME := tyche-postgres

db-start:
	sudo docker container start \
		$(POSTGRES_CONTAINER_NAME)

db-stop:
	sudo docker container stop \
		$(POSTGRES_CONTAINER_NAME)

db-run:
	sudo docker container run \
		--publish 5432:5432 \
		--name $(POSTGRES_CONTAINER_NAME) \
		--env POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
		--detach \
		postgres

db-rm:
	sudo docker remove \
		$(POSTGRES_CONTAINER_NAME)
