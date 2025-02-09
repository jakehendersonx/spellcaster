# Compiler settings
CC = gcc
CFLAGS = -Wall -std=c99 -D_DEFAULT_SOURCE
SRC_DIR = src
BUILD_DIR = build

# OS specific settings
ifeq ($(OS),Windows_NT)
    RAYLIB_PATH = raylib
    CFLAGS += -I$(RAYLIB_PATH)/include
    LDFLAGS = -L$(RAYLIB_PATH)/lib -lraylib -lopengl32 -lgdi32 -lwinmm
    EXE = spellcaster.exe
else
    UNAME = $(shell uname)
    ifeq ($(UNAME),Linux)
        LDFLAGS = -lraylib -lGL -lm -lpthread -ldl -lrt -lX11
    endif
    ifeq ($(UNAME),Darwin)
        LDFLAGS = -lraylib -framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo
    endif
    EXE = spellcaster
endif

# Default target
all: $(BUILD_DIR)/$(EXE)

# Compile the executable
$(BUILD_DIR)/$(EXE): $(SRC_DIR)/main.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) $< -o $@ $(LDFLAGS)

# Create build directory
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Clean build files
.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)

# Run the game
.PHONY: run
run: $(BUILD_DIR)/$(EXE)
	./$(BUILD_DIR)/$(EXE)
