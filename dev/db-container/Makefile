.PHONY: init up down
init: down up
up:
	$(COMPOSE) up --detach
down:
	$(COMPOSE) down

.PHONY: psql redis-cli
psql:
	$(COMPOSE) exec db psql --user=$(POSTGRES_USER) --dbname=$(POSTGRES_DB)
redis-cli:
	$(COMPOSE) exec redis redis-cli
