# **Lux Wine**
This project allows you to easily run your favorite applications and games for Windows on almost any Linux x86_64 distribution using a specially configured Wine/Proton and [RunImage](https://github.com/VHSgunzo/runimage) container. Lux Wine also comes with a native Steam, which allows you not to think about installing a large number of libraries into your system (especially 32-bit ones) for its work.

## Requirements:

* Supported architectures `x86_64`
* The minimum recommended `Linux kernel` version is `4.18+` with support for `user namespaces`, but `5.0+` would be better
* `FUSE` (but not necessarily, because it is possible to work in unpacked form without `FUSE` mounting)
* The driver for the video card in your system
* Some kind of desktop environment

## **Installation**:
```
curl -sL lwrap.github.io|bash
```
* HF mirror:
```
curl -sL lux-wine-git.static.hf.space|sed 1d|bash
```

* or with `wget`:
```
wget -qO - lwrap.github.io|bash
```
* HF mirror:
```
wget -qO - lux-wine-git.static.hf.space|sed 1d|bash
```

## Video of the installation process:
[![Installation video](https://img.youtube.com/vi/a0vrAgY2uZk/0.jpg)](https://www.youtube.com/watch?v=a0vrAgY2uZk)

## **Offline installation**:
1. Download latest `lwrun` release from the [**releases**](https://github.com/VHSgunzo/lwrun) page ([HF mirror](https://huggingface.co/lux-wine/lwrun/tree/main/releases))
2. Make it executable before run
3. For Nvidia users with a proprietary driver, it is also necessary to download or manually create an [image of the driver](https://github.com/VHSgunzo/runimage-nvidia-drivers) of the corresponding version and put it next to the `lwrun` container
4. Launch the `lwrun` container, the installation will be completely automatic

## **Terminal usage**:
```
┌──[user@linux]─[~]
└──╼ $ lwrun {lwrun arg} blabla.exe {exe args}

      -explorer                   Wine explorer
      -cmd                        Open CMD or open file in CMD
      -shell                      Open SHELL or open file in SHELL
      -config                     Settings
      -regedit                    Registry editor
      -control                    Control panel
      -winecfg                    Wine settings
      -winemgr {delonly}          Wine manager
      -taskmgr                    Task manager
      -uninstaller                Wine uninstaller
      -winetricks {arguments}     Winetricks
      -openpfx                    Open drive C:\
      -killwine                   Kill Wine processes
      -killtray                   Kill tray
      -killexe                    Kill running EXE
      -killshell                  Kill SHELL
      -exit                       Force exit
      -clearpfx                   Clear prefix
      -shortcut                   Create shortcut
      -debug                      DEBUG
      -help                       Show this usage info
      -version                    Show version info
      -tray {noclose}             Tray
      -init                       Forced initialization
      -pfxbackup {xz}             Create prefix backup
      -pfxrestore                 Restore prefix from backup
      -backupmnt                  Mount prefix backup
      -backupunmnt                Unmount prefix backup
      -lsapp                      Show a list of applications
      -runapp {num|name}          Launch the application
      -rmapp {num|name|all}       Remove application shortcut
      -appcfg {num|name|resetall} Applications settings
      -update {all}               Runtime updater
      {dx|dxvk} {vkd|vkd3d} {d3d|d3d_extras|d3d-extras} {eac|eac_runtime}
      {nvml|wine_nvml} {dxnv|dxvk-nvapi|dxvk_nvapi} {wtrx|winetricks}
      {bat|battleye|battleye_runtime} {dg|dgvoodoo2}
```

### **Forced re/install Lux Wine**:
```
lwrun --install
```

### **Forced check for updates**:
```
lwrun --update
```

### **Removing**:
```
lwrun --uninstall
```

## Keyboard shortcuts:
### **MangoHud:**
* Show/Hide: `R_Shift + F12`
* Disable/Enable `FPS limit`: `L_Shift + F1`
* Change postion: `R_Shift + F11`

### **VkBasalt:**
* Disable/Enable: `HOME`

## Features:
- [x] Easy installation without root rights and dependencies
- [x] The ability to install completely offline
- [x] Own portable [lwrun container](https://github.com/VHSgunzo/lwrun) with isolation from the host system
- [x] Ability to install apps from container repositories without root permissions
- [x] Automatic rolling update system
- [x] Quick initialization and launch
- [x] Ability to specify EXE arguments and save them in settings
- [x] Tray mode
- [x] Automatic creation/removal of shortcuts for installed programs and games
- [x] Automatic creation of shortcuts from game launchers
- [x] Ability to create separate configuration files for different EXEs
- [x] Automatic enable WineD3D (OpenGL) mode if Vulkan API is not available
- [x] [DXVK](https://github.com/doitsujin/dxvk/releases) with Wine DXGI mode
- [x] Supports running on virtual machines with GL acceleration
- [x] Debug mode
- [x] Terminal launch mode
- [x] Logging mode
- [x] SHELL mode or open file in SHELL (with access to all lux-wine functions)
- [x] CMD mode or open file in CMD
- [x] Built-in [Wine-staging](https://github.com/wine-staging/wine-staging)
- [x] Built-in [Proton GE](https://github.com/VHSgunzo/ge-proton-lw)
- [x] Wine Manager ([Lutris](https://github.com/lutris/wine/releases), [Proton GE](https://github.com/GloriousEggroll/proton-ge-custom/releases), [Wine GE](https://github.com/GloriousEggroll/wine-ge-custom/releases), [Kron4ek](https://github.com/Kron4ek/Wine-Builds/releases))
- [x] Manage and install multiple wine/proton/dxvk/dxvk-nvapi/vkd3d/d3d_extras/dgvoodoo2 versions and on-the-fly change
- [x] DXR/RTX (Ray Tracing) (if the video adapter support and depends on [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton))
- [x] DLSS (if the video adapter and game support and depends on [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi) + [WINE-NVML](https://github.com/Saancreed/wine-nvml))
- [x] Custom [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton) config
- [x] [MangoHud](https://github.com/flightlessmango/MangoHud/releases) settings (with custom config support)
- [x] [VkBasalt](https://github.com/DadSchoorse/vkBasalt/releases) settings
- [x] [Reshade Shaders](https://github.com/crosire/reshade-shaders) settings (VkBasalt Effects, depends on VkBasalt)
- [x] Vulkan ICD loader automatic and manual settings
- [x] AMD FidelityFX Contrast Adaptive Sharpening settings (depends on [VkBasalt](https://github.com/DadSchoorse/vkBasalt/releases))
- [x] [Gstreamer](https://github.com/GStreamer/gstreamer) (if Wine support)
- [x] [GameMode](https://github.com/FeralInteractive/gamemode/releases)
- [x] [Gamescope](https://github.com/ValveSoftware/gamescope)
- [x] [controllermap](https://aur.archlinux.org/packages/controllermap)
- [x] Esync
- [x] Fsync (if linux kernel support)
- [x] Vsync settings
- [x] [AMD FidelityFX Super Resolution settings (if Wine support)](https://github.com/GloriousEggroll/proton-ge-custom/releases/tag/GE-Proton7-24)
- [x] BattleEye runtime (if Wine support)
- [x] EasyAntiCheat runtime (if Wine support)
- [x] NVIDIA Prime Render Offload
- [x] US keyboard layout switcher
- [x] Reset PulseAudio (+PulseEffects)
- [x] Reduce PulseAudio latency
- [x] Restore gamma
- [x] CPU limitation mode
- [x] Show Crash Dialog (WINEDBG)
- [x] FPS limit settings (with [MangoHud](https://github.com/flightlessmango/MangoHud/releases) or [libstrangle](https://github.com/milaq/libstrangle))
- [x] Custom Wine themes
- [x] Automatic add/remove shortcuts for wine applications
- [x] Quick access to Wine stuff:
```
      * Wine explorer
      * Control panel
      * Wine settings
      * Task manager
      * Wine uninstaller
      * Registry editor
```
- [x] Built-in Winetricks
- [x] Quick open drive C:\
- [x] Killer stuff:
```
      * Kill Wine processes
      * Kill tray
      * Kill Lux Wine running EXE
      * Kill SHELL
      * Force exit (kill all)
```
- [x] Ability to clear a prefix without completely recreating it
- [x] Mouse context menu for Dolphin
- [x] Window compositing management (KDE, MATE, XFCE, Deepin)
- [x] Wine prefix management
- [x] Wine architecture management
- [x] Windows Version management
- [x] Wine DLL overrides
- [x] Wine MONO overrides
- [x] Disabling MONO (.NET Core)
- [x] Various optimizations for better gaming performance
- [x] Custom EXE DB launch helper [scripts](https://github.com/VHSgunzo/lux-wine/blob/main/db)
- [x] Custom loading bar animation
- [x] Wine virtual desktop
- [x] Disable no primary displays (only on x11)
- [x] Restore resolution (only on x11)
- [x] Wayland supports (experimental)
- [x] Wine prefix sandbox
- [x] Downloader selection (aria2c or wget, aria2c in priority)
- [x] Ability to use the built-in Proton D3D DLLs ([DXVK](https://github.com/doitsujin/dxvk/releases), [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi/releases) and [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton))
- [x] Custom Environment (globally and separately for EXE)
- [x] pre_launch and post_launch functions
- [x] Checking and usage anticheat libraries and Protons from native Steam (selected automatically if no other versions of Wine and anticheats are installed)
- [x] [WINE-NVML](https://github.com/Saancreed/wine-nvml) (automatic download if it does not exist in [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi/releases)
- [x] [LatencyFleX](https://github.com/ishitatsuyuki/LatencyFleX) (depends on [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi) >=0.5.3)
- [x] Nvidia Resizable BAR (if your system support)
- [x] Сreating/restoring backups of Wine prefixes
- [x] Mounting/unmounting backups of Wine prefixes
- [x] [OBS-VkCapture](https://github.com/nowrep/obs-vkcapture) supports
- [x] Redefining Wine options in settings GUI:
```
      * WINE_HIDE_NVIDIA_GPU
      * WINE_HEAP_DELAY_FREE
      * STAGING_SHARED_MEMORY
      * WINE_ALLOW_XIM
      * DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1
      * DXVK_ASYNC
      * WINE_LARGE_ADDRESS_AWARE
      * WINE_DISABLE_WRITE_WATCH
```
