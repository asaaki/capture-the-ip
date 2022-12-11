DB_CRED = cti_user:example
DB_NAME = cti_dev_diesel

export DATABASE_URL ?= postgres://$(DB_CRED)@localhost/$(DB_NAME)
export DATABASE_ADMIN_URL ?= postgres://$(DB_CRED)@localhost/$(DB_NAME)

export FLY_REGION ?= unset

DOCKER_RUN_ARGS = \
	--rm -ti \
	--add-host host.docker.internal:host-gateway \
	-e DATABASE_URL="$(DATABASE_URL)" \
	-e DATABASE_ADMIN_URL="$(DATABASE_ADMIN_URL)" \
	-e HOST_IP=0.0.0.0 \
	-e FLY_REGION=$(FLY_REGION) \
	-p 3000:3000

default:
	@echo nope

up:
	docker compose up -d

down:
	docker compose down --remove-orphans

image: front
	docker build -t cti_server:build --target builder .
	docker build -t cti_server .

NOW = $(shell bash -c 'date +%s')

publish: front image
	docker images cti_server

	docker tag cti_server:latest registry.fly.io/cti:deployment-$(NOW)
	docker push registry.fly.io/cti:deployment-$(NOW)

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

update: js app.stop
	docker compose up --build -d --no-deps app

# frontend code

ASSETS_DIR = assets
FRONT_DIR = frontend
MAIN_JS = cti.js
MAIN_CSS = cti.css

TIDY_SETTINGS = -q -utf8 \
	--newline LF \
	--wrap 0 --vertical-space auto \
	--drop-empty-elements no \
	--tidy-mark no

# NEEDS:
# tidy - https://binaries.html-tidy.org/
front:
	(cd $(FRONT_DIR) && ls *.html) | \
		xargs -t -P 5 -i tidy $(TIDY_SETTINGS) -o $(ASSETS_DIR)/{} $(FRONT_DIR)/{}
	npx uglify-js $(FRONT_DIR)/$(MAIN_JS) -c -m reserved="['updateTops']" \
		-o $(ASSETS_DIR)/$(MAIN_JS)
	npx csso-cli $(FRONT_DIR)/$(MAIN_CSS) \
		-o $(ASSETS_DIR)/$(MAIN_CSS)
	cp $(FRONT_DIR)/favicon.* $(ASSETS_DIR)/
