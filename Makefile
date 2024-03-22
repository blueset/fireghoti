ifneq (dev,$(wildcard config.env))
	include ./dev/config.env
	export
endif


.PHONY: db.init db.up db.down
db.init:
	$(MAKE) -C ./dev/db-container init
db.up:
	$(MAKE) -C ./dev/db-container up
db.down:
	$(MAKE) -C ./dev/db-container down

.PHONY: entities
entities:
	pnpm run migrate
	$(MAKE) -C ./packages/backend-rs regenerate-entities
