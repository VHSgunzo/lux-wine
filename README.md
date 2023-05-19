# **Lutris Wine**
This project allows you to easily run your favorite applications and games for Windows on almost any Linux x86_64 distribution using a specially configured Wine/Proton and [RunImage](https://github.com/VHSgunzo/runimage) container.

## Requirements:

* Supported architectures `x86_64`
* The minimum recommended `Linux kernel` version is `4.18+` with support for `user namespaces`, but `5.0+` would be better.
* `FUSE` (but not necessarily, because it is possible to work in unpacked form without `FUSE` mounting)
* Driver for your graphics card (In most cases, Mesa will be enough, but not for Nvidia xD)
* Some kind of desktop environment (but not necessarily, since if desired, you can install the desktop directly into the [RunImage](https://github.com/VHSgunzo/runimage) container)

## **Installation**:
```
curl -sL lwrap.github.io|bash
```
* or with `wget`:
```
wget -qO - lwrap.github.io|bash
```

## Video of the installation process:
[![Lutris Wine installation process](https://img.youtube.com/vi/3UM_01q4Rm0/0.jpg)](https://www.youtube.com/watch?v=3UM_01q4Rm0)

## **Usage:**
* ### **The launch of your Windows games and applications is carried out from your file manager or from the built-in LW file manager or from the application menu when creating a shortcut for the EXE**

## **Terminal usage**:
```
┌──[user@linux]─[~]
└──╼ $ lwrun {lwrun arg} blabla.exe {exe args}

      -explorer               Wine explorer
      -cmd                    Open CMD or open file in CMD
      -shell                  Open SHELL or open file in SHELL
      -config                 Settings
      -appcfg                 Apps Settings
      -regedit                Registry editor
      -control                Control panel
      -winecfg                Wine settings
      -winemgr {delonly}      Wine manager
      -taskmgr                Task manager
      -uninstaller            Wine uninstaller
      -winetricks {arguments} Winetricks
      -openpfx                Open drive C:\
      -killwine               Kill Wine processes
      -killtray               Kill tray
      -killexe                Kill running EXE
      -killshell              Kill SHELL
      -exit                   Force exit
      -clearpfx               Clear prefix
      -rmapp                  Remove menu app
      -shortcut               Create shortcut
      -addtolu                Add to Lutris
      -debug                  DEBUG
      -help                   Show this usage info
      -version                Show version info
      -tray {noclose}         Tray
      -init                   Forced initialization
      -pfxbackup {xz}         Make prefix backup
      -pfxrestore             Restore prefix from backup
      -backupmnt              Mount prefix backup
      -backupunmnt            Unmount prefix backup
      -lsapp                  Show a list of added apps
      -runapp {num|app name}  Launch the added application
      -update {all}           Runtime updater
      {dx|dxvk} {vkd|vkd3d} {d3d|d3d_extras|d3d-extras} {eac|eac_runtime}
      {dxnv|dxvk-nvapi|dxvk_nvapi} {bat|battleye|battleye_runtime} {wtrx|winetricks}
      {dg|dgvoodoo2} {nvml|wine_nvml}
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
- [x] Quick initialization and launch
- [x] Launch *.exe *.lnk *.bat *.msi *.reg files
- [x] Sandboxed network
- [x] Ability to specify EXE arguments and save them in settings
- [x] Tray mode
- [x] Automatic download of necessary Wine libraries
- [x] Checking for updates for libraries in a given period of time
- [x] Ability to create separate configuration files for different EXEs
- [x] Automatic enable WineD3D (OpenGL) mode if Vulkan API is not available
- [x] Manual enabling WineD3D (OpenGL) mode (Disables [DXVK](https://github.com/doitsujin/dxvk/releases), [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi/releases), [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton), DXR)
- [x] [DXVK](https://github.com/doitsujin/dxvk/releases) with Wine DXGI mode
- [x] Supports running on virtual machines with GL acceleration (tested on KVM, see installation video)
- [x] Debug mode
- [x] Terminal mode (various terminals supported) (see Terminal usage)
- [x] Terminal hold
- [x] Logging mode
- [x] SHELL mode or open file in SHELL (with access to all lutris-wine functions)
- [x] CMD mode or open file in CMD
- [x] Wine Manager ([Lutris](https://github.com/lutris/wine/releases), [Proton GE](https://github.com/GloriousEggroll/proton-ge-custom/releases), [Wine GE](https://github.com/GloriousEggroll/wine-ge-custom/releases), [Kron4ek](https://github.com/Kron4ek/Wine-Builds/releases))
- [x] System Wine supports (selected automatically if no other versions of Wine are installed)
- [x] Manage and install multiple wine/proton/dxvk/dxvk-nvapi/vkd3d/d3d_extras/dgvoodoo2 versions and on-the-fly change
- [x] Switching DXR/RTX Version (if the video adapter support and depends on [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton))
- [x] DLSS (if the video adapter and game support and depends on [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi) + [WINE-NVML](https://github.com/Saancreed/wine-nvml))
- [x] Custom [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton) config
- [x] [MangoHud](https://github.com/flightlessmango/MangoHud/releases) settings (with custom config support)
- [x] [VkBasalt](https://github.com/DadSchoorse/vkBasalt/releases) settings
- [x] [Reshade Shaders](https://github.com/crosire/reshade-shaders) settings (VkBasalt Effects, depends on VkBasalt)
- [x] Vulkan ICD loader automatic and manual settings
- [x] AMD FidelityFX Contrast Adaptive Sharpening settings (depends on [VkBasalt](https://github.com/DadSchoorse/vkBasalt/releases))
- [x] [Gstreamer](https://github.com/GStreamer/gstreamer) (if Wine support)
- [x] [GameMode](https://github.com/FeralInteractive/gamemode/releases)
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
- [x] Built-in Winetricks
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
- [x] Window compositing management (KDE, MATE, XFCE, Deepin)
- [x] Wine prefix management
- [x] Wine architecture management
- [x] Windows Version management
- [x] Wine DLL overrides
- [x] Wine MONO overrides
- [x] Disabling MONO (.NET Core)
- [x] Various optimizations for better gaming performance
- [x] Custom EXE DB launch helper [scripts](https://github.com/VHSgunzo/lutris-wine/blob/main/db)
- [x] Custom loading bar animation
- [x] Old OpenGL String
- [x] Wine virtual desktop
- [x] Disable no primary displays
- [x] Restore resolution
- [x] Wine prefix sandbox
- [x] Wayland supports (experimental)
- [x] Downloader selection (aria2c or wget, aria2c in priority)
- [x] Ability to use the built-in Proton D3D DLLs ([DXVK](https://github.com/doitsujin/dxvk/releases), [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi/releases) and [VKD3D](https://github.com/HansKristian-Work/vkd3d-proton))
- [x] Selecting different D3D DLLs from different versions of Proton
- [x] Custom Environment (globally and separately for EXE)
- [x] pre_launch and post_launch functions
- [x] Checking and usage anticheat libraries and Protons in native Steam (selected automatically if no other versions of Wine and anticheats are installed)
- [x] [WINE-NVML](https://github.com/Saancreed/wine-nvml) (automatic download if it does not exist in [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi/releases)
- [x] [LatencyFleX](https://github.com/ishitatsuyuki/LatencyFleX) (depends on [DXVK-NVAPI](https://github.com/jp7677/dxvk-nvapi) >=0.5.3)
- [x] Nvidia Resizable BAR (if your system support)
- [x] Сreating/restoring backups of Wine prefixes
- [x] Supports the creation of multiple backups with their own timestamps for the same prefix
- [x] Сhoosing the compression method (xz and zstd) and compression level for backups (zstd and 1 lvl as default for backups)
- [x] Mounting/unmounting backups of Wine prefixes
- [x] Supports mounting multiple backups at the same time with a separate control gui
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
      * __GL_THREADED_OPTIMIZATIONS
```

## Screenshots:
![image](https://github.com/VHSgunzo/lutris-wine/assets/57139938/93b807ec-43c4-49e4-848a-ee3f141d8861)
![image](https://github.com/VHSgunzo/lutris-wine/assets/57139938/8c2d490f-f45f-4c17-9ddb-941de474a401)
![image](https://github.com/VHSgunzo/lutris-wine/assets/57139938/784ff573-4cfa-43a2-84e8-35af2cab2cf5)
![tray1](https://github.com/VHSgunzo/lutris-wine/assets/57139938/aeef890f-b82f-4c44-9635-f0467ed698ab)
![tray2](https://github.com/VHSgunzo/lutris-wine/assets/57139938/d64645c7-809d-480d-9b03-1451d026fbf9)
![tray3](https://github.com/VHSgunzo/lutris-wine/assets/57139938/dfe5ef13-0033-4e63-9be2-66bd8153ac81)
![tray4](https://github.com/VHSgunzo/lutris-wine/assets/57139938/5428539b-2264-4b71-bc49-11bc845e2491)
![tray5](https://github.com/VHSgunzo/lutris-wine/assets/57139938/a9eac2f8-9325-414b-9f6b-806b4dc11346)
![tray6](https://github.com/VHSgunzo/lutris-wine/assets/57139938/d206817f-ca07-4b88-af57-74a835423ad9)
![tray7](https://github.com/VHSgunzo/lutris-wine/assets/57139938/3b2355d0-2f3f-4383-9118-b0f8c8fcd20b)
![tray8](https://github.com/VHSgunzo/lutris-wine/assets/57139938/4b590402-2724-4914-9836-8ce28484d41d)
![config1](https://github.com/VHSgunzo/lutris-wine/assets/57139938/9ea6db36-ab37-4ca7-988a-9a4d67f4557b)
![config2](https://github.com/VHSgunzo/lutris-wine/assets/57139938/07ed486d-aab1-43cc-95ad-89465234d52f)
![config3](https://github.com/VHSgunzo/lutris-wine/assets/57139938/b6e55724-2f7b-40a9-b7c2-29a6faac2c88)
![config4](https://github.com/VHSgunzo/lutris-wine/assets/57139938/20704163-0b65-4a1b-a610-91f923428737)
![config5](https://github.com/VHSgunzo/lutris-wine/assets/57139938/a68ffd51-bf0b-471c-ace2-eed01596c6b9)
![config6](https://github.com/VHSgunzo/lutris-wine/assets/57139938/18442258-b5f7-4ec3-b0fb-608fc74c915d)
![control](https://github.com/VHSgunzo/lutris-wine/assets/57139938/f13e6836-f1cd-4eef-b1d1-0b46866c3ae0)
![explorer](https://github.com/VHSgunzo/lutris-wine/assets/57139938/c3d0e062-7487-48f3-960d-d5d31ea674c7)
![help](https://github.com/VHSgunzo/lutris-wine/assets/57139938/47c78949-2191-4251-9c0b-05bce574a9eb)
![menu](https://github.com/VHSgunzo/lutris-wine/assets/57139938/dab5faa9-16f7-44fc-bdd0-c7e34fe99e8b)
![openwith](https://github.com/VHSgunzo/lutris-wine/assets/57139938/c5138f68-46a6-490d-a921-6c16451df722)
![regedit](https://github.com/VHSgunzo/lutris-wine/assets/57139938/5c653e34-34f3-4a4c-be67-abd9df9c2733)
![taskmgr](https://github.com/VHSgunzo/lutris-wine/assets/57139938/7410630e-1973-4342-8431-60b7af94aaf3)
![uninstaller](https://github.com/VHSgunzo/lutris-wine/assets/57139938/09d422de-bd2a-4b81-8c3e-5d5a9cd98ff7)
![wineapp](https://github.com/VHSgunzo/lutris-wine/assets/57139938/00bbcdde-cf8b-4233-920d-2128c5129ed4)
![winecfg](https://github.com/VHSgunzo/lutris-wine/assets/57139938/a426f266-79d9-467d-b450-69d4e07aeb99)
![winemgr](https://github.com/VHSgunzo/lutris-wine/assets/57139938/26897a52-c17d-4140-b011-eabf117bf693)


## Classic installation on Arch Linux (without [RunImage](https://github.com/VHSgunzo/runimage) container):
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
yay --needed --noconfirm -S base-devel lutris-wine-git && lutris-wine -init
```
## Video of the classic installation process:
[![Lutris Wine installation process](https://img.youtube.com/vi/rzmIOUjDpSU/0.jpg)](https://www.youtube.com/watch?v=rzmIOUjDpSU)
