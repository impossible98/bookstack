# See in https://www.gnu.org/software/make/manual/html_node/index.html
# for more information about Makefile.
.POSIX:
SHELL := /bin/bash
CARGO := $(shell which cargo)
YARN :=  $(shell which yarn)

BIN_NAME := bookstack

build:
	$(CARGO) build --release
	if [ -f "target/release/$(BIN_NAME)" ]; then \
		if [ ! -d "dist" ]; then \
			mkdir dist; \
		fi; \
		cp target/release/$(BIN_NAME) dist/$(BIN_NAME); \
	fi

# Clear the application
.PHONY: clear
clear: install
	if [ -d "dist" ]; then \
		rm -rf "dist"; \
	fi
# Local development
.PHONY: dev
dev:
	$(CARGO) run


# Format the code
.PHONY: fmt
fmt: install
	@echo -e "\033[32mFormatting the code...\033[0m"
	$(YARN) run fmt
	$(CARGO) fmt
	@echo -e "\033[32mFormatting finished.\033[0m"

# Lint the code
.PHONY: lint
lint: install
	@echo -e "\033[32mLinting the code...\033[0m"
	$(CARGO) check
	@echo -e "\033[32mLinting finished.\033[0m"

# Install dependencies
.PHONY: install
install:
	@echo -e "\033[32mInstalling dependencies...\033[0m"
	if [ ! -d "node_modules" ]; then \
		$(YARN); \
	fi
	@echo -e "\033[32mDependencies installed.\033[0m"
