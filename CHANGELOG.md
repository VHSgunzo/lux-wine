# v0.77.9.1

* Change project name to Lux Wine
* Update [RunImage](https://github.com/VHSgunzo/runimage) container to [v0.39.1](https://github.com/VHSgunzo/runimage/releases/tag/v0.39.1) for [lwrun](https://github.com/VHSgunzo/lwrun)
* Create [wine-prefix](https://github.com/VHSgunzo/wine-prefix) package
* Create [steam-runtime-libs](https://github.com/VHSgunzo/steam-runtime-libs) package
* Create EasyAntiCheat patched [glibc-eac-rc](https://github.com/VHSgunzo/glibc-eac-rc) (2.38-3) for Rogue Company
* Create Reshade Shaders [reshade-shaders-lw](https://github.com/VHSgunzo/reshade-shaders-lw) package
* Update [GE-Proton v8-15](https://github.com/VHSgunzo/ge-proton-lw/releases/tag/v8.15)
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
* Minor fixes
