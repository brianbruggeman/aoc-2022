.DEFAULT_GOAL := all

include makefiles/system.mk

USR_LOCAL ?= /usr/local
LOCAL_BIN ?= $(USR_LOCAL)/bin

HEY ?= $(LOCAL_BIN)/hey
DOCKER ?= $(LOCAL_BIN)/docker
CARGO_HOME ?= $(HOME)/.cargo
CARGO_BIN ?= $(CARGO_HOME)/bin
CARGO ?= $(CARGO_BIN)/cargo
RUSTUP ?= $(CARGO_BIN)/rustup

REST_BIND ?= http://localhost:3000


all:: | $(CARGO)
	$(CARGO) make

build:: | $(CARGO)
	$(CARGO) build

clean::
	rm -Rf ./target

debug:: | $(CARGO)
	$(CARGO) run

fmt:: | $(CARGO)
	$(CARGO) make format

image::
	$(eval LOCAL_CARGO_TARGET_DIR ?= ./target/docker/cargo)
	$(eval CONTEXT ?= .)
	mkdir -p $(LOCAL_CARGO_TARGET_DIR)
	$(DOCKER) build \
		--build-arg BUILDER_IMAGE_NAME=rust \
		--build-arg BUILDER_IMAGE_VERSION=1.61.0-slim \
		--build-arg CPU_TARGET=$(OS_ARCH)-unknown-linux-musl \
		--build-arg BIN_NAME=axum-test \
		--build-arg PACKAGE_NAME=axum_test \
		--build-arg PACKAGE_LOCATION=/app \
		--build-arg LOCAL_CARGO_TARGET_DIR=$(LOCAL_CARGO_TARGET_DIR) \
		--progress plain \
		--target base \
		--tag axum-test:builder \
		$(CONTEXT)
	$(DOCKER) container rm remove-me
	$(DOCKER) container create --name remove-me axum-test:builder
	$(DOCKER) cp remove-me:/cargo ./target/docker
	$(DOCKER) container rm remove-me
	$(DOCKER) build \
		--build-arg RUNTIME_IMAGE_NAME=alpine \
		--build-arg RUNTIME_IMAGE_VERSION=3.16.0 \
		--build-arg BUILDER_IMAGE_NAME=rust \
		--build-arg BUILDER_IMAGE_VERSION=1.61.0-slim \
		--build-arg PACKAGE_NAME=axum_test \
		--build-arg PACKAGE_LOCATION=/app \
		--build-arg RUNTIME_PORT=3030 \
		--progress plain \
		--target runtime \
		--tag axum-test:latest \
		$(CONTEXT)

run:: | $(CARGO)
	$(CARGO) run --release

load:: | $(HEY)
	$(eval CONCURRENT_CONNECTIONS ?= 100)
	$(eval TIMEOUT ?= 100s)
	$(HEY) -c $(CONCURRENT_CONNECTIONS) -z $(TIMEOUT) "$(REST_BIND)/unprotected"

smoke:: | $(CARGO)
	@echo "/      -> \c" && curl $(REST_BIND)/ && echo
	@echo "/users -> \c" && curl $(REST_BIND)/unprotected && echo

test:: | $(CARGO)
	$(CARGO) make test


# $(CARGO): $(RUSTUP)

# $(RUSTUP):
# 	@echo "Installing rust compiler and tools"
# 	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
