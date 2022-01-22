# Lutris Wine
## Supports:
```
All Arch-based distributions
```
## Features:
- [x] Easy setup
- [x] Quick initialization and launch
- [x] Launch *.exe *.lnk *.bat *.msi *.reg files in one click
- [x] Ability to specify EXE arguments and save them in settings
- [x] Tray launch
- [x] Full system integration
- [x] Automatic download of necessary Wine libraries
- [x] Checking for updates for libraries in a given period of time
- [x] Ability to create separate configuration files for different EXEs
- [x] Automatic enable WineD3D (OpenGL) mode if Vulkan API is not available
- [x] Supports running on virtual machines with GL acceleration (tested on KVM, see installation video)
- [x] Debug mode
- [x] Terminal mode (various terminals supported)
- [x] Logging mode
- [x] Shell mode or open file in SHELL (with access to all lutris-wine functions)
- [x] CMD mode or open file in CMD
- [x] Wine Manager (Lutris, GloriousEggroll, Kron4ek)
- [x] System Wine Support
- [x] Manage and install multiple wine/proton/dxvk/dxvk-nvapi/vkd3d/d3d_extras/dgvoodoo2 versions and on-the-fly change
- [x] Custom URL for runtime repo
- [x] Custom Wine themes
- [x] Runtime updater
- [x] Quick add applications to Lutris (if Lutris installed)
- [x] Add/remove applications in the menu
- [x] Quick access to Wine stuff:
```
      * Wine explorer
      * Control panel
      * Wine settings
      * Task manager
      * Wine uninstaller
      * Registry editor
```
- [x] GUI for custom apps configs
- [x] Built-in fixed Winetricks
- [x] System Winetricks Support
- [x] Quick open drive C:\
- [x] Killer stuff:
```
      * Kill Wine processes
      * Kill tray
      * Kill Lutris Wine running EXE
      * Kill SHELL
      * Force exit (kill all)
```
- [x] Ability to clear a prefix without completely recreating it
- [x] Mouse context menu for Dolphin
- [x] Ability to run applications from the terminal (see lutris-wine -help)
- [x] Window compositing management (KDE, MATE, XFCE, Deepin)
- [x] Wine prefix management
- [x] Wine architecture management
- [x] Windows Version management
- [x] Wine DLL overrides
- [x] Various optimizations for better gaming performance
- [x] Lutris Runtime (with system library priority if needed)
- [x] Custom EXE DB launch helper scripts, like this [LOL anticheat helper](https://github.com/VHSgunzo/lutris-wine/blob/main/usr/share/lutris-wine/db/LeagueClient.lwdb)
- [x] Custom loading bar animation
- [x] Lots of settings in GUI:
```
      * DXR/RTX Version (if video adapter suports)
      * Gstreamer
      * VkBasalt Effects
      * AMD FidelityFX Contrast Adaptive Sharpening
      * GameMode
      * MangoHud (With custom config)
      * FPS Limit
      * Esync
      * Fsync
      * Vsync
      * AMD FidelityFX Super Resolution (if Wine supports)
      * BattleEye Runtime
      * NVIDIA Prime Render Offload
      * Switch to US keyboard layout
      * Reset PulseAudio (+PulseEffects)
      * Reduce PulseAudio latency
      * Restore gamma
      * Single CPU mode
      * Show Crash Dialog
      * OLD GL STRING
      * NO WRITE WATCH
      * VULKAN NO ASYNC
      * HIDE NVIDIA GPU
      * HEAP DELAY FREE
      * STAGING SHARED MEMORY
      * ALLOW XIM
      * DISABLE LAYER AMD SWITCHABLE GRAPHICS
      * WineD3D (OpenGL) Mode
      * Disable MONO (.NET Core)
      * DXVK
      * VKD3D
      * DXVK-NVAPI/DLSS
      * D3D EXTRAS
      * DGVOODOO2
      * WINE MONO OVERRIDES
      * Hold terminal
```
- [ ] And much more
## To get started:
* **Install the latest video drivers for your video adapter:**
```
!!!Make sure your video adapter supports Vulkan API!!!

## NVIDIA ##
sudo pacman -Sy lib32-nvidia-utils lib32-opencl-nvidia libxnvctrl nvidia nvidia-dkms nvidia-settings nvidia-utils opencl-nvidia

## AMD ##
sudo pacman -Sy lib32-vulkan-radeon vulkan-radeon xf86-video-amdgpu

## INTEL ##
sudo pacman -Sy lib32-vulkan-intel vulkan-intel lib32-libva-intel-driver libva-intel-driver libva-utils xf86-video-intel
```
* **Install Lutris Wine using your favorite aur package manager:**
```
yay --needed --noconfirm -Sy base-devel lutris-wine-git && lutris-wine -help
```
## Installation video:
[![Lutris Wine installation process](https://img.youtube.com/vi/pozypVaPK0Y/0.jpg)](https://www.youtube.com/watch?v=pozypVaPK0Y)

## Terminal usage:
```
┌──[user@arch]─[~]
└──╼ $ lutris-wine {lutris-wine argument} blabla.exe {exe arguments}

-explorer               Wine explorer
-cmd                    Open CMD or open file in CMD
-shell                  Open SHELL or open file in SHELL
-config                 Lutris Wine settings
-appcfg                 Applications Settings
-regedit                Registry editor
-control                Control panel
-winecfg                Wine settings
-winemgr                Wine manager
-taskmgr                Task manager
-uninstaller            Wine uninstaller
-winetricks             Winetricks
-openpfx                Open drive C:
-killwine               Kill Wine processes
-killtray               Kill Lutris Wine tray
-killexe                Kill Lutris Wine running EXE
-killshell              Kill Lutris Wine SHELL
-exit                   Lutris Wine force exit
-clearpfx               Clear prefix
-rmapp                  Remove Lutris Wine shortcuts from menu
-shortcut               Create shortcut
-addtolu                Add to Lutris
-debug                  DEBUG
-help                   Show this usage info
-version                Show version info
-tray {noclose}         Open Lutris Wine in tray
-update {all}           Runtime updater
{dx|dxvk} {p7|p7zip} {vkd|vkd3d} {ub1804_8664|ubuntu1804-x86_64}
{inn|innoextract} {net|network-tools} {d3d|d3d_extras|d3d-extras}
{dxnv|dxvk-nvapi|dxvk_nvapi} {bat|battleye|battleye_runtime}{wtrx|winetricks}
{ga|gamectrlrdb|gamecontrollerdb} {dg|dgvoodoo2} {ub1804_686|ubuntu1804-i686}
```
