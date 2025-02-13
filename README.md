# Spellcaster

a game where you are a wizard, and must learn how to execute and combine spells to accomplish life goals
... but spells can only be executed by typing, and real expirementation is required to create combinations

### Development

This game was developed on a macbook pro using Raylib, in C and Rust.
Running Raylib on a mac isn't straight forward so a directory was shared with a Linux Docker container.
Raylib depends on X11, a common windowing and graphics system for Linux, so we must install those labraries
in our container.

with Docker desktop running, execute...

```bash
docker build -t spellcaster .
docker-compose up --build -d
docker exec -it spellcaster bash
```

### installation and setup

use the raylib quickstart for this (already done just fyi)

```bash
git clone git@github.com:raylib-extras/raylib-quickstart.git
```

in the base directory

```bash
mv raylib/ spellcaster/
```

```bash
cd spellcaster/build
./premake5.osx gmake2
cd ..
make
```
