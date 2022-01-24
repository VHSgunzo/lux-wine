# Lutris Wine

## Supports:
```
All Arch-based distributions
```

## Features:
- [x] Easy setup
- [x] Full system integration
- [x] Quick initialization and launch
- [x] Does not use containerization like steam runtime
- [x] This is not another game launcher
- [x] Launch *.exe *.lnk *.bat *.msi *.reg files in one click
- [x] Ability to specify EXE arguments and save them in settings
- [x] Tray mode
- [x] Automatic download of necessary Wine libraries
- [x] Checking for updates for libraries in a given period of time
- [x] Ability to create separate configuration files for different EXEs
- [x] Automatic enable WineD3D (OpenGL) mode if Vulkan API is not available
- [x] Manual enabling WineD3D (OpenGL) mode (Disables DXVK, DXVK-NVAPI, VKD3D, DXR)
- [x] Supports running on virtual machines with GL acceleration (tested on KVM, see installation video)
- [x] Debug mode
- [x] Terminal mode (various terminals supported)
- [x] Logging mode
- [x] SHELL mode or open file in SHELL (with access to all lutris-wine functions)
- [x] CMD mode or open file in CMD
- [x] Wine Manager (Lutris, Proton GE , Wine GE, Kron4ek)
- [x] System Wine supports (selected automatically if no other versions of Wine are installed)
- [x] Manage and install multiple wine/proton/dxvk/dxvk-nvapi/vkd3d/d3d_extras/dgvoodoo2 versions and on-the-fly change
- [x] Switching DXR/RTX Version (if the video adapter support and depends on VKD3D)
- [x] Custom URL for runtime repo
- [x] Custom VKD3D config
- [x] MangoHud settings (with custom config support)
- [x] VkBasalt settings
- [x] Reshade Shaders settings (VkBasalt Effects, depends on VkBasalt)
- [x] Vulkan ICD loader automatic and manual settings
- [x] AMD FidelityFX Contrast Adaptive Sharpening settings (depends on VkBasalt)
- [x] Gstreamer (if Wine support, system Gstreamer in priority)
- [x] GameMode
- [x] Esync
- [x] Fsync (if linux kernel support)
- [x] Vsync settings
- [x] AMD FidelityFX Super Resolution settings (if Wine support)
- [x] BattleEye Runtime (if Wine support)
- [x] NVIDIA Prime Render Offload
- [x] US keyboard layout switcher
- [x] Reset PulseAudio (+PulseEffects)
- [x] Reduce PulseAudio latency
- [x] Restore gamma
- [x] Single CPU mode
- [x] Show Crash Dialog (WINEDBG)
- [x] FPS limit settings (with MangoHud or libstrangle)
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
- [x] System Winetricks support
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
- [x] Ability to run applications from the terminal (see Terminal usage)
- [x] Terminal hold
- [x] Window compositing management (KDE, MATE, XFCE, Deepin)
- [x] Wine prefix management
- [x] Wine architecture management
- [x] Windows Version management
- [x] Wine DLL overrides
- [x] Wine MONO overrides
- [x] Disabling MONO (.NET Core)
- [x] Various optimizations for better gaming performance
- [x] Lutris Runtime (with system library priority if needed)
- [x] Custom EXE DB launch helper scripts, like this [LOL anticheat helper](https://github.com/VHSgunzo/lutris-wine/blob/main/usr/share/lutris-wine/db/LeagueClient.lwdb)
- [x] EXE DB launch helper script automatically starts if located in the application folder and has the same name, but with the extension *.lwdb
- [x] Custom loading bar animation
- [x] Redefining Wine options in settings GUI:
```
      * OLD GL STRING
      * NO WRITE WATCH
      * VULKAN NO ASYNC
      * HIDE NVIDIA GPU
      * HEAP DELAY FREE
      * STAGING SHARED MEMORY
      * ALLOW XIM
      * DISABLE LAYER AMD SWITCHABLE GRAPHICS
```
- [ ] And much more

## To get started:
* **Enable multilib in the pacman config:**
```
sudo sed -i "/\[multilib\]/,/Include/"'s/^#//' /etc/pacman.conf
```
* **Upgrade your system:**
```
sudo pacman -Syu
```
* **Install the latest video drivers for your video adapter:**
```
!!!Make sure your video adapter supports Vulkan API!!!

## NVIDIA ##
sudo pacman -S lib32-nvidia-utils lib32-opencl-nvidia libxnvctrl nvidia nvidia-dkms nvidia-settings nvidia-utils opencl-nvidia

## AMD ##
sudo pacman -S lib32-vulkan-radeon vulkan-radeon xf86-video-amdgpu

## INTEL ##
sudo pacman -S lib32-vulkan-intel vulkan-intel lib32-libva-intel-driver libva-intel-driver libva-utils xf86-video-intel
```
* **Reboot your system:**
```
sudo reboot
```
* **Install Lutris Wine using your favorite AUR package manager:**
```
yay --needed --noconfirm -S base-devel lutris-wine-git && lutris-wine -help
```
## Video with the installation process:
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
