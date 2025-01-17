# v0.80.2

* Fix hostexec hang on legacy forking

# v0.80.1

* Update [lwrun](https://github.com/VHSgunzo/runimage/releases/download/v0.40.1) container `v0.40.1`
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.22) `v9.22`
* Change container filesystem to [DwarFS](https://github.com/mhx/dwarfs) (reduce size)
* Enable `OverlayFS` (read-write) mode by default
* Now you can install any apps and updates from the container repos without rebuilding the container
* Fix `firefox`, `steam`, `lux-wine` decorations (now as in the system)
* Remove `AMDVLK` vulkan driver
* Update `lwrap`
* Add `ru_RU.KOI8-R` locale
* Remove `lwexec` and all the other pieces that are now part of the container.
* Add [continuous](https://github.com/VHSgunzo/lwrun/releases/download/continuous) update CI for lwrun
* Add ability to enable sandbox for `HOME` or create tmpfs `HOME`
* Add ability to `lwrun` to system `/usr/bin` `/bin` `/opt` path
* Replace `glibc` and `lib32-glibc` with `glibc-eac` and `lib32-glibc-eac`
* Add an attempt to use a local proprietary Nvidia driver (if there are no 32-bit libs, the driver will be downloaded as before)

# v0.78.5.5

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.9) `v9.21`
* Add NVK vulkan driver
* Add AMDVLK vulkan driver
* Remove DB script for `League of Legends`
* Fix `lwrap` download GUI
* Minor fixes

# v0.78.5.4

* Remove `Yandex Cloud` mirrors

# v0.78.5.3

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Add [lw-tray on golang](https://github.com/VHSgunzo/lw-tray/commit/d5976a68237d9a8d26ab2e8bdb47803ea565bdd4)
* Minor fixes

# v0.78.5.2

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.9) `v9.11`
* Add `winepath()` and `winpath()` functions
* Add gstreamer fixes
* Update DB script for `Honkai: Star Rail` (for StarRail.exe)
* Minor fixes

# v0.78.5.1

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.9) `v9.9`
* Add grep CHK_UPDATE from LW global config to lwrap
* Add DB script for `Prototype`
* Minor fixes

# v0.78.4.9

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.5) `v9.5`
* Update DB script for `Honkai: Star Rail`

# v0.78.4.8

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Fix lwrap WGET_ARGS

# v0.78.4.7

* Fix auto create shortcut for `Yandex Music`
* Add `QUIET_MODE` for `check_config_only()`

# v0.78.4.6

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.4) `v9.4`
* Update `wine-staging` (System) in container to `v9.6`
* Remove DB script for `Caliber`
* Remove DB script for `setup`
* Update DB script for `Need for Speed Underground`
* Update DB script for `Need for Speed Underground 2`
* Update DB script for `Need for Speed Most Wanted`
* Update DB script for `Need for Speed Carbon`
* Add install LW from container if couldn't download from the internet
* Fix install with `wget2`
* Fix install with `aria2` if user config exist
* Add `Controllermap` to `Settings`
* Add `SDL_GAMECONTROLLERCONFIG` to `Settings`
* Add `Gamescope` to `Settings`

# v0.78.4.5

* HotFix menu apps LW categories icon
* Fix custom `GTK theme` for init progress bar

# v0.78.4.4

* Change applications menu shortcuts
* Remove `Don't launch tray` option from `Settings`
* Add `Add tray to autostart` option to`Settings`
* Fix custom `GTK theme` for some LW windows
* Minor fixes

# v0.78.4.3

* Change applications menu shortcuts (it is divided into categories and added to the RMB)

# v0.78.4.2

* Fix mkdir steam compat dir

# v0.78.4.1

* Fix getting icon from dll when creating shortcut from `.lnk` and `.url` files
* Add DB script for `TimeShift`

# v0.78.3.9

* Update [obs-vkcapture-lw](https://github.com/VHSgunzo/obs-vkcapture-lw/releases/tag/v1.5.0) `v1.5.0`
* Remove `EXE_LWDBFL` from the EXE installer config when automatically creating shortcuts
* Update DB script for `Honkai: Star Rail`
* Update DB script for `Setup`

# v0.78.3.8

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Remove possibility of updating from the old version (`LutrisWine`) from `lwrap`
* Replace `chromium` with `firefox` in `lwrun` container
* Remove DB script for `Assassin's Creed Odyssey`
* Add the `Wine` prefix name to the shortcut name if a shortcut with the same name already exists and the app used custom `Wine` prefix
* Add `Yandex Cloud` mirrors
* Update `lwrap`
* Minor fixes

# v0.78.3.7

* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.2) `v9.2`
* Now you can manually drop `Proton's` "as is" in `~/LuxWine/runners/wine/`
* Add old Wine's/Proton's versions to `Wine manager`

# v0.78.3.6

* Fix `Remove shortcut` from App Settings
* Add `application/x-ms-shortcut` MimeType for `Remove shortcut` desktop
* Fix `Remove shortcut` for `.lnk` files
* Minor fixes

# v0.78.3.5

* HotFix don't shift `lu_rmapp()` args
* Update `lwrap` (installing `LuxWineService.desktop`)

# v0.78.3.4

* Add a submenu for application settings to [lw-tray](https://github.com/VHSgunzo/lw-tray/releases/tag/v0.0.6) `v0.0.6`
* HotFix `-appcfg` EXEs backslash args

# v0.78.3.3

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v9.1) `v9.1`
* Update [lw-runtime](https://github.com/VHSgunzo/lw-runtime/releases/tag/v0.0.3) `v0.0.3`
* Fix showing notify
* Update DB script for `Infernal`
* Update DB script for `Assassin's Creed Odyssey`
* Disable check default Wine for some arguments
* Fix run Debug if in EXEs path exist single quotes
* Add editable for `Wine version` field in settings
* Add question for restore from default prefix backup if custom Wine prefix doesn't exist
* Add run `console` EXEs in terminal
* Fix `Wine manager` lists
* Add `GTK theme` selector to settings (from $HOME/.themes)
* Add `WINE_CPU_TOPOLOGY` for `CPU limit`
* Add open apps settings with `-appcfg` arg
* Add linking for Steam Proton's `9.0 (Beta)` `Hotfix`
* Add linking Proton's GE from LW `Wine manager` to `Steam` ($HOME/.local/share/Steam/compatibilitytools.d)
* Update `README.md`
* Replace `firefox` with `chromium` in `lwrun` container
* Minor fixes

# v0.78.3.2

* Fix exit error on install init
* Add DB script for `Lineage 2 Warland`
* Disable `GameMode` by default
* Minor fixes

# v0.78.3.1

* HotFix wait for `lwrun` container processes exit
* Add automatic shortcut creation for `Winetricks`
* Add DB script for `Bio F.R.E.A.K.S.`
* Minor fixes

# v0.78.2.9

* Add `[EXE_NAMES].lwdb` DB script with custom EXE names
* Fix set `WINE_VERSION` from DB scripts for `System` Wine
* Fix wait for `lwrun` container processes exit
* Add check for duplicate exe arguments
* Minor fixes

# v0.78.2.8

* Add pseudo CD-ROM for fix old games CD-ROM check (`Road Rash` and others)
* Add `installrun_nupkg_exe()` for install and run exe with `*.nupkg` in DB scripts
* Update DB script for `Direct Commander`
* Update DB script for `WeMod`
* Add DB script for `EVE Online Setup`
* Add `Auto remove apps shortcuts` option to LW `Settings`
* Minor fixes

# v0.78.2.7

* Add app and EXE names to all DB scripts
* Update automatic shortcuts creation
* Fix manual shortcut creation from *.url file
* Fix manual shortcut removing for apps with a partial of the same name
* Fix passing arguments for EXE
* Fix launch GOG `Blade Runner` and other games with `ScummVM`
* Add DB script for `Syberia`
* Add DB script for `Infernal`
* Add DB script for `Abe's Oddysee`
* Add DB script for `upc.exe` from `Ubisoft Connect`
* Update DB script for `Crashday`
* Minor fixes

# v0.78.2.6

* Add DB script for `Caliber`
* Add prefix `syswinepfx` to DB scripts for games that runs on `System` wine
* Update automatic shortcuts creation

# v0.78.2.5

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.32) `v8.32`
* Update [glibc-eac](https://github.com/VHSgunzo/glibc-eac-rc/releases/tag/v2.39.2) `v2.39.2`
* Update [obs-vkcapture-lw](https://github.com/VHSgunzo/obs-vkcapture-lw/releases/tag/v1.4.9) `v1.4.9`
* Move `lwrun-pkg` to [lwrun](https://github.com/VHSgunzo/lwrun/tree/main/lwrun-pkg)
* Add `init` after update RunImage packages
* Add `xorg-xlsfonts` to `lwrap` depends
* Disable by default `Reduce PulseAudio latency`
* Change default MangoHud options
* Change default `MUST_HAVE_DLL` to `Disabled`
* Remove check free space
* Add DB script for `Anno 1800`
* Add DB script for `Assassin's Creed Odyssey`
* Add DB script for `Soldier of Fortune 2`
* Add DB script for `Direct Commander`
* Update DB script for `Zona`
* Update DB script for `The Honkers Railway` `v2.0.0`
* Update DB script for `WeMod`

# v0.78.2.4

* Update [steam-runtime-libs](https://github.com/VHSgunzo/steam-runtime-libs/releases/tag/v0.0.4) `v0.0.4`
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.28) `v8.28`
* Fix loading bar cover
* Add duplicate application installation config for automatically created shortcuts
* Fix creation of shortcuts
* Minor fixes

# v0.78.2.3

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update `lwrap`
* Update [lw-runtime](https://github.com/VHSgunzo/lw-runtime/releases/tag/v0.0.2) `v0.0.2`
* Reduce the size of the `lwrun` installer
* Revert/replace `mesa-tkg-git` with `mesa` in `lwrun`
* Fix prefix restore on first run with `-init`

# v0.78.2.2

* Add ability to create config for `msi`, `reg`, `bat` and `lnk`
* Update DB script for `The Honkers Railway` `v1.6.0`
* Update DB script for `Zona`
* Replace all GUI on `Zenity` with `YAD`
* Downgrade `Zenity` to `v3.44.2` in `lwrun`
* Temporary replace `mesa` with `mesa-tkg-git` in `lwrun` (mesa v23.3.2 is broken)
* Minor fixes

# v0.78.2.1

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [obs-vkcapture-lw](https://github.com/VHSgunzo/obs-vkcapture-lw/releases/tag/v1.4.7) `v1.4.7`
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.25) `v8.25`
* Speedup `lite_init()`
* Add DB script for `Battle.net`
* Update DB script for `League of Legends`
* Update lwrap `PKGBUILD`
* Fix `dll_manager()` for set old local `Wine` runtime DLLs
* Fix open `Settings` with different versions of `Wine` in the same `prefix`

# v0.78.1.9

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update DB script for `The Honkers Railway` `v1.5.0`
* Fix passing environment variables for `lwrap` - `lwexec`
* Add ability exec command as args in SHELL mode from terminal

# v0.78.1.8

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Replace `mhfsize` with [moninfo](https://github.com/VHSgunzo/moninfo) for automatic `MangoHud` font size detection
* Changed the way `GL SHADER` cache is stored
* Fix move `vkd3d-proton.cache`
* Add `RU` translation to [lw-tray](https://github.com/VHSgunzo/lw-tray/releases/tag/v0.0.5) `v0.0.5`

# v0.78.1.7

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.23) `v8.23`
* Update [obs-vkcapture-lw](https://github.com/VHSgunzo/obs-vkcapture-lw/releases/tag/v1.4.5) `v1.4.5`
* Add ([mhfsize](https://github.com/VHSgunzo/lux-wine/blob/main/lwrap/mhfsize)) for automatic `MangoHud` font size detection
* Add `MANGOHUD_FONT_SIZE` env var
* Add `MangoHud config` tab to `Settings`
* Changed the way `VKD3D` cache is stored
* Changed the way `Mesa` cache is stored
* Minor fixes

# v0.78.1.6

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.22) `v8.22`
* Add `gstreamer-vaapi` to `lwrap` depends

# v0.78.1.5

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.20) `v8.20`
* Add `egl-wayland` to `lwrap` depends
* Remove DB script for `World of Warships`

# v0.78.1.4

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Create [obs-vkcapture-lw](https://github.com/VHSgunzo/obs-vkcapture-lw) package
* Add [OBS-VkCapture](https://github.com/nowrep/obs-vkcapture) supports
* Minor fixes

# v0.78.1.3

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.19) `v8.19`
* Update `HF mirror`
* Add DB script for `Gothic 4: Arcania`
* Update DB script for `The Honkers Railway` `v1.4.0`
* Update DB script for `World of Warships`
* Fix add `$HOME/.local/bin` to `PATH`
* Add [installation video](https://www.youtube.com/watch?v=a0vrAgY2uZk)
* Update [glibc-eac-rc](https://github.com/VHSgunzo/glibc-eac-rc/releases/tag/v2.38-8) `v2.38-8`

# v0.78.1.2

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Force use msiexec for MSI install
* Fix symlinks for Proton libvkd3d-1 libvkd3d-shader-1 dlls
* Minor fixes

# v0.78.1.1

* Add MSI install with wine-staging (hotfix)

# v0.77.9.9

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Add `Enabled` FSR mode without preset (fix dx9 bug)
* Update DB script for `Command and Conquer Red Alert 3`
* Update DB script for `Command and Conquer Red Alert 3 Uprising`
* Update DB script for `Heritage of Kings - The Settlers`
* Update DB script for `League of Legends`
* Update [wine-prefix](https://github.com/VHSgunzo/wine-prefix/releases/tag/v0.0.8) `v0.0.8`
* Minor fixes

# v0.77.9.8

* Add DB script for `Heritage of Kings - The Settlers`
* Add DB script for `Command and Conquer Red Alert 3 Uprising`
* Update DB script for setup's
* Fix auto-creation of shortcuts for apps and games (now case insensitive)

# v0.77.9.7

* Update [lwrun](https://huggingface.co/lux-wine/lwrun/tree/main/releases/v0.39.1) packages
* Add DB script for `Wolfenstein 2009`
* Add DB script for `World in Conflict`
* Add DB script for `Command and Conquer - Red Alert 3`
* Update DB script for `WeMod` (now installation from `WeMod` setup EXE is also possible)
* Remove old DB scripts
* Fix fake Windows version in wine registry
* Update [wine-prefix](https://github.com/VHSgunzo/wine-prefix/releases/tag/v0.0.7) `v0.0.7`
* Update README.md

# v0.77.9.6

* Update [lwrun](https://github.com/VHSgunzo/lwrun) packages
* Update lwrap `PKGBUILD`
* Add time to complete all background processes to properly close the container
* Update [wine-prefix](https://github.com/VHSgunzo/wine-prefix/releases/tag/v0.0.6) `v0.0.6`
* Revert [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw) to `lwrun` container
* Update [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.16) `v8.16`
* Changed `FSR mode` to `balanced` by default (fix screen position and the lights for old games on Proton GE)
* Updated lists of must have libraries for prefix
* Add installing the Wine specified in the LW config or DB scripts, if it is not installed
* Add request to restore the prefix from the backup of the default prefix, when changing the path to the prefix in the LW settings
* Refactoring `lu_winemgr()`
* Add the ability to disable the Wine theme changer
* Add using `gstreamer` from the container in its absence in the custom `Wine/Proton`
* Changed the use of DB scripts for applications and games
* Update old DB scripts
* Disable DB script for `The Last Of Us Part I`
* Add DB script for `Emperor: Battle for Dune`
* Add DB script for `Epic Games Launcher`
* Add DB script for `League of Legends`
* Add DB script for `Mass Effect`
* Add DB script for `Need for Speed Porsche`
* Add DB script for `WeMod`
* Add DB script for `World Of Sea Battle`
* Add DB script for `The Honkers Railway`
* Add DB script for `Lesta Game Center`
* Add DB script for `Wargaming Game Center`
* Add DB script for `GOG Galaxy installer`
* Add DB script for `osu!`
* Add DB script for `Need for Speed Underground`
* Add DB script for `Need for Speed Underground 2`
* Add DB script for `Need for Speed Most Wanted`
* Add DB script for `Need for Speed Carbon`
* Update DB script for `Crossout`
* Add DB script for setup's and installer's EXE
* Remove Steam icon (fix get it from Steam exe)
* Minor fixes

# v0.77.9.1

* Change project name to Lux Wine
* Update [RunImage](https://github.com/VHSgunzo/runimage) container to [v0.39.1](https://github.com/VHSgunzo/runimage/releases/tag/v0.39.1) for [lwrun](https://github.com/VHSgunzo/lwrun)
* Create [wine-prefix](https://github.com/VHSgunzo/wine-prefix) package
* Create [steam-runtime-libs](https://github.com/VHSgunzo/steam-runtime-libs) package
* Create EasyAntiCheat patched [glibc-eac-rc](https://github.com/VHSgunzo/glibc-eac-rc) (2.38-3) for Rogue Company
* Create Reshade Shaders [reshade-shaders-lw](https://github.com/VHSgunzo/reshade-shaders-lw) package
* Replace palemoon with firefox
* Remove mangoapp and lib32-mangoapp
* Replace mangohud-lw-git with mangohud lib32-mangohud
* Update [hosts](https://github.com/StevenBlack/hosts)
* Fix LatencyFlex and cabextract in [GE-Proton](https://github.com/VHSgunzo/ge-proton-lw)
* LW tray moved to a separate [repository](https://github.com/VHSgunzo/lw-tray)
* Create [lw-tray](https://github.com/VHSgunzo/lw-tray) package
* The path to the EXE is now case insensitive
* Replace pluma with mousepad
* Replace icoutils with icoextract
* Add Steam icon
* Change default LW icon
* Change default LW loading gif
* Remove "Add app to Lutris"
* Disable by default DXR/RTX (and now it's one checkbox)
* Disable by default ESYNC
* Revert MangoHud DLSYM mode
* Remove some notify
* Fix zenity icon
* Add timeout to try_shutdown_wine() and kill wine
* Add automatic creation/removal of shortcuts for installed programs and games
* Add automatic creation of shortcuts from game launchers
* Add manual mode of creating shortcuts from *.lnk and *.url files
* Fix removing shortcuts in manual mode
* Fix disabling the wine native theme
* Fix creating shortcuts in manual mode
* Add VKD3D_SHADER_DEBUG="none" for regular launch
* Fix displaying the wine version in MangoHud
* Fix AUTO mode for Vulkan ICD loader
* Remove __GL_THREADED_OPTIMIZATIONS from Settings
* Add automatic assign drive letters for wine when mounting partitions at standard mounting points
* Add H:\ drive for $HOME (together with the upper one, it corrects pseudo-lack of space errors)
* Fix fonts links for wine prefix from protons
* Add steam steam-runtime steam-native links to ~/.local/bin/
* Add lwrap link to ~/.local/bin/
* Fix add ~/.local/bin to PATH
* Add launching all EXEs in one container instance by lwrap wrapper (fixes a lot of problems and speeds up the startup)
* Add assigning mime types to LuxWine.desktop during installation/update
* Add force lwrap update when installing LW
* Add a completely offline installation mode
* Add shutdown request after LW update
* Add [huggingface](https://huggingface.co/spaces/lux-wine) mirror for LW installation
* Remove GDrive mirror for download lwrun
* Revert packages update for lwrun
* Add lwrun packages update from builtin terminal (if the update is not launched from the terminal)
* Add migration from old LW
* Remove [Proton GE](https://github.com/GloriousEggroll/proton-ge-custom)
* Add DB script for Ubisoft Connect
* Add DB script for Dark Sector
* Add DB script for Crossout
* Add DB script for Remnant 2
* Add DB script for Serious Sam 4
* Add DB script for Starfield
* Add DB script for xrEngine (S.T.A.L.K.E.R)
* Add DB script for Crashday
* Add DB script for Crysis
* Update DB script for Zona
* Minor fixes
