.DEFAULT_GOAL := all

CARGO_HOME ?= $(HOME)/.cargo
CARGO_BIN ?= $(CARGO_HOME)/bin
CARGO ?= $(CARGO_BIN)/cargo

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

lint:: | $(CARGO)
	@$(CARGO) make lint

run:: | $(CARGO)
	$(CARGO) run --release

style:: | $(CARGO)
	$(CARGO) make style

test:: | $(CARGO)
	$(CARGO) make test