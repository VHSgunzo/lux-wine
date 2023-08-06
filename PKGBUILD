# Maintainer: VHSgunzo <vhsgunzo.github.io>

pkgname='lwrap'
pkgver='0.77.9'
pkgrel='1'
pkgdesc='Lux Wine wrapper for RunImage container'
arch=('x86_64')
url='https://github.com/VHSgunzo/lux-wine'
license=('MIT')
depends=('aria2' 'qt5-tools' 'xterm' 'xorg-fonts-100dpi' 'xorg-fonts-75dpi' 'firefox' 'intel-media-driver'
         'xorg-fonts-cyrillic' 'xorg-fonts-type1' 'xorg-fonts-misc' 'cabextract' 'exo' 'xapp' 'pulseaudio'
         'perl' 'unzip' 'xorg-xmessage' 'zenity' 'yad' 'xkb-switch' 'imagemagick' 'qt5ct' 'syslog-ng'
         'icoutils' 'lib32-vkbasalt' 'mangohud' 'lib32-mangohud' 'latencyflex-wine-git' 'steam'
         'vkbasalt' 'gamemode' 'lib32-gamemode' 'giflib' 'lib32-giflib' 'libpng' 'appmenu-gtk-module'
         'lib32-libpng' 'libldap' 'lib32-libldap' 'gnutls' 'lib32-gnutls' 'bash' 'gawk' 'steam-native-runtime'
         'mpg123' 'lib32-mpg123' 'openal' 'lib32-openal' 'v4l-utils' 'lib32-v4l-utils' 'qt5-styleplugins'
         'libpulse' 'lib32-libpulse' 'libgpg-error' 'lib32-libgpg-error' 'grep' 'sed' 'nano' 'wine-nine'
         'alsa-plugins' 'lib32-alsa-plugins' 'alsa-lib' 'lib32-alsa-lib' 'wine-prefix' 'wget' 'lw-tray'
         'libjpeg-turbo' 'lib32-libjpeg-turbo' 'sqlite' 'lib32-sqlite' 'libxcomposite' 'alsa-utils'
         'lib32-libxcomposite' 'libxinerama' 'lib32-libgcrypt' 'libgcrypt' 'mousepad' 'btop'
         'lib32-libxinerama' 'ncurses' 'lib32-ncurses' 'spacefm' 'nvtop' 'libxslt' 'lib32-libxslt'
         'gtk3' 'lib32-gtk3' 'lib32-vulkan-icd-loader' 'vulkan-headers' 'vulkan-icd-loader' 'libva'
         'vulkan-tools' 'mesa' 'lib32-glu' 'glu' 'lib32-libva-mesa-driver' 'lib32-mesa' 'lib32-libva'
         'libva-mesa-driver' 'mesa-demos' 'mesa-utils' 'mesa-vdpau' 'vulkan-mesa-layers' 'steam-runtime-libs'
         'lib32-vulkan-mesa-layers' 'libstrangle-git' 'openssl' 'libnotify' 'coreutils' 'xdotool'
         'libxrandr' 'lib32-libxrandr' 'xorg-xrandr' 'bc' 'xorg-xgamma' 'lsb-release' 'neofetch'
         'iputils' 'curl' 'xdelta3' 'xdg-utils' 'lsvkdev' 'wmctrl' 'importenv' 'xdg-desktop-portal-gtk'
         'desktop-file-utils' 'squashfs-tools' 'squashfuse' 'fuse2' 'reshade-shaders-lw' 'glibc-eac'
         'winetricks-git' 'innoextract' 'p7zip' 'gnu-netcat' 'xdg-user-dirs' 'GE-Proton' 'dbus-x11'
         'libva-utils' 'libva-intel-driver' 'lib32-libva-intel-driver' 'vulkan-intel' 'lib32-glibc-eac'
         'lib32-vulkan-intel' 'vulkan-radeon' 'lib32-vulkan-radeon' 'lib32-libxnvctrl' 'libxnvctrl'
)
optdepends=('xf86-video-amdgpu' 'xf86-video-intel' 'llvm'
            'lib32-opencl-icd-loader' 'opencl-icd-loader'
            'lib32-llvm' 'wine' 'wine-prefix-dotnet')
source=('lwrap' 'lux-wine')
sha256sums=('SKIP' 'SKIP')

shopt -s extglob

package() {
    install -dm755 "$pkgdir/usr/bin"
    install -Dm755 "$pkgname" "$pkgdir/opt/$pkgname/bin/$pkgname"
    install -Dm755 "lux-wine" "$pkgdir/opt/$pkgname/bin/lux-wine"
    cp -ar --no-preserve=ownership "$(realpath ../)"/!(screenshots|.git*|${pkgname}*|lux-wine|pkg|src) "$pkgdir/opt/$pkgname"
    ln -sfr "$pkgdir/opt/$pkgname/bin/$pkgname" "$pkgdir/usr/bin/$pkgname"
    mkdir -p "$pkgdir/usr"/{lib,lib32}
    ln -sfr "$pkgdir/usr/lib/syslog-ng/libcef.so" "$pkgdir/usr/lib/libcef.so"
    ln -sfr "$pkgdir/usr/lib32/libva.so" "$pkgdir/usr/lib32/libva.so.1"
}
