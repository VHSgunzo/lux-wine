#!/usr/bin/env bash
set -e

# https://github.com/VHSgunzo/runimage-runtime-static
RUNTIME='runtime-fuse2-all'
RUNTIMESIZE=($(du -sb "$RUNTIME" 2>/dev/null))

# RunDir/static
#     bash        https://github.com/robxu9/bash-static
#     coreutils   https://github.com/VHSgunzo/coreutils-static :
#         cat
#         cp
#         dirname
#         mkdir
#         mv
#         realpath
#     tar         https://github.com/VHSgunzo/tar-static
#     zstd        https://github.com/VHSgunzo/zstd-static

# https://github.com/VHSgunzo/Run-wrapper
# RunDir/Run

# https://github.com/VHSgunzo/lwrun
tar -I 'zstd -T0 --ultra -22 --progress' -cf RunDir/lwrun.tar.zst lwrun
mksquashfs RunDir lwrun -offset "$RUNTIMESIZE" -root-owned -no-xattrs -quiet -mkfs-time 0 -noappend -b 1M -comp lz4 -Xhc
dd if="$RUNTIME" bs="$RUNTIMESIZE" count=1 of=lwrun conv=notrunc
chmod +x lwrun
