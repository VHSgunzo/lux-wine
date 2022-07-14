# lutris-wine - Easy launch of your Windows applications and games with Wine/Proton

VERSION := 0.74.r0

INSTALL	:= install
RM	:= rm
LS	:= ls

install:
	$(INSTALL) -Dm755 lutris-wine "$(DESTDIR)/usr/bin/lutris-wine"
	$(INSTALL) -Dm644 LutrisWine.menu "$(DESTDIR)/etc/xdg/menus/applications-merged/LutrisWine.menu"
	$(INSTALL) -Dm644 LutrisWineService.desktop "$(DESTDIR)/usr/share/kservices5/ServiceMenus/LutrisWineService.desktop"
	$(INSTALL) -Dm644 LICENSE "$(DESTDIR)/usr/share/licenses/lutris-wine/LICENSE"
	(IFS=$$'\n' ; for APP in $$($(LS) -1 applications) ; do \
	$(INSTALL) -Dm644 "applications/$$APP" "$(DESTDIR)/usr/share/applications/LutrisWine/$$APP" ; \
	done)
	(IFS=$$'\n' ; for DIR in $$($(LS) -1 directories) ; do \
	$(INSTALL) -Dm644 "directories/$$DIR" "$(DESTDIR)/usr/share/desktop-directories/$$DIR" ; \
	done)
	(IFS=$$'\n' ; for ICON in $$($(LS) -1 icons) ; do \
	$(INSTALL) -Dm644 "icons/$$ICON" "$(DESTDIR)/usr/share/lutris-wine/icons/$$ICON" ; \
	done)
	(IFS=$$'\n' ; for GIF in $$($(LS) -1 gif) ; do \
	$(INSTALL) -Dm644 "gif/$$GIF" "$(DESTDIR)/usr/share/lutris-wine/gif/$$GIF" ; \
	done)
	(IFS=$$'\n' ; for THEME in $$($(LS) -1 themes) ; do \
	$(INSTALL) -Dm644 "themes/$$THEME" "$(DESTDIR)/usr/share/lutris-wine/themes/$$THEME" ; \
	done)
	(IFS=$$'\n' ; for DB in $$($(LS) -1 db) ; do \
	$(INSTALL) -Dm644 "db/$$DB" "$(DESTDIR)/usr/share/lutris-wine/db/$$DB" ; \
	done)

uninstall:
	$(RM) -rfv "$(DESTDIR)/usr/bin/lutris-wine"
	$(RM) -rfv "$(DESTDIR)/etc/xdg/menus/applications-merged/LutrisWine.menu"
	$(RM) -rfv "$(DESTDIR)/usr/share/applications/LutrisWine"
	$(RM) -rfv "$(DESTDIR)/usr/share/desktop-directories/LutrisWine"*
	$(RM) -rfv "$(DESTDIR)/usr/share/kservices5/ServiceMenus/LutrisWine"*
	$(RM) -rfv "$(DESTDIR)/usr/share/lutris-wine /usr/share/licenses/lutris-wine"

.PHONY: install uninstall
