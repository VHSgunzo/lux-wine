#!/usr/bin/env bash

[ ! -d "$RUNDIR" ] && \
RUNDIR="$(dirname "$(realpath "$0" 2>/dev/null)" 2>/dev/null)"
export PATH="$RUNDIR/static:$PATH"

LW_DIR="$HOME/.local/share/LuxWine"
LWBIN_DIR="$LW_DIR/bin"
LWRUN="$LWBIN_DIR/lwrun"

mkdir -p "$LWBIN_DIR"
tar -I 'zstd -T0 --progress' -xf "$RUNDIR/lwrun.tar.zst" -C "$LWBIN_DIR"

exec "$LWRUN" -init
