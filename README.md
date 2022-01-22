# Lutris Wine
## Supports:
```
All Arch-based distributions
```
## Features:
```
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
- [x] Manage and install multiple wine/proton/dxvk/dxvk-nvapi/vkd3d/d3d_extras/dgvoodoo2 versions and on-the-fly change
- [x] Custom URL for runtime repo
- [x] Custom Wine themes
- [x] Runtime updater
- [x] Quick add applications to Lutris (if installed)
- [x] Add/remove applications in the menu
- [x] Quick access to Wine stuff:
   - [x] Wine explorer
   - [x] Control panel
   - [x] Wine settings
   - [x] Task manager
   - [x] Wine uninstaller
   - [x] Registry editor
- [x] GUI for custom apps configs
- [x] Built-in fixed Winetricks
- [x] Quick open drive C:\
- [x] Killer stuff:
   - [x] Kill Wine processes
   - [x] Kill tray
   - [x] Kill Lutris Wine running EXE
   - [x] Kill SHELL
   - [x] Force exit (kill all)
- [x] Ability to clear a prefix without completely recreating it
- [x] Mouse context menu for Dolphin
- [x] Ability to run applications from the terminal (see lutris-wine -help)
- [x] Window compositing management (KDE, MATE, XFCE, Deepin)
- [x] Wine prefix management
- [x] Wine architecture management
- [x] Windows Version management
- [x] DLL overrides
- [x] Various optimizations for better gaming performance
- [x] Lutris Runtime
- [x] Custom EXE DB launch helper scripts, like this [LOL anticheat helper](https://github.com/VHSgunzo/lutris-wine/blob/main/usr/share/lutris-wine/db/LeagueClient.lwdb)
- [x] Custom loading bar animation
- [x] Lots of settings in GUI:
   - [x] DXR/RTX Version (if video adapter suports)
   - [x] Gstreamer
   - [x] VkBasalt Effects
   - [x] AMD FidelityFX Contrast Adaptive Sharpening
   - [x] GameMode
   - [x] MangoHud (With custom config)
   - [x] FPS Limit
   - [x] Esync
   - [x] Fsync
   - [x] Vsync
   - [x] AMD FidelityFX Super Resolution (if Wine supports)
   - [x] BattleEye Runtime
   - [x] NVIDIA Prime Render Offload
   - [x] Switch to US keyboard layout
   - [x] Reset PulseAudio (+PulseEffects)
   - [x] Reduce PulseAudio latency
   - [x] Restore gamma
   - [x] Single CPU mode
   - [x] Show Crash Dialog
   - [x] OLD GL STRING
   - [x] NO WRITE WATCH
   - [x] VULKAN NO ASYNC
   - [x] HIDE NVIDIA GPU
   - [x] HEAP DELAY FREE
   - [x] STAGING SHARED MEMORY
   - [x] ALLOW XIM
   - [x] DISABLE LAYER AMD SWITCHABLE GRAPHICS
   - [x] WineD3D (OpenGL) Mode
   - [x] Disable MONO (.NET Core)
   - [x] DXVK
   - [x] VKD3D
   - [x] DXVK-NVAPI/DLSS
   - [x] D3D EXTRAS
   - [x] DGVOODOO2
   - [x] WINE MONO OVERRIDES
   - [x] Hold terminal
   - [ ] And much more
```
## To get started:
* **Install the latest video drivers for your video card:**
```
!!!Make sure your video card supports Vulkan API!!!

## NVIDIA ##
sudo pacman -Sy lib32-nvidia-utils lib32-opencl-nvidia libxnvctrl nvidia nvidia-dkms nvidia-settings nvidia-utils opencl-nvidia

## AMD ##
sudo pacman -Sy lib32-vulkan-radeon vulkan-radeon xf86-video-amdgpu

## INTEL ##
sudo pacman -Sy lib32-vulkan-intel vulkan-intel lib32-libva-intel-driver libva-intel-driver libva-utils xf86-video-intel
```
* Terminal usage:
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
* **Install Lutris Wine using your favorite aur package manager**
```
yay --needed --noconfirm -Sy base-devel lutris-wine-git && lutris-wine -help
```
[![Alt text](https://img.youtube.com/vi/pozypVaPK0Y/0.jpg)](https://www.youtube.com/watch?v=pozypVaPK0Y)

