install:
	rm -rfv /usr/bin/lutris-wine /etc/xdg/menus/applications-merged/LutrisWine.menu /usr/share/applications/LutrisWine /usr/share/desktop-directories/LutrisWine* /usr/share/lutris-wine /usr/share/licenses/lutris-wine
	cp -rfv usr/bin/lutris-wine /usr/bin/lutris-wine
	cp -rfv etc/xdg/menus/applications-merged/LutrisWine.menu /etc/xdg/menus/applications-merged/LutrisWine.menu
	cp -rfv usr/share/applications/LutrisWine /usr/share/applications/
	cp -rfv usr/share/desktop-directories/LutrisWineApp.directory /usr/share/desktop-directories/LutrisWineApp.directory
	cp -rfv usr/share/desktop-directories/LutrisWine.directory /usr/share/desktop-directories/LutrisWine.directory
	cp -rfv usr/share/kservices5/ServiceMenus/LutrisWineService.desktop /usr/share/kservices5/ServiceMenus/LutrisWineService.desktop
	cp -rfv usr/share/lutris-wine /usr/share/
	cp -rfv LICENSE /usr/share/licenses/lutris-wine/LICENSE

uninstall:
	rm -rfv /usr/bin/lutris-wine /etc/xdg/menus/applications-merged/LutrisWine.menu /usr/share/applications/LutrisWine /usr/share/desktop-directories/LutrisWine* /usr/share/lutris-wine /usr/share/licenses/lutris-wine

.PHONY: install uninstall
