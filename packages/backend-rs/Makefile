.PHONY: regenerate-entities
regenerate-entities:
	sea-orm-cli generate entity \
		--output-dir='src/model/entity' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:25432/$(POSTGRES_DB)'
