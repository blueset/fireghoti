.PHONY: regenerate-entities
regenerate-entities:
	sea-orm-cli generate entity \
		--output-dir='src/model/entity' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:25432/$(POSTGRES_DB)'

index.js:
	NODE_OPTIONS='--max_old_space_size=3072' pnpm run build:debug
	[ -f built/index.js ]
	rm --force index.js
	cp built/index.js index.js
