#!/usr/bin/env bash

which_exe() { command -v "$@" ; }

try_mv_nvdrv() {
    mkdir -p "$NVIDIA_DRIVERS_DIR"
    if [ -w "$1" ]
        then mv -f "$1" "$NVIDIA_DRIVERS_DIR/" 2>/dev/null
        else cp -rf "$1" "$NVIDIA_DRIVERS_DIR/" 2>/dev/null
    fi
}

if [[ -n "$RUNOFFSET" && -n "$ARGV0" ]]
    then
        RUNSTATIC="$RUNDIR/static"
        [ "$SYS_TOOLS" == 1 ] && \
            export PATH="$PATH:$RUNSTATIC"||\
            export PATH="$RUNSTATIC:$PATH"
        if [ ! -n "$RUNIMAGE" ] # KDE Neon, CachyOS, Puppy Linux bug
            then
                if [ -x "$(realpath "$ARGV0" 2>/dev/null)" ]
                    then RUNIMAGE="$(realpath "$ARGV0" 2>/dev/null)"
                elif [ -x "$(realpath "$(which_exe "$ARGV0")" 2>/dev/null)" ]
                    then RUNIMAGE="$(realpath "$(which_exe "$ARGV0")" 2>/dev/null)"
                else RUNIMAGE="$ARGV0"
                fi
        fi
        PRUNDIR="$(dirname "$RUNIMAGE" 2>/dev/null)"
    else
        [ ! -d "$RUNDIR" ] && \
        RUNDIR="$(dirname "$(realpath "$0" 2>/dev/null)" 2>/dev/null)"
        RUNSTATIC="$RUNDIR/static"
        [ "$SYS_TOOLS" == 1 ] && \
            export PATH="$PATH:$RUNSTATIC"||\
            export PATH="$RUNSTATIC:$PATH"
        PRUNDIR="$(realpath "$RUNDIR/../" 2>/dev/null)"
fi

export LWRUN_PKG=1
LW_DIR="$HOME/.local/share/LuxWine"
LWBIN_DIR="$LW_DIR/bin"
LWRUN="$LWBIN_DIR/lwrun"
NVIDIA_DRIVERS_DIR="$LW_DIR/bin/nvidia-drivers"

mkdir -p "$LWBIN_DIR"
tar -I 'zstd -T0 --progress' -xf "$RUNDIR/lwrun.tar.zst" -C "$LWBIN_DIR"

if [[ "$SANDBOX_HOME" != 1 && \
      "$SANDBOX_HOME_DL" != 1 && \
      ! -d "$SANDBOXHOMEDIR/lwrap" && \
      "$PORTABLE_HOME" != 1 && \
      "$TMP_HOME" != 1 && \
      "$TMP_HOME_DL" != 1 && \
      ! -d "$PORTABLEHOMEDIR/lwrap" ]]
    then
        nvidia_version="$(cat /sys/module/nvidia/version 2>/dev/null)"
        if [ -n "$nvidia_version" ]
            then
                if [[ ! -d "$NVIDIA_DRIVERS_DIR/$nvidia_version" && \
                      ! -f "$NVIDIA_DRIVERS_DIR/$nvidia_version.nv.drv" ]]
                    then
                        if [ -d "$PRUNDIR/$nvidia_version" ]
                            then try_mv_nvdrv "$PRUNDIR/$nvidia_version"
                        elif [ -f "$PRUNDIR/$nvidia_version.nv.drv" ]
                            then try_mv_nvdrv "$PRUNDIR/$nvidia_version.nv.drv"
                        fi
                fi
        fi
fi

exec "$LWRUN" -init
