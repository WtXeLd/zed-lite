use gpui::{App, Menu, MenuItem, OsAction};
use localization::t as tr;
use terminal_view::terminal_panel;

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    use zed_actions::Quit;

    let mut view_items = vec![
        MenuItem::action(
            tr(cx, "Zoom In"),
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            tr(cx, "Zoom Out"),
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            tr(cx, "Reset Zoom"),
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            tr(cx, "Reset All Zoom"),
            zed_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action(tr(cx, "Toggle Left Dock"), workspace::ToggleLeftDock),
        MenuItem::action(tr(cx, "Toggle Right Dock"), workspace::ToggleRightDock),
        MenuItem::action(tr(cx, "Toggle Bottom Dock"), workspace::ToggleBottomDock),
        MenuItem::action(tr(cx, "Toggle All Docks"), workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: tr(cx, "Editor Layout"),
            disabled: false,
            items: vec![
                MenuItem::action(tr(cx, "Split Up"), workspace::SplitUp::default()),
                MenuItem::action(tr(cx, "Split Down"), workspace::SplitDown::default()),
                MenuItem::action(tr(cx, "Split Left"), workspace::SplitLeft::default()),
                MenuItem::action(tr(cx, "Split Right"), workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action(
            tr(cx, "Project Panel"),
            zed_actions::project_panel::ToggleFocus,
        ),
        MenuItem::action(tr(cx, "Outline Panel"), outline_panel::ToggleFocus),
        MenuItem::action(tr(cx, "Terminal Panel"), terminal_panel::ToggleFocus),
    ];

    view_items.push(MenuItem::separator());
    view_items.push(MenuItem::action(tr(cx, "Diagnostics"), diagnostics::Deploy));
    view_items.push(MenuItem::separator());

    let mut zed_items = vec![MenuItem::action(tr(cx, "About Zed"), zed_actions::About)];
    zed_items.extend([
        MenuItem::submenu(Menu::new(tr(cx, "Settings")).items([
            MenuItem::action(tr(cx, "Open Settings"), zed_actions::OpenSettings),
            MenuItem::action(tr(cx, "Open Settings File"), super::OpenSettingsFile),
            MenuItem::action(
                tr(cx, "Open Project Settings"),
                zed_actions::OpenProjectSettings,
            ),
            MenuItem::action(
                tr(cx, "Open Project Settings File"),
                super::OpenProjectSettingsFile,
            ),
            MenuItem::action(tr(cx, "Open Default Settings"), super::OpenDefaultSettings),
            MenuItem::separator(),
            MenuItem::action(tr(cx, "Open Keymap"), zed_actions::OpenKeymap),
            MenuItem::action(tr(cx, "Open Keymap File"), zed_actions::OpenKeymapFile),
            MenuItem::action(
                tr(cx, "Open Default Key Bindings"),
                zed_actions::OpenDefaultKeymap,
            ),
            MenuItem::separator(),
            MenuItem::action(
                tr(cx, "Select Theme..."),
                zed_actions::theme_selector::Toggle::default(),
            ),
            MenuItem::action(
                tr(cx, "Select Icon Theme..."),
                zed_actions::icon_theme_selector::Toggle::default(),
            ),
        ])),
        MenuItem::separator(),
    ]);
    #[cfg(target_os = "macos")]
    zed_items.push(MenuItem::os_submenu(
        tr(cx, "Services"),
        gpui::SystemMenuType::Services,
    ));
    zed_items.push(MenuItem::separator());
    zed_items.push(MenuItem::action(
        tr(cx, "Extensions"),
        zed_actions::Extensions::default(),
    ));
    #[cfg(not(target_os = "windows"))]
    zed_items.push(MenuItem::action(
        tr(cx, "Install CLI"),
        install_cli::InstallCliBinary,
    ));
    zed_items.push(MenuItem::separator());
    #[cfg(target_os = "macos")]
    zed_items.push(MenuItem::action(tr(cx, "Hide Zed"), super::Hide));
    #[cfg(target_os = "macos")]
    zed_items.push(MenuItem::action(tr(cx, "Hide Others"), super::HideOthers));
    #[cfg(target_os = "macos")]
    zed_items.push(MenuItem::action(tr(cx, "Show All"), super::ShowAll));
    zed_items.push(MenuItem::separator());
    zed_items.push(MenuItem::action(tr(cx, "Quit Zed"), Quit));

    let mut file_items = vec![
        MenuItem::action(tr(cx, "New"), workspace::NewFile),
        MenuItem::action(tr(cx, "New Window"), workspace::NewWindow),
        MenuItem::separator(),
    ];
    #[cfg(not(target_os = "macos"))]
    file_items.push(MenuItem::action(
        tr(cx, "Open File..."),
        workspace::OpenFiles,
    ));
    file_items.extend([
        MenuItem::action(
            tr(
                cx,
                if cfg!(not(target_os = "macos")) {
                    "Open Folder..."
                } else {
                    "Open…"
                },
            ),
            workspace::Open::default(),
        ),
        MenuItem::action(
            tr(cx, "Open Recent..."),
            zed_actions::OpenRecent {
                create_new_window: false,
            },
        ),
    ]);
    file_items.extend([
        MenuItem::separator(),
        MenuItem::action(
            tr(cx, "Add Folder to Project…"),
            workspace::AddFolderToProject,
        ),
        MenuItem::separator(),
        MenuItem::action(tr(cx, "Save"), workspace::Save { save_intent: None }),
        MenuItem::action(tr(cx, "Save As…"), workspace::SaveAs),
        MenuItem::action(tr(cx, "Save All"), workspace::SaveAll { save_intent: None }),
        MenuItem::separator(),
        MenuItem::action(
            tr(cx, "Close Editor"),
            workspace::CloseActiveItem {
                save_intent: None,
                close_pinned: true,
            },
        ),
        MenuItem::action(tr(cx, "Close Project"), workspace::CloseProject),
        MenuItem::action(tr(cx, "Close Window"), workspace::CloseWindow),
    ]);

    vec![
        Menu {
            name: tr(cx, "Zed"),
            disabled: false,
            items: zed_items,
        },
        Menu {
            name: tr(cx, "File"),
            disabled: false,
            items: file_items,
        },
        Menu {
            name: tr(cx, "Edit"),
            disabled: false,
            items: vec![
                MenuItem::os_action(tr(cx, "Undo"), editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action(tr(cx, "Redo"), editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action(tr(cx, "Cut"), editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action(tr(cx, "Copy"), editor::actions::Copy, OsAction::Copy),
                MenuItem::action(tr(cx, "Copy and Trim"), editor::actions::CopyAndTrim),
                MenuItem::os_action(tr(cx, "Paste"), editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action(tr(cx, "Find"), search::buffer_search::Deploy::find()),
                MenuItem::action(
                    tr(cx, "Find in Project"),
                    workspace::DeploySearch::default(),
                ),
                MenuItem::separator(),
                MenuItem::action(
                    tr(cx, "Toggle Line Comment"),
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: tr(cx, "Selection"),
            disabled: false,
            items: vec![
                MenuItem::os_action(
                    tr(cx, "Select All"),
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action(
                    tr(cx, "Expand Selection"),
                    editor::actions::SelectLargerSyntaxNode,
                ),
                MenuItem::action(
                    tr(cx, "Shrink Selection"),
                    editor::actions::SelectSmallerSyntaxNode,
                ),
                MenuItem::action(
                    tr(cx, "Select Next Sibling"),
                    editor::actions::SelectNextSyntaxNode,
                ),
                MenuItem::action(
                    tr(cx, "Select Previous Sibling"),
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    tr(cx, "Add Cursor Above"),
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    tr(cx, "Add Cursor Below"),
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    tr(cx, "Select Next Occurrence"),
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    tr(cx, "Select Previous Occurrence"),
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    tr(cx, "Select All Occurrences"),
                    editor::actions::SelectAllMatches,
                ),
                MenuItem::separator(),
                MenuItem::action(tr(cx, "Move Line Up"), editor::actions::MoveLineUp),
                MenuItem::action(tr(cx, "Move Line Down"), editor::actions::MoveLineDown),
                MenuItem::action(
                    tr(cx, "Duplicate Selection"),
                    editor::actions::DuplicateLineDown,
                ),
            ],
        },
        Menu {
            name: tr(cx, "View"),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: tr(cx, "Go"),
            disabled: false,
            items: vec![
                MenuItem::action(tr(cx, "Back"), workspace::GoBack),
                MenuItem::action(tr(cx, "Forward"), workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(
                    tr(cx, "Command Palette..."),
                    zed_actions::command_palette::Toggle,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    tr(cx, "Go to File..."),
                    workspace::ToggleFileFinder::default(),
                ),
                // MenuItem::action(tr(cx, "Go to Symbol in Project"), project_symbols::Toggle),
                MenuItem::action(
                    tr(cx, "Go to Symbol in Editor..."),
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action(
                    tr(cx, "Go to Line/Column..."),
                    editor::actions::ToggleGoToLine,
                ),
                MenuItem::separator(),
                MenuItem::action(tr(cx, "Go to Definition"), editor::actions::GoToDefinition),
                MenuItem::action(
                    tr(cx, "Go to Declaration"),
                    editor::actions::GoToDeclaration,
                ),
                MenuItem::action(
                    tr(cx, "Go to Type Definition"),
                    editor::actions::GoToTypeDefinition,
                ),
                MenuItem::action(
                    tr(cx, "Find All References"),
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action(
                    tr(cx, "Next Problem"),
                    editor::actions::GoToDiagnostic::default(),
                ),
                MenuItem::action(
                    tr(cx, "Previous Problem"),
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: tr(cx, "Run"),
            disabled: false,
            items: vec![
                MenuItem::action(
                    tr(cx, "Spawn Task"),
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action(tr(cx, "Edit tasks.json..."), crate::zed::OpenProjectTasks),
            ],
        },
        Menu {
            name: tr(cx, "Window"),
            disabled: false,
            items: vec![
                MenuItem::action(tr(cx, "Minimize"), super::Minimize),
                MenuItem::action(tr(cx, "Zoom"), super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: tr(cx, "Help"),
            disabled: false,
            items: vec![
                MenuItem::action(
                    tr(cx, "Documentation"),
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    tr(cx, "Zed Repository"),
                    super::OpenBrowser {
                        url: "https://github.com/zed-industries/zed".into(),
                    },
                ),
            ],
        },
    ]
}
