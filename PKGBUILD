# Maintainer: VHSgunzo <vhsgunzo.github.io>
pkgname=lutris-wine-git
pkgver=0.37.r0.ge461a2d
pkgrel=1
pkgdesc='Lutris Wine Runner as system Wine and even better'
arch=('any')
url='https://github.com/VHSgunzo/lutris-wine'
license=('MIT')
depends=('lutris-git' 'lutris-wine-meta' 'plasma-desktop')
makedepends=('git')
#checkdepends=('')
#optdepends=('')
provides=("${pkgname%-git}")
conflicts=("${pkgname%-git}")
source=('git+https://github.com/VHSgunzo/lutris-wine.git')
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/${pkgname%-git}"
  git describe --long --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

package() {
    cd $srcdir/${pkgname%-git}
    install -Dm755 usr/bin/lutris-wine ${pkgdir}/usr/bin/lutris-wine
    install -Dm644 etc/xdg/menus/applications-merged/LutrisWine.menu ${pkgdir}/etc/xdg/menus/applications-merged/LutrisWine.menu
    install -Dm644 usr/share/applications/LutrisWine/clearpfx.desktop ${pkgdir}/usr/share/applications/LutrisWine/clearpfx.desktop
    install -Dm644 usr/share/applications/LutrisWine/cmd.desktop ${pkgdir}/usr/share/applications/LutrisWine/cmd.desktop
    install -Dm644 usr/share/applications/LutrisWine/control.desktop ${pkgdir}/usr/share/applications/LutrisWine/control.desktop
    install -Dm644 usr/share/applications/LutrisWine/debug.desktop ${pkgdir}/usr/share/applications/LutrisWine/debug.desktop
    install -Dm644 usr/share/applications/LutrisWine/explorer.desktop ${pkgdir}/usr/share/applications/LutrisWine/explorer.desktop
    install -Dm644 usr/share/applications/LutrisWine/killwine.desktop ${pkgdir}/usr/share/applications/LutrisWine/killwine.desktop
    install -Dm644 usr/share/applications/LutrisWine/LutrisWine.desktop ${pkgdir}/usr/share/applications/LutrisWine/LutrisWine.desktop
    install -Dm644 usr/share/applications/LutrisWine/openpfx.desktop ${pkgdir}/usr/share/applications/LutrisWine/openpfx.desktop
    install -Dm644 usr/share/applications/LutrisWine/regedit.desktop ${pkgdir}/usr/share/applications/LutrisWine/regedit.desktop
    install -Dm644 usr/share/applications/LutrisWine/rmapp.desktop ${pkgdir}/usr/share/applications/LutrisWine/rmapp.desktop
    install -Dm644 usr/share/applications/LutrisWine/shell.desktop ${pkgdir}/usr/share/applications/LutrisWine/shell.desktop
    install -Dm644 usr/share/applications/LutrisWine/taskmgr.desktop ${pkgdir}/usr/share/applications/LutrisWine/taskmgr.desktop
    install -Dm644 usr/share/applications/LutrisWine/uninstaller.desktop ${pkgdir}/usr/share/applications/LutrisWine/uninstaller.desktop
    install -Dm644 usr/share/applications/LutrisWine/winecfg.desktop ${pkgdir}/usr/share/applications/LutrisWine/winecfg.desktop
    install -Dm644 usr/share/applications/LutrisWine/winetricks.desktop ${pkgdir}/usr/share/applications/LutrisWine/winetricks.desktop
    install -Dm644 usr/share/desktop-directories/LutrisWineApp.directory ${pkgdir}/usr/share/desktop-directories/LutrisWineApp.directory
    install -Dm644 usr/share/desktop-directories/LutrisWine.directory ${pkgdir}/usr/share/desktop-directories/LutrisWine.directory
    install -Dm644 usr/share/kservices5/ServiceMenus/LutrisWineService.desktop ${pkgdir}/usr/share/kservices5/ServiceMenus/LutrisWineService.desktop
    install -Dm644 LICENSE ${pkgdir}/usr/share/licenses/${pkgname}/LICENSE
}
