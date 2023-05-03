extern crate ksni;
extern crate execute;
use ksni::menu::*;
use std::{process::{exit, Child}, time::Duration, fs};
use execute::{shell, Execute};
use std::{env, process::Stdio};

fn get_env(var: &str) -> String {
    if let Ok(ret) = env::var(var) 
        { ret } else { "".to_string() }
}

fn shellspawn(command: &str) -> Child {
    shell(command).spawn()
        .expect("Shell command failed to start!")
}

fn shellexec(command: &str) -> String {
    String::from_utf8(
        shell(command)
            .stdout(Stdio::piped())
            .execute_output().unwrap().stdout
    ).unwrap()
}

fn lwexec(command: String) -> Child {
    shellspawn(format!("{} {}", get_env("LW_SOURCE"), command).as_str())
}

struct LwTray {
    icon: String,
    title: String,
    lw_apps: String
}

fn lw_activate(cmd: String) -> Box<dyn Fn(&mut LwTray) + 'static> {
    Box::new(move |_| { lwexec(cmd.to_string()); })
}

fn to_vec_string(argv: Vec<&str>) -> Vec<String> {
    argv.iter().map(|&s| s.to_string()).collect::<Vec<String>>()
}

impl ksni::Tray for LwTray {
    fn icon_name(&self) -> String {
        self.icon.clone().into()
    }
    fn title(&self) -> String {
        self.title.clone().into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {

        let mut apps_submenu = Vec::new();
        if ! self.lw_apps.is_empty() {
            for app in self.lw_apps.lines() {
                let app_split = to_vec_string(app.split(" ##&## ").collect());
                let name = app_split[0].clone();
                let icon = app_split[1].clone();
                apps_submenu.push(
                    StandardItem {
                        label: name.clone().into(),
                        icon_name: icon.clone().into(),
                        activate: lw_activate(format!("-runapp \"{}\"", name.clone())),
                        ..Default::default()
                    }.into()
                );
            };
        };

        vec![
            SubMenu {
                label: "Apps".into(),
                icon_name: self.icon.clone().into(),
                submenu: apps_submenu,
                visible: if self.lw_apps.is_empty() { false } else { true },
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Open".into(),
                icon_name: self.icon.clone().into(),
                visible: if get_env("LU_EXE").is_empty() { true } else { false },
                submenu: vec![
                    StandardItem {
                        label: "Lutris Wine".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "DEBUG".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-debug".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Wine".into(),
                icon_name: self.icon.clone().into(),
                submenu: vec![
                    StandardItem {
                        label: "Explorer".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-explorer".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Task manager".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-taskmgr".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "CMD".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-cmd".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Control panel".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-control".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Settings".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-winecfg".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Registry editor".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-regedit".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Uninstaller".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-uninstaller".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Kill".into(),
                icon_name: self.icon.clone().into(),
                submenu: vec![
                    StandardItem {
                        label: "EXE".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-killexe".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Wine".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-killwine".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "SHELL".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-killshell".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "ALL".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-exit".to_string()),
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Prefix".into(),
                icon_name: self.icon.clone().into(),
                submenu: vec![
                    StandardItem {
                        label: "Open drive C:\\".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-openpfx".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Backup".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-pfxbackup".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Restore".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-pfxrestore".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Clear".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-clearpfx".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Mount backup".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-backupmnt".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Unmount backup".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-backupunmnt".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: "Settings".into(),
                icon_name: self.icon.clone().into(),
                submenu: vec![
                    StandardItem {
                        label: self.title.clone().into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-config".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Apps settings".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-appcfg".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Wine manager".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-winemgr".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Runtime updater".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-update".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Winetricks".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-winetricks".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Forced init".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-init".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: "Open SHELL".into(),
                        icon_name: self.icon.clone().into(),
                        activate: lw_activate("-shell".to_string()),
                        ..Default::default()
                    }.into(),
                    SubMenu {
                        label: "Shortcut".into(),
                        icon_name: self.icon.clone().into(),
                        submenu: vec![
                            StandardItem {
                                label: "Create".into(),
                                icon_name: self.icon.clone().into(),
                                activate: lw_activate("-shortcut".to_string()),
                                ..Default::default()
                            }.into(),
                            StandardItem {
                                label: "Remove".into(),
                                icon_name: self.icon.clone().into(),
                                activate: lw_activate("-rmapp".to_string()),
                                ..Default::default()
                            }.into(),
                            StandardItem {
                                label: "Add to Lutris".into(),
                                icon_name: self.icon.clone().into(),
                                activate: lw_activate("-addtolu".to_string()),
                                visible: if get_env("SYS_LU").is_empty() { false } else { true },
                                ..Default::default()
                            }.into()
                        ],
                        ..Default::default()
                    }.into(),
                    SubMenu {
                        label: "Info".into(),
                        icon_name: self.icon.clone().into(),
                        submenu: vec![
                            StandardItem {
                                label: "Usage".into(),
                                icon_name: self.icon.clone().into(),
                                activate: lw_activate("-help".to_string()),
                                ..Default::default()
                            }.into(),
                            StandardItem {
                                label: "Version".into(),
                                icon_name: self.icon.clone().into(),
                                activate: lw_activate("-version".to_string()),
                                ..Default::default()
                            }.into()
                        ],
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| exit(0)),
                ..Default::default()
            }.into()
        ]
    }
    fn id(&self) -> String {
        "LwTray".to_string()
    }
}

fn get_lw_apps() -> String {
    shellexec(
        "grep '^Categories=Lutris Wine App' -lr ''$LW_APPS_DIR'' 2>/dev/null|\
                 xargs -I {{}} grep -m2 '^Name=\\|^Icon=' {{}} 2>/dev/null|sed 's|^Name=||g'|\
                 sed ':a;N;$!ba;s|\\nIcon=| ##\\&## |g'|sort -u"
    )
}

fn ls_lw_apps() -> Vec<String> {
    let mut apps = vec![];
    if let Ok(entries) = fs::read_dir(get_env("LW_APPS_DIR")) {
        for app in entries {
            apps.push(app.unwrap().file_name().to_str().unwrap().to_string())
        }
    }
    apps
}

fn main() {

    let lw_apps = get_lw_apps();

    let mut icon = get_env("LW_EXE_ICON");
    if icon.is_empty() {
        icon = get_env("LW_DEF_ICO").to_string()
    }

    let mut title = get_env("EXE_NAME");
    if title.is_empty() {
        title = "Lutris Wine".to_string()
    }

    let service = ksni::TrayService::new(LwTray {
        icon,
        title,
        lw_apps
    });
    let handle = service.handle();
    service.spawn();
    
    loop {
        let old_lw_apps = ls_lw_apps();
        std::thread::sleep(Duration::from_secs(1));
        let new_lw_apps = ls_lw_apps();
        if new_lw_apps != old_lw_apps {
            handle.update(|tray: &mut LwTray| {
                tray.lw_apps = get_lw_apps()
            });
        }
    }
}
