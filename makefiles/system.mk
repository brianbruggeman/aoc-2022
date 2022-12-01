OS_NAME := $(shell uname -s)
ifeq "$(OS_NAME)" "Darwin"
OS_ARCH ?= aarch64
endif
