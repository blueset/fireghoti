.PHONY: regenerate-entities
regenerate-entities:
	sea-orm-cli generate entity \
		--output-dir='src/model/entity' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:25432/$(POSTGRES_DB)' \
		--date-time-crate='chrono' \
	  --model-extra-attributes='NAPI_EXTRA_ATTR_PLACEHOLDER' && \
	for file in src/model/entity/*; do \
	  base=$$(basename -- "$${file}"); \
	  jsname=$$(printf '%s\n' "$${base%.*}" | perl -pe 's/(^|_)./uc($$&)/ge;s/_//g'); \
	  attribute=$$(printf 'cfg_attr(feature = "napi", napi_derive::napi(object, js_name = "%s", use_nullable = true))' "$${jsname}"); \
	  sed -i "s/NAPI_EXTRA_ATTR_PLACEHOLDER/$${attribute}/" "$${file}"; \
	done
	sed -i 's/#\[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)\]/#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]\n#[cfg_attr(not(feature = "napi"), derive(Clone))]\n#[cfg_attr(feature = "napi", napi_derive::napi)]/' \
	  src/model/entity/sea_orm_active_enums.rs

index.js:
	NODE_OPTIONS='--max_old_space_size=3072' pnpm run build:debug
	[ -f built/index.js ]
	rm --force index.js
	cp built/index.js index.js
