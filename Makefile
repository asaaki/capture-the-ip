DB_CRED = cti_user:example
DB_NAME = cti_dev_diesel

DATABASE_HOST ?= localhost
export DATABASE_URL ?= postgres://$(DB_CRED)@$(DATABASE_HOST)/$(DB_NAME)
export DATABASE_ADMIN_URL ?= postgres://$(DB_CRED)@$(DATABASE_HOST)/$(DB_NAME)

export FLY_REGION ?= unset

DOCKER_RUN_ARGS = \
	--rm -ti \
	--add-host host.docker.internal:host-gateway \
	-e DATABASE_URL="$(DATABASE_URL)" \
	-e DATABASE_ADMIN_URL="$(DATABASE_ADMIN_URL)" \
	-e HOST_IP=0.0.0.0 \
	-e FLY_REGION=$(FLY_REGION) \
	-p 3000:3000

NOW = $(shell bash -c 'date +%s')
STAMP = $(shell bash -c 'date +%gW%V.%w%H%M') # 22W50.12345

default:
	@echo nope

up:
	docker compose up -d

down:
	docker compose down --remove-orphans

image: assets
	docker build -t cti_server:build --target builder .
	docker build -t cti_server:$(STAMP) .
	docker rmi cti_server:latest
	docker tag cti_server:$(STAMP) cti_server:latest
	docker images cti_server

publish: assets image
	docker tag cti_server:latest registry.fly.io/cti:deployment-$(NOW)
	docker push registry.fly.io/cti:deployment-$(NOW)

deploy:
	fly scale count 1
	sleep 30
	fly deploy
	sleep 30
	fly scale count 3

run:
	docker run $(DOCKER_RUN_ARGS) cti_server

run.local:
	cargo run -p cti_server

shell:
	docker run $(DOCKER_RUN_ARGS) cti_server sh

shell.build:
	docker run $(DOCKER_RUN_ARGS) cti_server:build bash

app.stop:
	docker compose stop app
	docker compose kill app

update: front app.stop
	docker compose up --build -d --no-deps app

# frontend code

ASSETS_DIR = cti_assets/assets
FRONT_DIR = frontend

JS_FILES = \
	$(ASSETS_DIR)/cti.js \
	$(ASSETS_DIR)/cti.v1.js

CSS_FILES = \
	$(ASSETS_DIR)/cti.css \
	$(ASSETS_DIR)/cti.v1.css

HTML_FILES = \
	$(ASSETS_DIR)/index.html \
	$(ASSETS_DIR)/users.html

TIDY_SETTINGS = -q -utf8 \
	--newline LF \
	--wrap 0 --vertical-space auto \
	--drop-empty-elements no \
	--tidy-mark no

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
