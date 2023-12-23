DB_CRED = cti_user:example
DB_NAME = cti_dev_diesel

DATABASE_HOST ?= localhost
export DATABASE_URL ?= postgres://$(DB_CRED)@$(DATABASE_HOST)/$(DB_NAME)
export DATABASE_ADMIN_URL ?= postgres://$(DB_CRED)@$(DATABASE_HOST)/$(DB_NAME)

export FLY_REGION ?= unset

RUSTFLAGS ?=
ifneq ($(RUSTFLAGS),)
	BUILD_ARGS = --build-arg RUSTFLAGS="$(RUSTFLAGS)"
endif

DOCKER_RUN_ARGS = \
	--rm -ti \
	-e DATABASE_URL="$(DATABASE_URL)" \
	-e DATABASE_ADMIN_URL="$(DATABASE_ADMIN_URL)" \
	-e HOST_IP=0.0.0.0 \
	-e FLY_REGION=$(FLY_REGION) \
	-p 3000:3000

TAG ?= latest
STAMP ?= $(shell buildstamp minute) # 23W42.12345

ASSETS_DIR = cti_assets/assets
FRONT_DIR = frontend


JS_FILES = \
	$(ASSETS_DIR)/cti.js

CSS_FILES = \
	$(ASSETS_DIR)/cti.css

HTML_FILES = \
	$(ASSETS_DIR)/index.html \
	$(ASSETS_DIR)/users.html

TIDY_SETTINGS = -q -utf8 \
	--newline LF \
	--wrap 0 --vertical-space auto \
	--drop-empty-elements no \
	--tidy-mark no

default:
	@echo nope

# fly.io

deploy:
	fly deploy

# fly scale count 1 --region fra
# fly scale count 1 --region iad
# fly scale count 1 --region lax
scale:
	fly scale count 3 --max-per-region 1 --region fra,iad,lax

# docker image

# docker build -t cti_server:build --target builder .
image: assets
	docker build $(BUILD_ARGS) -t cti_server:$(STAMP) .
	-docker rmi cti_server:$(TAG)
	docker tag cti_server:$(STAMP) cti_server:$(TAG)
	docker images cti_server

publish: assets image
	docker tag cti_server:$(TAG) registry.fly.io/cti:$(STAMP)
	docker push registry.fly.io/cti:$(STAMP)

# local

debug: assets
	cargo adev
	cargo cdev
	cargo serve

release: assets
	cargo assets
	cargo core
	cargo sr

# frontend code / asset pipeline

# NEEDS:
# tidy - https://binaries.html-tidy.org/
assets: $(ASSETS_DIR) js.files css.files html.files
	cp $(FRONT_DIR)/favicon.* $(ASSETS_DIR)/

$(ASSETS_DIR):
	rm -rf $@
	mkdir -p $@

js.files: $(ASSETS_DIR) $(JS_FILES)

$(JS_FILES): $(ASSETS_DIR)/%: $(FRONT_DIR)/%
	npx uglify-js $< -c -m reserved="['updateTops']" -o $@

css.files: $(ASSETS_DIR) $(CSS_FILES)

$(CSS_FILES): $(ASSETS_DIR)/%: $(FRONT_DIR)/%
	npx csso-cli $< -o $@

html.files: $(HTML_FILES)

$(HTML_FILES): $(ASSETS_DIR)/%: $(FRONT_DIR)/%
	tidy $(TIDY_SETTINGS) -o $@ $<

# docker

run:
	docker run $(DOCKER_RUN_ARGS) cti_server:$(TAG)

run.local:
	cargo run -p cti_server

shell:
	docker run $(DOCKER_RUN_ARGS) cti_server:$(TAG) sh

shell.build:
	docker run $(DOCKER_RUN_ARGS) cti_server:build bash

# docker compose

up:
	docker compose up -d

up.db:
	docker compose up -d db

down:
	docker compose down --remove-orphans

app.stop:
	docker compose stop app
	docker compose kill app

update: assets app.stop
	docker compose up --build -d --no-deps app
