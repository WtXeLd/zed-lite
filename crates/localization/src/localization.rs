use gpui::{App, PromptButton, SharedString};
use settings::SettingsStore;
pub use settings::UiLanguage;

#[derive(Clone)]
pub enum LocalizableString {
    User(SharedString),
    Ui(&'static str),
}

impl From<&'static str> for LocalizableString {
    fn from(value: &'static str) -> Self {
        Self::User(value.into())
    }
}

impl From<String> for LocalizableString {
    fn from(value: String) -> Self {
        Self::User(value.into())
    }
}

impl From<&String> for LocalizableString {
    fn from(value: &String) -> Self {
        Self::User(value.clone().into())
    }
}

impl From<SharedString> for LocalizableString {
    fn from(value: SharedString) -> Self {
        Self::User(value)
    }
}

impl LocalizableString {
    pub fn resolve(self, cx: &App) -> SharedString {
        match self {
            Self::User(value) => value,
            Self::Ui(source) => t(cx, source),
        }
    }

    pub fn source_text(&self) -> &str {
        match self {
            Self::User(value) => value.as_ref(),
            Self::Ui(source) => source,
        }
    }

    pub fn map_user(self, map: impl FnOnce(SharedString) -> SharedString) -> Self {
        match self {
            Self::User(value) => Self::User(map(value)),
            Self::Ui(source) => Self::Ui(source),
        }
    }
}

pub fn ui(source: &'static str) -> LocalizableString {
    LocalizableString::Ui(source)
}

pub fn current_language(cx: &App) -> UiLanguage {
    cx.try_global::<SettingsStore>()
        .and_then(|settings| settings.merged_settings().ui_language)
        .unwrap_or_default()
}

pub fn t(cx: &App, source: &'static str) -> SharedString {
    translate(current_language(cx), source).into()
}

pub fn prompt_button(cx: &App, source: &'static str) -> PromptButton {
    PromptButton::localized(source, t(cx, source))
}

pub fn t_shared(cx: &App, source: SharedString) -> SharedString {
    translate_str(current_language(cx), source.as_ref())
        .map(Into::into)
        .unwrap_or(source)
}

pub fn t_shared_if_known(cx: &App, source: &SharedString) -> SharedString {
    translate_str(current_language(cx), source.as_ref())
        .map(Into::into)
        .unwrap_or_else(|| source.clone())
}

pub fn translate(language: UiLanguage, source: &'static str) -> &'static str {
    translate_str(language, source).unwrap_or(source)
}

pub fn translate_str(language: UiLanguage, source: &str) -> Option<&'static str> {
    match language {
        UiLanguage::English => None,
        UiLanguage::ChineseSimplified => zh_cn(source),
    }
}

fn zh_cn(source: &str) -> Option<&'static str> {
    Some(match source {
        "Appearance" => "外观",
        "Auto Save" => "自动保存",
        "Auto Save Mode" => "自动保存模式",
        "Activate On Close" => "关闭后激活",
        "Active Encoding Button" => "当前编码按钮",
        "Active File Name" => "当前文件名",
        "Active Language Button" => "当前语言按钮",
        "Active Line Width" => "活动辅助线宽度",
        "Allow Rewrap" => "允许重新换行",
        "Allowed" => "允许",
        "Alternate Scroll" => "备用滚动",
        "Always Treat Brackets As Autoclosed" => "始终将括号视为自动闭合",
        "Arguments" => "参数",
        "Audible Bell" => "声音提示",
        "Auto Fold Directories" => "自动折叠目录",
        "Auto Indent" => "自动缩进",
        "Auto Indent On Paste" => "粘贴时自动缩进",
        "Auto Open Files On Create" => "创建后自动打开文件",
        "Auto Open Files On Drop" => "拖放后自动打开文件",
        "Auto Open Files On Paste" => "粘贴后自动打开文件",
        "Auto Reveal Entries" => "自动显示条目",
        "Auto Signature Help" => "自动签名帮助",
        "Autoscroll On Clicks" => "点击时自动滚动",
        "Base Keymap" => "基础快捷键",
        "Background Coloring" => "背景着色",
        "Bold Folder Labels" => "文件夹标签加粗",
        "Border Size" => "边框大小",
        "Buffer Font" => "编辑器字体",
        "Buffer Search" => "缓冲区搜索",
        "Bottom Dock Layout" => "底部停靠栏布局",
        "Breadcrumbs" => "面包屑",
        "Button Layout" => "按钮布局",
        "Case Sensitive" => "区分大小写",
        "Center on Match" => "匹配项居中",
        "Centered Layout Left Padding" => "居中布局左内边距",
        "Centered Layout Right Padding" => "居中布局右内边距",
        "CLI Default Open Behavior" => "CLI 默认打开行为",
        "Choose a static, fixed theme or dynamically select themes based on appearance and light/dark modes." => {
            "选择固定主题，或根据外观和明暗模式动态选择主题。"
        }
        "Clone a repository from GitHub or other sources." => "从 GitHub 或其他来源克隆仓库。",
        "Code Actions On Format" => "格式化时代码操作",
        "Completions" => "补全",
        "Cursor" => "光标",
        "Cursor Blink" => "光标闪烁",
        "Cursor Shape" => "光标形状",
        "Customize keybindings in the keymap editor." => "在快捷键编辑器中自定义快捷键。",
        "Dark Icon Theme" => "深色图标主题",
        "Dark Theme" => "深色主题",
        "Delay (milliseconds)" => "延迟（毫秒）",
        "Developer" => "开发者",
        "Display Settings" => "显示设置",
        "Edit Keybindings" => "编辑快捷键",
        "Editor" => "编辑器",
        "Enable Helix mode and key bindings." => "启用 Helix 模式和快捷键。",
        "Enable Vim mode and key bindings." => "启用 Vim 模式和快捷键。",
        "Feature Flags" => "功能开关",
        "Font Fallbacks" => "备用字体",
        "Font Family" => "字体族",
        "Font Features" => "字体特性",
        "Font Size" => "字号",
        "Font Weight" => "字体粗细",
        "Font family for UI elements." => "用于界面元素的字体族。",
        "Font family for editor text." => "用于编辑器文本的字体族。",
        "Font size for UI elements." => "用于界面元素的字号。",
        "Font size for editor text." => "用于编辑器文本的字号。",
        "Font weight for UI elements (100-900)." => "界面元素的字体粗细（100-900）。",
        "Font weight for editor text (100-900)." => "编辑器文本的字体粗细（100-900）。",
        "Additional code actions to run when formatting." => "格式化时额外运行的代码操作。",
        "Amount of indentation for nested items." => "嵌套条目的缩进量。",
        "Amount of time to wait before changing focus." => "切换焦点前等待的时间。",
        "Automatically close files that have been deleted." => "自动关闭已删除的文件。",
        "Automatically show a signature help pop-up." => "自动显示签名帮助弹窗。",
        "Character counts at which to show wrap guides." => "显示换行参考线的字符列数。",
        "Character counts at which to show wrap guides in the editor." => {
            "编辑器中显示换行参考线的字符列数。"
        }
        "Control when to show the active encoding in the status bar." => {
            "控制何时在状态栏显示当前编码。"
        }
        "Control whether Git status is shown in the editor's gutter." => {
            "控制是否在编辑器边栏显示 Git 状态。"
        }
        "Controls automatic indentation behavior when typing." => "控制输入时的自动缩进行为。",
        "Controls how LSP completions are inserted." => "控制 LSP 补全的插入方式。",
        "Controls how words are completed." => "控制单词补全方式。",
        "Controls when to use system clipboard in Vim mode." => {
            "控制 Vim 模式下何时使用系统剪贴板。"
        }
        "Controls where the `editor::rewrap` action is allowed for this language." => {
            "控制此语言允许在哪里执行 `editor::rewrap` 操作。"
        }
        "Cursor shape for insert mode. Inherit uses the editor's cursor shape." => {
            "插入模式的光标形状。继承时使用编辑器的光标形状。"
        }
        "Cursor shape for normal mode." => "普通模式的光标形状。",
        "Cursor shape for replace mode." => "替换模式的光标形状。",
        "Cursor shape for the editor." => "编辑器的光标形状。",
        "Cursor shape for visual mode." => "可视模式的光标形状。",
        "Custom digraph mappings for Vim mode." => "Vim 模式的自定义双字符映射。",
        "Custom line height value (must be at least 1.0)." => "自定义行高值（至少为 1.0）。",
        "Debounce threshold in milliseconds after which changes are reflected in the Git gutter." => {
            "变更反映到 Git 边栏前的防抖阈值（毫秒）。"
        }
        "Default branch name will be when init.defaultbranch is not set in Git." => {
            "Git 未设置 init.defaultbranch 时使用的默认分支名。"
        }
        "Default depth to expand outline items in the current file." => {
            "当前文件中默认展开大纲项的深度。"
        }
        "Default height when the terminal is docked to the bottom (in pixels)." => {
            "终端停靠在底部时的默认高度（像素）。"
        }
        "Default width of the Git panel in pixels." => "Git 面板默认宽度（像素）。",
        "Default width of the outline panel in pixels." => "大纲面板默认宽度（像素）。",
        "Default width of the project panel in pixels." => "项目面板默认宽度（像素）。",
        "Default width when the terminal is docked to the left or right (in pixels)." => {
            "终端停靠在左侧或右侧时的默认宽度（像素）。"
        }
        "Delay in milliseconds before drag and drop selection starts." => {
            "拖放选择开始前的延迟（毫秒）。"
        }
        "Delay in milliseconds before the which-key menu appears." => {
            "Which-key 菜单显示前的延迟（毫秒）。"
        }
        "Determines how indent guide backgrounds are colored." => "决定缩进辅助线背景的着色方式。",
        "Determines how indent guides are colored." => "决定缩进辅助线的着色方式。",
        "Determines how snippets are sorted relative to other completion items." => {
            "决定片段相对于其他补全项的排序方式。"
        }
        "Direction to split horizontally." => "水平拆分方向。",
        "Direction to split vertically." => "垂直拆分方向。",
        "Disable all Git integration features in Zed." => "禁用 Zed 中所有 Git 集成功能。",
        "Display indent guides in the editor." => "在编辑器中显示缩进辅助线。",
        "Display the terminal title in breadcrumbs inside the terminal pane." => {
            "在终端面板的面包屑中显示终端标题。"
        }
        "Enable drag and drop selection." => "启用拖放选择。",
        "Enable middle-click paste on Linux." => "在 Linux 上启用中键粘贴。",
        "Enable smartcase searching in Vim mode." => "在 Vim 模式下启用智能大小写搜索。",
        "Enables or disables formatting with Prettier for a given language." => {
            "为指定语言启用或禁用 Prettier 格式化。"
        }
        "Extra task variables to set for a particular language." => {
            "为特定语言设置的额外任务变量。"
        }
        "Fast scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平和垂直快速滚动的灵敏度倍数。"
        }
        "Font features for terminal text." => "终端文本的字体特性。",
        "Font size for terminal text. If not set, defaults to buffer font size." => {
            "终端文本字号。未设置时默认使用编辑器字号。"
        }
        "Font weight for terminal text in CSS weight units (100-900)." => {
            "终端文本的字体粗细，使用 CSS 字重单位（100-900）。"
        }
        "(Linux only) choose how window control buttons are laid out in the titlebar." => {
            "（仅 Linux）选择标题栏中窗口控制按钮的布局方式。"
        }
        "(Linux only) whether Zed or your compositor should draw window decorations." => {
            "（仅 Linux）由 Zed 还是窗口合成器绘制窗口装饰。"
        }
        "(macOS only) whether to allow Windows to tab together." => {
            "（仅 macOS）是否允许窗口合并为标签页。"
        }
        "A mapping from languages to files and file extensions that should be treated as that language." => {
            "语言到文件和扩展名的映射，用于将这些文件视为对应语言。"
        }
        "Activates the Python virtual environment, if one is found, in the terminal's working directory." => {
            "如果在终端工作目录中找到 Python 虚拟环境，则激活它。"
        }
        "An optional string to override the title of the terminal tab." => {
            "用于覆盖终端标签页标题的可选字符串。"
        }
        "Any number of settings profiles that are temporarily applied on top of your existing user settings." => {
            "任意数量的设置配置，可临时叠加在现有用户设置之上。"
        }
        "Border style for the minimap's scrollbar thumb." => "小地图滚动条滑块的边框样式。",
        "Choose whether to use the selected light or dark icon theme or to follow your OS appearance configuration." => {
            "选择使用所选浅色/深色图标主题，或跟随系统外观配置。"
        }
        "Choose whether to use the selected light or dark theme or to follow your OS appearance configuration." => {
            "选择使用所选浅色/深色主题，或跟随系统外观配置。"
        }
        "Collect timing data for foreground and background executor tasks so they can be inspected via `zed: open performance profiler`. May lead to increased memory usage." => {
            "收集前台和后台执行器任务的计时数据，以便通过 `zed: open performance profiler` 检查。可能增加内存使用。"
        }
        "Controls line number display in the editor's gutter. \"disabled\" shows absolute line numbers, \"enabled\" shows relative line numbers for each absolute line, and \"wrapped\" shows relative line numbers for every line, absolute or wrapped." => {
            "控制编辑器边栏中的行号显示。\"disabled\" 显示绝对行号，\"enabled\" 为每个绝对行显示相对行号，\"wrapped\" 为每一行（包括换行后的视觉行）显示相对行号。"
        }
        "Controls the appearance behavior of the tab's close button." => {
            "控制标签页关闭按钮的显示行为。"
        }
        "Controls whether the closing characters are always skipped over and auto-removed no matter how they were inserted." => {
            "控制闭合字符是否总是被跳过并自动移除，无论它们是如何插入的。"
        }
        "Default Prettier options, in the format as in package.json section for Prettier." => {
            "默认 Prettier 选项，格式与 package.json 中的 Prettier 配置相同。"
        }
        "Default cursor shape for the terminal (bar, block, underline, or hollow)." => {
            "终端的默认光标形状（竖线、方块、下划线或空心）。"
        }
        "Determines how much space the file finder can take up in relation to the available window width." => {
            "决定文件查找器相对于可用窗口宽度可占用多少空间。"
        }
        "Display the which-key menu with matching bindings while a multi-stroke binding is pending." => {
            "多键快捷键待完成时，显示包含匹配绑定的 which-key 菜单。"
        }
        "Duration in milliseconds to highlight yanked text in Vim mode." => {
            "Vim 模式下高亮已复制文本的持续时间（毫秒）。"
        }
        "Enable to show entries in tree view list, disable to show in flat view list." => {
            "启用后以树状列表显示条目，禁用后以平铺列表显示。"
        }
        "Enable to sort entries in the panel by path, disable to sort by status." => {
            "启用后按路径排序面板条目，禁用后按状态排序。"
        }
        "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"" => {
            "Zed 完全排除的文件或文件 glob。它们会在文件扫描和文件搜索中被跳过，也不会显示在项目文件树中。优先级高于 \"File Scan Inclusions\"。"
        }
        "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions" => {
            "即使被 git 忽略也会被 Zed 包含的文件或文件 glob。适用于未被 git 跟踪但对项目仍重要的文件。注意，过宽的 glob 会降低 Zed 文件扫描速度。\"File Scan Exclusions\" 优先级高于这些包含项。"
        }
        "Font fallbacks for terminal text. If not set, defaults to buffer font fallbacks." => {
            "终端文本的备用字体。未设置时默认使用编辑器备用字体。"
        }
        "Font family for terminal text. If not set, defaults to buffer font family." => {
            "终端文本字体族。未设置时默认使用编辑器字体族。"
        }
        "Forces Prettier integration to use a specific parser name when formatting files with the language." => {
            "强制 Prettier 集成在格式化此语言文件时使用指定解析器名称。"
        }
        "Forces Prettier integration to use specific plugins when formatting files with the language." => {
            "强制 Prettier 集成在格式化此语言文件时使用指定插件。"
        }
        "GNOME-style layout string such as \"close:minimize,maximize\"." => {
            "GNOME 风格的布局字符串，例如 \"close:minimize,maximize\"。"
        }
        "Globs to match files that will be considered \"hidden\" and can be hidden from the project panel." => {
            "用于匹配隐藏文件的 glob，这些文件可在项目面板中隐藏。"
        }
        "General" => "通用",
        "General Settings" => "通用设置",
        "Get Started" => "开始使用",
        "Git Integration" => "Git 集成",
        "Guides" => "辅助线",
        "Helix Mode" => "Helix 模式",
        "Hide Mouse" => "隐藏鼠标",
        "Highlighting" => "高亮",
        "Icon Theme" => "图标主题",
        "Icon Theme Name" => "图标主题名称",
        "Keybindings" => "快捷键",
        "Keymap" => "快捷键",
        "Language Servers" => "语言服务器",
        "Locked File" => "文件已锁定",
        "Languages & Tools" => "语言和工具",
        "Layout Settings" => "布局设置",
        "Light Icon Theme" => "浅色图标主题",
        "Light Theme" => "浅色主题",
        "Line Ending" => "换行符",
        "Line Height" => "行高",
        "Mode" => "模式",
        "Multi Cursor Modifier" => "多光标修饰键",
        "Network" => "网络",
        "No settings match" => "没有匹配的设置",
        "On Last Window Closed" => "关闭最后一个窗口时",
        "Panels" => "面板",
        "Performance Profiler" => "性能分析器",
        "Performance metrics:" => "性能指标：",
        "Preview Channel" => "预览通道",
        "Preview CSV" => "预览 CSV",
        "Preview Markdown" => "预览 Markdown",
        "Preview SVG" => "预览 SVG",
        "Private Files" => "私有文件",
        "Proxy" => "代理",
        "Redact Private Values" => "隐藏私有值",
        "Restore On Startup" => "启动时恢复",
        "Restore Unsaved Buffers" => "恢复未保存缓冲区",
        "Scoped Settings" => "作用域设置",
        "Search & Files" => "搜索和文件",
        "Security" => "安全",
        "Settings Profiles" => "设置配置档",
        "Show Wrap Guides" => "显示换行参考线",
        "Text Rendering" => "文本渲染",
        "Text Alignment:" => "文本对齐：",
        "Text Rendering Mode" => "文本渲染模式",
        "The language used for Zed Lite's UI and application menus." => {
            "用于 Zed Lite 界面和应用菜单的语言。"
        }
        "The name of a base set of key bindings to use." => "要使用的基础快捷键方案名称。",
        "The name of your selected icon theme." => "所选图标主题的名称。",
        "The name of your selected theme." => "所选主题的名称。",
        "Theme" => "主题",
        "Theme Mode" => "主题模式",
        "Theme Name" => "主题名称",
        "UI Font" => "界面字体",
        "UI Language" => "界面语言",
        "Use System Path Prompts" => "使用系统路径对话框",
        "Use System Prompts" => "使用系统提示框",
        "Version Control" => "版本控制",
        "Vim Mode" => "Vim 模式",
        "When Closing With No Tabs" => "无标签页时关闭",
        "Where would you like to initialize this git repository?" => {
            "要在哪个目录中初始化此 Git 仓库？"
        }
        "Window & Layout" => "窗口和布局",
        "Workspace Restoration" => "工作区恢复",
        "worktree root" => "工作树根目录",
        "What to do when the last window is closed." => "关闭最后一个窗口时要执行的操作。",
        "What to do when using the 'close active item' action with no tabs." => {
            "没有标签页时使用“关闭当前项”操作要执行的行为。"
        }
        "What to restore from the previous session when opening Zed." => {
            "打开 Zed 时从上一次会话恢复哪些内容。"
        }
        "Whether or not to restore unsaved buffers on restart." => "重启时是否恢复未保存的缓冲区。",
        "About Zed" => "关于 Zed",
        "(binary file not shown)" => "（二进制文件未显示）",
        "Actions" => "操作",
        "Action" => "操作",
        "Activate" => "激活",
        "Add" => "添加",
        "Add Review (drag to select multiple lines)" => "添加评审（拖动可选择多行）",
        "Add comment" => "添加评论",
        "Add Cursor Above" => "在上方添加光标",
        "Add Cursor Below" => "在下方添加光标",
        "Add Folder to Project…" => "添加文件夹到项目…",
        "Add Keybinding…" => "添加快捷键…",
        "Add a review comment..." => "添加评审评论...",
        "Add repo to project" => "添加仓库到项目",
        "Add to .gitignore" => "添加到 .gitignore",
        "All highlights are filtered out" => "所有高亮都已被过滤",
        "Apply" => "应用",
        "Are you sure?" => "确定吗？",
        "Are you sure you want to quit?" => "确定要退出吗？",
        "Are you sure you want to restart?" => "确定要重启吗？",
        "Author" => "作者",
        "Back" => "后退",
        "Background" => "背景",
        "Backup and Update" => "备份并更新",
        "Body Text" => "正文文本",
        "Bottom" => "底部",
        "Based off the current branch" => "基于当前分支",
        "Cancel" => "取消",
        "Categories" => "类别",
        "Center" => "居中",
        "Change Keybinding…" => "更改快捷键…",
        "Clear Events" => "清除事件",
        "Clear Filter" => "清除过滤器",
        "Clear" => "清除",
        "Close" => "关闭",
        "Close Editor" => "关闭编辑器",
        "Close Without Saving" => "不保存并关闭",
        "Close Other Tabs" => "关闭其他标签页",
        "Close Project" => "关闭项目",
        "Close Tab" => "关闭标签页",
        "Close Window" => "关闭窗口",
        "Close on File Delete" => "文件删除时关闭",
        "Clone Repository" => "克隆仓库",
        "Code Lens" => "代码透镜",
        "Code Actions" => "代码操作",
        "Collapse Untracked Diff" => "折叠未跟踪差异",
        "Colors" => "颜色",
        "Coloring" => "着色",
        "Colorize Brackets" => "彩色括号",
        "Commit Title Max Length" => "提交标题最大长度",
        "Completion Detail Alignment" => "补全详情对齐",
        "Completion Menu Item Kind" => "补全菜单项类型",
        "Completion Menu Scrollbar" => "补全菜单滚动条",
        "Command Palette..." => "命令面板...",
        "Changes" => "变更",
        "Click to Close" => "点击关闭",
        "Commit" => "提交",
        "Commit SHA" => "提交 SHA",
        "Confirm" => "确认",
        "Copy" => "复制",
        "Copied!" => "已复制！",
        "Copy Diagnostic" => "复制诊断",
        "Copy Link" => "复制链接",
        "Copy Path" => "复制路径",
        "Copy Relative Path" => "复制相对路径",
        "Copy Action" => "复制操作",
        "Copy Context" => "复制上下文",
        "Copy and Trim" => "复制并裁剪",
        "Could not start inotify" => "无法启动 inotify",
        "Could not start ReadDirectoryChangesW" => "无法启动 ReadDirectoryChangesW",
        "Could not open file" => "无法打开文件",
        "Choose vertical text alignment within cells" => "选择单元格内文本的垂直对齐方式",
        "Create" => "创建",
        "Create Branch" => "创建分支",
        "Compare Marked Files" => "比较已标记文件",
        "Create Remote" => "创建远程",
        "Create Keybinding" => "创建快捷键",
        "Create Remote Repository" => "创建远程仓库",
        "Current Branch" => "当前分支",
        "Current Context Stack" => "当前上下文栈",
        "Current editor has no associated language" => "当前编辑器没有关联语言",
        "Configure" => "配置",
        "Copy On Select" => "选中即复制",
        "Created Text" => "新增文本",
        "Custom Commands" => "自定义命令",
        "Current Line Highlight" => "当前行高亮",
        "Cursor Blinking" => "光标闪烁",
        "Cursor Position Button" => "光标位置按钮",
        "Cursor Shape - Insert Mode" => "插入模式光标形状",
        "Cursor Shape - Normal Mode" => "普通模式光标形状",
        "Cursor Shape - Replace Mode" => "替换模式光标形状",
        "Cursor Shape - Visual Mode" => "可视模式光标形状",
        "Cursors" => "光标",
        "Custom Button Layout" => "自定义按钮布局",
        "Custom Digraphs" => "自定义双字符",
        "Custom Line Height" => "自定义行高",
        "Customize Keymaps" => "自定义快捷键",
        "current branches" => "当前分支",
        "Cut" => "剪切",
        "CSV Debug Options" => "CSV 调试选项",
        "CSV Preview" => "CSV 预览",
        "Date" => "日期",
        "Default Text" => "默认文本",
        "Delete" => "删除",
        "Delete from Recent Tasks" => "从最近任务中删除",
        "Delete from Recent Projects" => "从最近项目中删除",
        "Deleted Text" => "已删除文本",
        "Description" => "描述",
        "Debounce" => "防抖",
        "Default Height" => "默认高度",
        "Default Mode" => "默认模式",
        "Default Width" => "默认宽度",
        "Delay" => "延迟",
        "Detect Virtual Environment" => "检测虚拟环境",
        "Disabled Text" => "禁用文本",
        "Dismiss Button" => "关闭按钮",
        "Drop Size Target" => "拖放目标大小",
        "Dev-only:" => "仅开发：",
        "Dev-only section used for debugging purposes.\nWill be removed on public release of CSV feature" => {
            "用于调试的开发专用区域。\nCSV 功能公开发布时会移除"
        }
        "Diagnostics" => "诊断",
        "Diagnostic Badges" => "诊断徽标",
        "Diagnostics Button" => "诊断按钮",
        "Diff Stats" => "差异统计",
        "Diff View Style" => "差异视图样式",
        "Directory" => "目录",
        "Disable Git Integration" => "禁用 Git 集成",
        "Disconnected" => "已断开连接",
        "Display In" => "显示位置",
        "Documentation" => "文档",
        "Double Click In Multibuffer" => "多缓冲区双击",
        "Drag and Drop" => "拖放",
        "Do you want to save all changes in the following files?" => {
            "是否保存以下文件中的所有更改？"
        }
        "Do you want to save changes to the following files?" => "是否保存以下文件的更改？",
        "Don't Save" => "不保存",
        "Drop" => "丢弃",
        "Drop Stash" => "丢弃贮藏",
        "Don't ask me again" => "不再询问",
        "Discard" => "丢弃",
        "Discard all" => "全部丢弃",
        "Discard Changes" => "丢弃更改",
        "Discard Tracked Changes" => "丢弃已跟踪文件更改",
        "Dismiss" => "关闭",
        "Duplicate Selection" => "复制选区",
        "Duplicate" => "复制",
        "ElevatedSurface" => "浮层表面",
        "Edit" => "编辑",
        "Edit Arguments" => "编辑参数",
        "Edit in JSON" => "在 JSON 中编辑",
        "Edit Keybinding" => "编辑快捷键",
        "Edit Context" => "编辑上下文",
        "Edit in settings.json" => "在 settings.json 中编辑",
        "Edit Keymap File" => "编辑快捷键文件",
        "Edit Keystroke" => "编辑按键",
        "Edit Settings" => "编辑设置",
        "Edit Debounce Ms" => "编辑防抖毫秒数",
        "Edit tasks.json..." => "编辑 tasks.json...",
        "EditorSurface" => "编辑器表面",
        "Editor Controls" => "编辑器控制",
        "Editor Layout" => "编辑器布局",
        "Enable Git Diff" => "启用 Git 差异",
        "Enable Git Status" => "启用 Git 状态",
        "Enable Keep Preview On Code Navigation" => "代码导航时保留预览",
        "Enable Language Server" => "启用语言服务器",
        "Enable Preview File From Code Navigation" => "代码导航时预览文件",
        "Enable Preview From File Finder" => "从文件查找器预览",
        "Enable Preview From Multibuffer" => "从多缓冲区预览",
        "Enable Preview From Project Panel" => "从项目面板预览",
        "Enable Preview Multibuffer From Code Navigation" => "代码导航时预览多缓冲区",
        "Enabled" => "已启用",
        "Ensure Final Newline On Save" => "保存时确保末尾换行",
        "Entry Spacing" => "条目间距",
        "Environment Variables" => "环境变量",
        "Error Text" => "错误文本",
        "Excerpt Context Lines" => "摘录上下文行数",
        "Expand Excerpt Lines" => "展开摘录行数",
        "Expand Outlines With Depth" => "按深度展开大纲",
        "Expand Selection" => "扩大选区",
        "Expand Excerpt" => "展开摘录",
        "Extend Comment On Newline" => "换行时延续注释",
        "Exclude: vendor/*, *.lock" => "排除：vendor/*, *.lock",
        "Exclude Warnings" => "排除警告",
        "Explore Extensions" => "浏览扩展",
        "Extensions" => "扩展",
        "Failed to spawn terminal" => "启动终端失败",
        "Failed to read telemetry log" => "读取 telemetry 日志失败",
        "Failed to move Zed to Applications" => "无法将 Zed 移到 Applications",
        "Failed to Load Mermaid Diagram" => "加载 Mermaid 图失败",
        "Failed to load mermaid diagram" => "加载 Mermaid 图失败",
        "Failed to load SVG image" => "加载 SVG 图片失败",
        "Failed to apply stash" => "应用贮藏失败",
        "Failed to create branch" => "创建分支失败",
        "Failed to create buffer" => "创建缓冲区失败",
        "Failed to create remote" => "创建远程失败",
        "Failed to drop stash" => "丢弃贮藏失败",
        "Failed to open file" => "打开文件失败",
        "Failed to pop stash" => "弹出贮藏失败",
        "Failed to rename branch" => "重命名分支失败",
        "Failed to save" => "保存失败",
        "Failed to trash files" => "移到废纸篓失败",
        "Failed to write lines" => "写入行失败",
        "Fallback Branch Name" => "备用分支名",
        "Fast Scroll Sensitivity" => "快速滚动灵敏度",
        "Fetch Timeout (milliseconds)" => "获取超时（毫秒）",
        "File Icons" => "文件图标",
        "File Scan Exclusions" => "文件扫描排除项",
        "File Scan Inclusions" => "文件扫描包含项",
        "File Type Associations" => "文件类型关联",
        "Failed to :w" => ":w 失败",
        "Error installing zed cli" => "安装 zed cli 出错",
        "Filter Options" => "过滤选项",
        "Filters" => "过滤器",
        "File" => "文件",
        "Find" => "查找",
        "Find All References" => "查找所有引用",
        "Find in Project" => "在项目中查找",
        "Find in Folder…" => "在文件夹中查找…",
        "Find and replace" => "查找并替换",
        "Find in Results" => "在结果中查找",
        "Fix in settings.json" => "在 settings.json 中修复",
        "Focus Follows Mouse" => "焦点跟随鼠标",
        "Focus Follows Mouse Debounce ms" => "焦点跟随鼠标防抖毫秒数",
        "Focus an editor to show a new tree view" => "聚焦编辑器以显示新的语法树视图",
        "Focus an editor to show highlights" => "聚焦编辑器以显示高亮",
        "Fold Directory" => "折叠目录",
        "Folder Icons" => "文件夹图标",
        "Format Buffer" => "格式化缓冲区",
        "Format Selections" => "格式化选区",
        "Format On Save" => "保存时格式化",
        "Formatter" => "格式化工具",
        "Forward" => "前进",
        "Force Delete" => "强制删除",
        "Force Push" => "强制推送",
        "Fetch" => "获取",
        "Fetch From" => "从远程获取",
        "Git Diff" => "Git 差异",
        "Git Panel Button" => "Git 面板按钮",
        "Git Panel Default Width" => "Git 面板默认宽度",
        "Git Panel Dock" => "Git 面板停靠位置",
        "Git Panel Status Style" => "Git 面板状态样式",
        "Git Status" => "Git 状态",
        "Git Status Indicator" => "Git 状态指示器",
        "Global Substitution Default" => "全局替换默认值",
        "Go" => "跳转",
        "Go to Declaration" => "跳转到声明",
        "Go to Definition" => "跳转到定义",
        "Go to Implementation" => "跳转到实现",
        "Go to File..." => "跳转到文件...",
        "Go to Line/Column" => "跳转到行/列",
        "Go to Line/Column..." => "跳转到行/列...",
        "Go to next hunk" => "跳转到下一个变更块",
        "Go to previous hunk" => "跳转到上一个变更块",
        "Go to Symbol" => "跳转到符号",
        "Go to Symbol in Editor..." => "跳转到编辑器符号...",
        "Go to Type Definition" => "跳转到类型定义",
        "Go To Definition Fallback" => "跳转到定义回退",
        "Go To Definition Scroll Strategy" => "跳转到定义滚动策略",
        "Graph" => "图",
        "Hard Tabs" => "硬制表符",
        "Help" => "帮助",
        "History" => "历史",
        "Hide" => "隐藏",
        "Headline 1" => "一级标题",
        "Headline 2" => "二级标题",
        "Headline 3" => "三级标题",
        "Headline 4" => "四级标题",
        "Headline 5" => "五级标题",
        "Headline Sizes" => "标题尺寸",
        "Highlights Settings" => "高亮设置",
        "Hidden Text" => "隐藏文本",
        "Hidden Files" => "隐藏文件",
        "Hint Text" => "提示文本",
        "Hide Others" => "隐藏其他",
        "Hide Zed" => "隐藏 Zed",
        "Hide .gitignore" => "隐藏 .gitignore",
        "Hide Hidden" => "隐藏隐藏文件",
        "Hide Root" => "隐藏根目录",
        "Hiding Delay" => "隐藏延迟",
        "Highlight on Yank Duration" => "复制高亮持续时间",
        "Horizontal Scroll" => "水平滚动",
        "Horizontal Scroll Margin" => "水平滚动边距",
        "Horizontal Scrollbar" => "水平滚动条",
        "Horizontal Split Direction" => "水平拆分方向",
        "Hunk Style" => "变更块样式",
        "Image failed to load. Open `zed: log` for more details." => {
            "图片加载失败。打开 `zed: log` 查看详情。"
        }
        "Global switch to toggle hints on and off." => "用于开关提示的全局开关。",
        "Global switch to toggle inline values on and off when debugging." => {
            "调试时用于开关内联值的全局开关。"
        }
        "Globs to match against file paths to determine if a file is private." => {
            "用于匹配文件路径以判断文件是否为私有文件的 glob。"
        }
        "Hide the values of variables in private files." => "隐藏私有文件中的变量值。",
        "Highlight all occurrences of selected text." => "高亮选中文本的所有出现位置。",
        "How Git hunks are displayed visually in the editor." => {
            "Git 变更块在编辑器中的视觉显示方式。"
        }
        "How `zed <path>` opens directories when no flag is specified." => {
            "`zed <path>` 未指定参数时打开目录的方式。"
        }
        "How line endings should be handled for new files and during format and save operations." => {
            "新文件以及格式化和保存操作中换行符的处理方式。"
        }
        "How many characters has to be in the completions query to automatically show the words-based completions." => {
            "补全查询至少包含多少字符时自动显示基于单词的补全。"
        }
        "How many columns a tab should occupy." => "一个制表符占用的列数。",
        "How many lines of context to provide in multibuffer excerpts by default." => {
            "多缓冲区摘录默认提供的上下文行数。"
        }
        "How many lines to expand the multibuffer excerpts by default." => {
            "多缓冲区摘录默认展开的行数。"
        }
        "How much to fade out unused code (0.0 - 0.9)." => "未使用代码淡化程度（0.0 - 0.9）。",
        "How to display diffs in the editor." => "编辑器中显示差异的方式。",
        "How to display the LSP item kind (function, method, variable, etc.) of each entry in the completions menu." => {
            "补全菜单中每个条目的 LSP 类型（函数、方法、变量等）显示方式。"
        }
        "How to highlight the current line." => "当前行的高亮方式。",
        "How to perform a buffer format." => "执行缓冲区格式化的方式。",
        "How to render LSP color previews in the editor." => "编辑器中渲染 LSP 颜色预览的方式。",
        "How to scroll the target into view when navigating to a definition or reference." => {
            "跳转到定义或引用时如何滚动目标到视图中。"
        }
        "How to soft-wrap long lines of text." => "长文本行的软换行方式。",
        "How and when the scrollbar should be displayed." => "滚动条的显示方式和显示时机。",
        "How entry statuses are displayed." => "条目状态的显示方式。",
        "How to highlight the current line in the minimap." => "小地图中当前行的高亮方式。",
        "Include ignored files in search results by default." => "默认在搜索结果中包含被忽略文件。",
        "Key-value pairs to add to the terminal's environment." => "添加到终端环境中的键值对。",
        "Layout mode for the bottom dock." => "底部停靠栏的布局模式。",
        "Left padding for centered layout." => "居中布局的左内边距。",
        "Line height for editor text." => "编辑器文本行高。",
        "Line height for terminal text." => "终端文本行高。",
        "Maximum length of the commit message title before a warning is shown. Set to 0 to disable." => {
            "提交消息标题触发警告的最大长度。设为 0 可禁用。"
        }
        "Maximum number of columns to display in the minimap." => "小地图中显示的最大列数。",
        "Maximum number of lines to keep in scrollback history (max: 100,000; 0 disables scrolling)." => {
            "滚动历史中保留的最大行数（最大 100,000；0 禁用滚动）。"
        }
        "Maximum open tabs in a pane. Will not close an unsaved tab." => {
            "一个窗格中最多打开的标签页数。不会关闭未保存的标签页。"
        }
        "Minimum number of characters to reserve space for in the gutter." => {
            "边栏中预留空间的最小字符数。"
        }
        "Minimum time to wait before pulling diagnostics from the language server(s)." => {
            "从语言服务器拉取诊断前等待的最短时间。"
        }
        "Modifier key for adding multiple cursors." => "用于添加多个光标的修饰键。",
        "Number of lines to search for modelines (set to 0 to disable)." => {
            "搜索 modeline 的行数（设为 0 可禁用）。"
        }
        "Opacity of inactive panels (0.0 - 1.0)." => "非活动面板的透明度（0.0 - 1.0）。",
        "Padding between the end of the source line and the start of the inline blame in columns." => {
            "源码行末尾与内联 blame 开始位置之间的列间距。"
        }
        "Position of the close button in a tab." => "标签页中关闭按钮的位置。",
        "Relative size of the drop target in the editor that will open dropped file as a split pane." => {
            "编辑器中拖放目标的相对大小，该目标会将拖入文件打开为拆分窗格。"
        }
        "Restore previous file state when reopening." => "重新打开时恢复之前的文件状态。",
        "Right padding for centered layout." => "居中布局的右内边距。",
        "Save after inactivity period (in milliseconds)." => "空闲一段时间后保存（毫秒）。",
        "Scroll sensitivity multiplier for both horizontal and vertical scrolling." => {
            "水平和垂直滚动的灵敏度倍数。"
        }
        "Search case-sensitively by default." => "默认区分大小写搜索。",
        "Search for whole words by default." => "默认搜索完整单词。",
        "Sets the cursor blinking behavior in the terminal." => "设置终端中的光标闪烁行为。",
        "Should the name or path be displayed first in the git view." => {
            "Git 视图中名称或路径哪个优先显示。"
        }
        "Show Git diff indicators in the scrollbar." => "在滚动条中显示 Git 差异指示器。",
        "Show a badge on the terminal panel icon with the count of open terminals." => {
            "在终端面板图标上显示打开终端数量徽标。"
        }
        "Show a git status indicator next to file names in the project panel." => {
            "在项目面板文件名旁显示 Git 状态指示器。"
        }
        "Show author name as part of the commit information in branch picker." => {
            "在分支选择器的提交信息中显示作者名称。"
        }
        "Show buffer search result indicators in the scrollbar." => {
            "在滚动条中显示缓冲区搜索结果指示器。"
        }
        "Show code action buttons in the editor toolbar." => "在编辑器工具栏显示代码操作按钮。",
        "Show error and warning count badges next to file names in the project panel." => {
            "在项目面板文件名旁显示错误和警告数量徽标。"
        }
        "Show file icons next to the Git status icon." => "在 Git 状态图标旁显示文件图标。",
        "Show git status indicators on the branch icon in the titlebar." => {
            "在标题栏分支图标上显示 Git 状态指示器。"
        }
        "Show indent guides in the project panel." => "在项目面板中显示缩进辅助线。",
        "Show padding for zoomed panes." => "为缩放后的窗格显示内边距。",
        "Show pinned tabs in a separate row above unpinned tabs." => {
            "在未固定标签页上方单独一行显示固定标签页。"
        }
        "Show the Git file status on a tab item." => "在标签页条目上显示 Git 文件状态。",
        "Show the Git panel button in the status bar." => "在状态栏显示 Git 面板按钮。",
        "Show the Git status in the outline panel." => "在大纲面板中显示 Git 状态。",
        "Show the Git status in the project panel." => "在项目面板中显示 Git 状态。",
        "Show the active language button in the status bar." => "在状态栏显示当前语言按钮。",
        "Show the active line endings button in the status bar." => "在状态栏显示当前换行符按钮。",
        "Show the navigation history buttons in the tab bar." => "在标签栏显示导航历史按钮。",
        "Show the outline panel button in the status bar." => "在状态栏显示大纲面板按钮。",
        "Show the project diagnostics button in the status bar." => "在状态栏显示项目诊断按钮。",
        "Show the project host and name in the titlebar." => "在标题栏显示项目主机和名称。",
        "Show the project panel button in the status bar." => "在状态栏显示项目面板按钮。",
        "Show the scrollbar in the project panel." => "在项目面板中显示滚动条。",
        "Show the selections menu in the editor toolbar." => "在编辑器工具栏显示选区菜单。",
        "Show the signature help pop-up after completions or bracket pairs are inserted." => {
            "插入补全或括号对后显示签名帮助弹窗。"
        }
        "Show the tab bar buttons (New, Split Pane, Zoom)." => {
            "显示标签栏按钮（新建、拆分窗格、缩放）。"
        }
        "Show the tab bar in the editor." => "在编辑器中显示标签栏。",
        "Show the terminal button in the status bar." => "在状态栏显示终端按钮。",
        "Size of the border surrounding the active pane." => "活动窗格周围边框的大小。",
        "Sort order for entries in the project panel." => "项目面板中条目的排序顺序。",
        "Spacing between worktree entries in the project panel." => "项目面板中工作区条目的间距。",
        "Image Viewer" => "图片查看器",
        "Inactive Opacity" => "非活动透明度",
        "Inlay Hints" => "嵌入提示",
        "Inline Values" => "内联值",
        "Install CLI" => "安装 CLI",
        "Install Icon Themes" => "安装图标主题",
        "Install Themes" => "安装主题",
        "Initialize Repository" => "初始化仓库",
        "Inline Diagnostics" => "内联诊断",
        "Inline diagnostics are not available until regular diagnostics are enabled." => {
            "启用常规诊断后才能使用内联诊断。"
        }
        "Installing Zed…" => "正在安装 Zed…",
        "Include Ignored Files" => "包含被忽略文件",
        "Include Ignored" => "包含被忽略项",
        "Include Ignored in Search" => "搜索中包含被忽略文件",
        "Ignored Text" => "忽略文本",
        "Info Text" => "信息文本",
        "Include Warnings" => "包含警告",
        "Include: crates/**/*.toml" => "包含：crates/**/*.toml",
        "Indent Size" => "缩进大小",
        "Inline Code Actions" => "内联代码操作",
        "Insert Mode" => "插入模式",
        "JSX Tag Auto Close" => "JSX 标签自动闭合",
        "Key Equivalents" => "按键等效项",
        "Keybinding Context" => "快捷键上下文",
        "Keyboard Context" => "键盘上下文",
        "Keep Open" => "保持打开",
        "Keep Selection On Copy" => "复制时保留选区",
        "Learn More" => "了解更多",
        "Learn more" => "了解更多",
        "Left" => "左侧",
        "Language Servers can't run until you trust this project." => {
            "信任此项目后才能运行语言服务器。"
        }
        "Language servers from running" => "语言服务器运行",
        "Last Keystroke" => "上一次按键",
        "Large Headline" => "大标题",
        "MCP integrations from installing" => "MCP 集成安装",
        "MCP Server integrations from installing" => "MCP 服务器集成安装",
        "Loading Commit History…" => "正在加载提交历史…",
        "Loading" => "正在加载",
        "Line Endings Button" => "换行符按钮",
        "Line Width" => "线宽",
        "Linked Edits" => "关联编辑",
        "LSP Document Colors" => "LSP 文档颜色",
        "LSP Document Symbols" => "LSP 文档符号",
        "LSP Folding Ranges" => "LSP 折叠范围",
        "Manage Trust" => "管理信任",
        "Max Scroll History Lines" => "最大滚动历史行数",
        "Max Severity" => "最高严重级别",
        "Max Width Columns" => "最大宽度列数",
        "Maximum Tabs" => "最大标签页数",
        "Medium Headline" => "中标题",
        "Menu Delay" => "菜单延迟",
        "Minimize" => "最小化",
        "Middle Click Paste" => "中键粘贴",
        "Min Line Number Digits" => "最小行号位数",
        "Minimum Column" => "最小列",
        "Minimum Contrast" => "最小对比度",
        "Minimum Contrast For Highlights" => "高亮最小对比度",
        "Minimum Split Diff Width" => "分屏差异最小宽度",
        "Modal Max Width" => "模态框最大宽度",
        "Modified Text" => "已修改文本",
        "More Info" => "更多信息",
        "Muted Text" => "弱化文本",
        "Match case" => "匹配大小写",
        "Match with regex" => "使用正则匹配",
        "Match whole words" => "匹配完整单词",
        "Move Line Down" => "下移行",
        "Move Line Up" => "上移行",
        "Move Tab to New Window" => "将标签页移到新窗口",
        "Move Zed to Applications?" => "要将 Zed 移到 Applications 吗？",
        "Moving Zed to Applications" => "正在将 Zed 移到 Applications",
        "Mouse Wheel Zoom" => "鼠标滚轮缩放",
        "New" => "新建",
        "New Window" => "新窗口",
        "New File" => "新建文件",
        "New Folder" => "新建文件夹",
        "New Terminal" => "新建终端",
        "New Center Terminal" => "新建中央终端",
        "Next Hunk" => "下一个变更块",
        "Next Problem" => "下一个问题",
        "Next Signature" => "下一个签名",
        "No Git Repositories" => "没有 Git 仓库",
        "No highlights found" => "未找到高亮",
        "No problems" => "没有问题",
        "No problems in workspace" => "工作区没有问题",
        "No errors in workspace" => "工作区没有错误",
        "No Results" => "无结果",
        "No" => "否",
        "No changes to commit" => "没有可提交的变更",
        "No Code Actions Available" => "没有可用代码操作",
        "No CSV content to display" => "没有可显示的 CSV 内容",
        "No commits found" => "未找到提交",
        "No results found in this project for the provided query" => {
            "此项目中没有找到匹配当前查询的结果"
        }
        "No SVG file selected" => "未选择 SVG 文件",
        "No telemetry events recorded yet" => "尚未记录遥测事件",
        "No events match the current filter" => "没有匹配当前过滤器的事件",
        "No uncommitted changes" => "没有未提交的变更",
        "Not attached to an editor" => "未附加到编辑器",
        "Nathan Sobo accepted your contact request" => "Nathan Sobo 接受了你的联系人请求",
        "Ok" => "确定",
        "Operation completed" => "操作已完成",
        "Option As Meta" => "Option 作为 Meta",
        "Options" => "选项",
        "Open Application Menu" => "打开应用菜单",
        "Open Documentation" => "打开文档",
        "Open File" => "打开文件",
        "Open Diff" => "打开差异",
        "Open Diff (File)" => "打开差异（文件）",
        "Open Permalink" => "打开永久链接",
        "Open Project" => "打开项目",
        "Open" => "打开",
        "Open Default Key Bindings" => "打开默认快捷键",
        "Open Default Settings" => "打开默认设置",
        "Open File..." => "打开文件...",
        "Open Folder..." => "打开文件夹...",
        "Open Keymap" => "打开快捷键设置",
        "Open Keymap File" => "打开快捷键文件",
        "Open Local Folders" => "打开本地文件夹",
        "Open in Terminal" => "在终端中打开",
        "Open Project Settings" => "打开项目设置",
        "Open Project Settings File" => "打开项目设置文件",
        "Open Git Graph" => "打开 Git 图",
        "Open Command Palette" => "打开命令面板",
        "Open a directory first" => "请先打开一个目录",
        "Open repo in new project" => "在新项目中打开仓库",
        "Open Raw Log File" => "打开原始日志文件",
        "Open Markdown Preview" => "打开 Markdown 预览",
        "Open SVG Preview" => "打开 SVG 预览",
        "Open Recent..." => "打开最近项目...",
        "Open Settings" => "打开设置",
        "Open Settings File" => "打开设置文件",
        "Open Threads Sidebar" => "打开线程侧边栏",
        "Only Search Open Files" => "仅搜索打开的文件",
        "Open in Default App" => "在默认应用中打开",
        "Open in New Window" => "在新窗口打开",
        "Open Pull Request" => "打开拉取请求",
        "Open…" => "打开…",
        "Outline Panel" => "大纲面板",
        "Outline Panel Button" => "大纲面板按钮",
        "Outline Panel Default Width" => "大纲面板默认宽度",
        "Outline Panel Dock" => "大纲面板停靠位置",
        "Overwrite" => "覆盖",
        "Padding" => "内边距",
        "Parser" => "解析器",
        "Paste" => "粘贴",
        "Paste Text" => "粘贴文本",
        "Paste a URL to open." => "粘贴要打开的 URL。",
        "Path Style" => "路径样式",
        "Placeholder Text" => "占位文本",
        "Pick which remote to fetch" => "选择要拉取的远程",
        "Pick which remote to push to" => "选择要推送到的远程",
        "Pinned Tabs Layout" => "固定标签页布局",
        "Plugins" => "插件",
        "Pop" => "弹出",
        "Pop Stash" => "弹出贮藏",
        "Prefer LSP" => "优先使用 LSP",
        "Preferred Line Length" => "首选行长度",
        "Preview Tabs Enabled" => "启用预览标签页",
        "Previous Hunk" => "上一个变更块",
        "Previous Problem" => "上一个问题",
        "Previous Signature" => "上一个签名",
        "Pushed 4 changes to `zed/main`" => "已推送 4 个更改到 `zed/main`",
        "Project Panel" => "项目面板",
        "Project Diagnostics" => "项目诊断",
        "Program" => "程序",
        "Project Panel Button" => "项目面板按钮",
        "Project Panel Default Width" => "项目面板默认宽度",
        "Project Panel Dock" => "项目面板停靠位置",
        "Project Search Button" => "项目搜索按钮",
        "Quick Actions" => "快速操作",
        "Reveal In Project Panel" => "在项目面板中显示",
        "Reveal in File Explorer" => "在文件资源管理器中显示",
        "Reveal in File Manager" => "在文件管理器中显示",
        "Reveal in Finder" => "在 Finder 中显示",
        "Project Scan in Progress…" => "正在扫描项目…",
        "Project search buffer contains unsaved edits. Do you want to save it?" => {
            "项目搜索缓冲区包含未保存的编辑。是否保存？"
        }
        "Project is in Restricted Mode" => "项目处于受限模式",
        "Project settings from being applied" => "应用项目设置",
        "Pull" => "拉取",
        "Pull (Rebase)" => "拉取（变基）",
        "Push" => "推送",
        "Push To" => "推送到",
        "Quit Zed" => "退出 Zed",
        "Quit" => "退出",
        "Redo" => "重做",
        "Recent Projects" => "最近项目",
        "Regex" => "正则",
        "Regex Search" => "正则搜索",
        "Relative Line Numbers" => "相对行号",
        "REC" => "录制",
        "Remote up to date" => "远程已是最新",
        "Remove Folder" => "移除文件夹",
        "Remove Folder from Project" => "从项目中移除文件夹",
        "Remove Project from Window" => "从窗口中移除项目",
        "Remove from Project" => "从项目中移除",
        "Remove from Window" => "从窗口中移除",
        "Remove Trailing Whitespace On Save" => "保存时移除行尾空白",
        "Reset" => "重置",
        "Reset All Zoom" => "重置所有缩放",
        "Reset Zoom" => "重置缩放",
        "Reset to Default" => "重置为默认",
        "Rename" => "重命名",
        "Rename Symbol" => "重命名符号",
        "Rendering Mode" => "渲染模式",
        "Rendering..." => "正在渲染...",
        "Rerun task" => "重新运行任务",
        "Restore" => "还原",
        "Restore File" => "还原文件",
        "Restore File State" => "恢复文件状态",
        "Restore selected hunk" => "还原选中的变更块",
        "Rounded Selection" => "圆角选区",
        "Refresh Diagnostics" => "刷新诊断",
        "Restart Server" => "重启服务器",
        "Restart" => "重启",
        "Restricted Mode" => "受限模式",
        "Restricted Mode prevents:" => "受限模式会阻止：",
        "Restricted mode prevents:" => "受限模式会阻止：",
        "Right-Click to Copy Path" => "右键复制路径",
        "Right" => "右侧",
        "Run" => "运行",
        "Save" => "保存",
        "Save All" => "全部保存",
        "Save all" => "全部保存",
        "Save As…" => "另存为…",
        "Search All Files" => "搜索所有文件",
        "Search Inside" => "在内部搜索",
        "Search Project" => "搜索项目",
        "Search Results" => "搜索结果",
        "Search Wrap" => "循环搜索",
        "Scan Symbolic Links" => "扫描符号链接",
        "Scroll Bar" => "滚动条",
        "Scroll Beyond Last Line" => "滚动超过最后一行",
        "Scroll Debounce Ms" => "滚动防抖毫秒数",
        "Scroll Multiplier" => "滚动倍数",
        "Scroll Sensitivity" => "滚动灵敏度",
        "Search Symbols" => "搜索符号",
        "Seed Search Query From Cursor" => "从光标处填充搜索查询",
        "SEARCH" => "搜索",
        "Searching:" => "正在搜索：",
        "Searching…" => "正在搜索…",
        "Select All" => "全选",
        "Select All Occurrences" => "选择所有匹配项",
        "Select Icon Theme..." => "选择图标主题...",
        "Select Next Occurrence" => "选择下一个匹配项",
        "Select Next Sibling" => "选择下一个同级节点",
        "Select Previous Occurrence" => "选择上一个匹配项",
        "Select Previous Sibling" => "选择上一个同级节点",
        "Select Theme..." => "选择主题...",
        "Selection" => "选择",
        "Selection Controls" => "选择控制",
        "Semantic Highlights" => "语义高亮",
        "Semantic Tokens" => "语义令牌",
        "Services" => "服务",
        "Settings" => "设置",
        "Show All" => "全部显示",
        "Show" => "显示",
        "Show 1 warning" => "显示 1 条警告",
        "Show All Tabs" => "显示所有标签页",
        "Show Author Name" => "显示作者名称",
        "Show Avatar" => "显示头像",
        "Show Background" => "显示背景",
        "Show Bookmarks" => "显示书签",
        "Show Branch Name" => "显示分支名",
        "Show Branch Status Icon" => "显示分支状态图标",
        "Show Close Button" => "显示关闭按钮",
        "Show Code Actions" => "显示代码操作",
        "Show Commit Summary" => "显示提交摘要",
        "Show Completion Documentation" => "显示补全文档",
        "Show Completions On Input" => "输入时显示补全",
        "Show Count Badge" => "显示数量徽标",
        "Show Diagnostics" => "显示诊断",
        "Show Error Logs" => "显示错误日志",
        "Show File Icons In Tabs" => "在标签页中显示文件图标",
        "Show Folds" => "显示折叠控件",
        "Show Git Status In Tabs" => "在标签页中显示 Git 状态",
        "Show Indent Guides" => "显示缩进辅助线",
        "Show Line Numbers" => "显示行号",
        "Show Matching Keybindings" => "显示匹配的快捷键",
        "Show Menus" => "显示菜单",
        "Show Navigation History Buttons" => "显示导航历史按钮",
        "Show Other Hints" => "显示其他提示",
        "Show Parameter Hints" => "显示参数提示",
        "Show Project Items" => "显示项目条目",
        "Show Runnables" => "显示可运行项",
        "Show Scrollbar" => "显示滚动条",
        "Show Signature Help After Edits" => "编辑后显示签名帮助",
        "Show Stage/Restore Buttons" => "显示暂存/还原按钮",
        "Show Symbol Outline" => "显示符号大纲",
        "Show Tab Bar" => "显示标签栏",
        "Show Tab Bar Buttons" => "显示标签栏按钮",
        "Show Type Hints" => "显示类型提示",
        "Show Value Hints" => "显示值提示",
        "Show Which-key Menu" => "显示 Which-key 菜单",
        "Show Whitespaces" => "显示空白字符",
        "Show a background for inlay hints." => "为内嵌提示显示背景。",
        "Show bookmarks in the gutter." => "在边栏显示书签。",
        "Show breadcrumbs." => "显示面包屑。",
        "Show code action button at start of buffer line." => "在缓冲区行首显示代码操作按钮。",
        "Show code folding controls in the gutter." => "在边栏显示代码折叠控件。",
        "Show commit summary as part of the inline blame." => "在内联 blame 中显示提交摘要。",
        "Show cursor positions in the scrollbar." => "在滚动条中显示光标位置。",
        "Show file icons in the file finder." => "在文件查找器中显示文件图标。",
        "Show file icons in the outline panel." => "在大纲面板中显示文件图标。",
        "Show file icons in the project panel." => "在项目面板中显示文件图标。",
        "Show Git diff information in the editor." => "在编辑器中显示 Git 差异信息。",
        "Show Git status information in the editor." => "在编辑器中显示 Git 状态信息。",
        "Show line numbers in the gutter." => "在边栏显示行号。",
        "Show opened editors as preview tabs." => "将打开的编辑器显示为预览标签页。",
        "Show quick action buttons (e.g., search, selection, editor controls, etc.)." => {
            "显示快速操作按钮（如搜索、选择、编辑器控制等）。"
        }
        "Show runnable buttons in the gutter." => "在边栏显示可运行按钮。",
        "Show selected symbol occurrences in the scrollbar." => {
            "在滚动条中显示选中符号的出现位置。"
        }
        "Show selected text occurrences in the scrollbar." => "在滚动条中显示选中文本的出现位置。",
        "Show the avatar of the author of the commit." => "显示提交作者头像。",
        "Show the branch name button in the titlebar." => "在标题栏显示分支名按钮。",
        "Show the cursor position button in the status bar." => "在状态栏显示光标位置按钮。",
        "Show the file icon for a tab." => "为标签页显示文件图标。",
        "Show the informational hover box when moving the mouse over symbols in the editor." => {
            "鼠标悬停在编辑器符号上时显示信息浮窗。"
        }
        "Show the menus in the titlebar." => "在标题栏显示菜单。",
        "Show the name of the active file in the status bar." => "在状态栏显示当前文件名。",
        "Show the project search button in the status bar." => "在状态栏显示项目搜索按钮。",
        "Show wrap guides (vertical rulers)." => "显示换行参考线（垂直标尺）。",
        "Show wrap guides in the editor." => "在编辑器中显示换行参考线。",
        "Show perf metrics" => "显示性能指标",
        "Show in Git Graph" => "在 Git 图中显示",
        "Shift-click to Suppress" => "按住 Shift 点击可不再提示",
        "Shrink Selection" => "缩小选区",
        "Shortcuts defined using some characters have been remapped so that shortcuts can be typed without holding option." => {
            "使用某些字符定义的快捷键已被重映射，因此无需按住 Option 也能输入这些快捷键。"
        }
        "Select base branch" => "选择基准分支",
        "Select" => "选择",
        "Select Language" => "选择语言",
        "Select Line Ending" => "选择换行符",
        "Select Toolchain Path" => "选择工具链路径",
        "Selected Branch" => "已选分支",
        "Scope" => "作用域",
        "Selected Symbol" => "选中的符号",
        "Selected Text" => "选中的文本",
        "Selection Highlight" => "选区高亮",
        "Selections Menu" => "选区菜单",
        "Shell" => "Shell",
        "Skip Focus For Active In Search" => "搜索时跳过当前活动项焦点",
        "Snippet Sort Order" => "片段排序顺序",
        "Small Headline" => "小标题",
        "Soft Wrap" => "软换行",
        "Sort By Path" => "按路径排序",
        "Sort Mode" => "排序模式",
        "Sort Order" => "排序顺序",
        "Space Whitespace Indicator" => "空格空白标记",
        "Spawn Task" => "启动任务",
        "Split" => "拆分",
        "Split Pane" => "拆分窗格",
        "Split Down" => "向下拆分",
        "Split Left" => "向左拆分",
        "Split Right" => "向右拆分",
        "Split Up" => "向上拆分",
        "Split…" => "拆分…",
        "Stage" => "暂存",
        "Stage All" => "全部暂存",
        "Stash All" => "全部贮藏",
        "Stash Pop" => "弹出贮藏",
        "Stage all changes" => "暂存所有变更",
        "Stage File" => "暂存文件",
        "Stage file" => "暂存文件",
        "Stage and go to next hunk" => "暂存并跳转到下一个变更块",
        "Start Recording" => "开始录制",
        "Start Searching" => "开始搜索",
        "Starts Open" => "启动时打开",
        "Stay in Restricted Mode" => "保持受限模式",
        "Sticky" => "粘滞",
        "Sticky Scroll" => "粘滞滚动",
        "Stop Server" => "停止服务器",
        "Stop Diagnostics Update" => "停止诊断更新",
        "Stop Recording" => "停止录制",
        "Stop Searching" => "停止搜索",
        "Suppress" => "不再提示",
        "Success Text" => "成功文本",
        "Surface" => "表面",
        "Switch" => "切换",
        "Skip" => "跳过",
        "Switch Branch" => "切换分支",
        "SVG Preview" => "SVG 预览",
        "Text" => "文本",
        "Text Colors" => "文本颜色",
        "This action is unbound" => "此操作未绑定快捷键",
        "The editor for what's next" => "面向未来的编辑器",
        "Theme Preview" => "主题预览",
        "This file has changed on disk since you started editing it. Do you want to overwrite it?" => {
            "此文件在你开始编辑后已在磁盘上发生变化。是否覆盖？"
        }
        "This file has been deleted on disk since you started editing it. Do you want to recreate it?" => {
            "此文件在你开始编辑后已从磁盘删除。是否重新创建？"
        }
        "This file is read-only" => "此文件为只读",
        "This view lets you determine the current context stack for creating custom key bindings in Zed. When a keyboard shortcut is triggered, it also shows all the possible contexts it could have triggered in, and which one matched." => {
            "此视图可用于确定当前上下文栈，以便在 Zed 中创建自定义快捷键。触发快捷键时，它也会显示所有可能触发的上下文以及实际匹配的上下文。"
        }
        "This view lets you preview a range of UI elements across a theme. Use it for testing out changes to the theme." => {
            "此视图可用于预览一个主题下的多种 UI 元素。可用它测试主题改动。"
        }
        "This is a longer piece of text that should wrap to multiple lines. It demonstrates how text behaves when it exceeds the width of its container." => {
            "这是一段较长的文本，应该换行成多行。它用于展示文本超出容器宽度时的表现。"
        }
        "This will make this file editable" => "这会使此文件可编辑",
        "This will update your most recent commit." => "这会更新你最近一次提交。",
        "This cannot be undone." => "此操作无法撤销。",
        "The OpenType features to enable for rendering in UI elements." => {
            "渲染 UI 元素时启用的 OpenType 特性。"
        }
        "The OpenType features to enable for rendering in text buffers." => {
            "渲染文本缓冲区时启用的 OpenType 特性。"
        }
        "The amount of padding between the end of the source line and the start of the inline diagnostic." => {
            "源码行末尾与内联诊断开始位置之间的内边距。"
        }
        "The arguments to pass to the shell program." => "传递给 shell 程序的参数。",
        "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled." => {
            "启用软换行的缓冲区中执行软换行的列。"
        }
        "The custom set of icons Zed will associate with files and directories." => {
            "Zed 用于关联文件和目录的自定义图标集。"
        }
        "The debounce delay before querying highlights from the language." => {
            "查询语言高亮前的防抖延迟。"
        }
        "The default mode when Vim starts." => "Vim 启动时的默认模式。",
        "The delay after which the inline blame information is shown." => {
            "显示内联 blame 信息前的延迟。"
        }
        "The delay in milliseconds to show inline diagnostics after the last diagnostic update." => {
            "最后一次诊断更新后显示内联诊断的延迟（毫秒）。"
        }
        "The directory path to use (will be shell expanded)." => {
            "要使用的目录路径（会进行 shell 展开）。"
        }
        "The font fallbacks to use for rendering in text buffers." => {
            "渲染文本缓冲区时使用的备用字体。"
        }
        "The font fallbacks to use for rendering in the UI." => "渲染 UI 时使用的备用字体。",
        "The icon theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式设为深色，或设为系统且系统处于深色模式时使用的图标主题。"
        }
        "The icon theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式设为浅色，或设为系统且系统处于浅色模式时使用的图标主题。"
        }
        "The list of language servers to use (or disable) for this language." => {
            "此语言要使用（或禁用）的语言服务器列表。"
        }
        "The minimum APCA perceptual contrast between foreground and background colors (0-106)." => {
            "前景色和背景色之间的最小 APCA 感知对比度（0-106）。"
        }
        "The minimum APCA perceptual contrast to maintain when rendering text over highlight backgrounds." => {
            "在高亮背景上渲染文本时要保持的最小 APCA 感知对比度。"
        }
        "The minimum column at which to display inline diagnostics." => "显示内联诊断的最小列。",
        "The minimum column number at which to show the inline blame information." => {
            "显示内联 blame 信息的最小列号。"
        }
        "The minimum width (in columns) at which the split diff view is used. When the editor is narrower, the diff view automatically switches to unified mode. Set to 0 to disable." => {
            "使用分屏差异视图的最小宽度（列）。当编辑器更窄时，差异视图会自动切换为统一视图。设为 0 可禁用。"
        }
        "The multiplier for scrolling in the terminal with the mouse wheel" => {
            "使用鼠标滚轮在终端中滚动的倍数"
        }
        "The number of characters to keep on either side when scrolling with the mouse." => {
            "使用鼠标滚动时两侧保留的字符数。"
        }
        "The number of lines to keep above/below the cursor when auto-scrolling." => {
            "自动滚动时在光标上方/下方保留的行数。"
        }
        "The proxy to use for network requests." => "网络请求使用的代理。",
        "The shell program to run." => "要运行的 shell 程序。",
        "The shell program to use." => "要使用的 shell 程序。",
        "The text rendering mode to use." => "要使用的文本渲染模式。",
        "The theme to use when mode is set to dark, or when mode is set to system and it is in dark mode." => {
            "模式设为深色，或设为系统且系统处于深色模式时使用的主题。"
        }
        "The theme to use when mode is set to light, or when mode is set to system and it is in light mode." => {
            "模式设为浅色，或设为系统且系统处于浅色模式时使用的主题。"
        }
        "The unit for image file sizes." => "图片文件大小的单位。",
        "The width of the active indent guide in pixels, between 1 and 10." => {
            "活动缩进辅助线的宽度，单位像素，范围 1 到 10。"
        }
        "The width of the indent guides in pixels, between 1 and 10." => {
            "缩进辅助线的宽度，单位像素，范围 1 到 10。"
        }
        "Time to wait in milliseconds before hiding the hover popover after the mouse moves away." => {
            "鼠标移开后隐藏悬停弹窗前等待的时间（毫秒）。"
        }
        "Time to wait in milliseconds before showing the informational hover box." => {
            "显示信息悬停框前等待的时间（毫秒）。"
        }
        "Toggle relative line numbers in Vim mode." => "在 Vim 模式下切换相对行号。",
        "Toggles inlay hints (hides or shows) when the user presses the modifiers specified." => {
            "用户按下指定修饰键时切换内嵌提示（隐藏或显示）。"
        }
        "There are still conflicts. You must stage these before committing" => {
            "仍有冲突。提交前必须先暂存这些文件"
        }
        "Tab Close Position" => "标签关闭按钮位置",
        "Tab Show Diagnostics" => "标签页显示诊断",
        "Tab Size" => "制表符宽度",
        "Tab Whitespace Indicator" => "制表符空白标记",
        "Terminal" => "终端",
        "Terminal Button" => "终端按钮",
        "Terminal Dock" => "终端停靠位置",
        "Terminal Panel" => "终端面板",
        "Terminal Panel Flexible Sizing" => "终端面板弹性尺寸",
        "Thumb" => "滑块",
        "Thumb Border" => "滑块边框",
        "Title Override" => "标题覆盖",
        "Toggle Panel With" => "切换面板快捷键",
        "Top" => "顶部",
        "Uniform Height" => "统一高度",
        "Toggle All Docks" => "切换所有停靠栏",
        "Toggle Filters" => "切换过滤器",
        "Toggle Bottom Dock" => "切换底部停靠栏",
        "Toggle Folder" => "展开/折叠文件夹",
        "Toggle Left Dock" => "切换左侧停靠栏",
        "Toggle Line Comment" => "切换行注释",
        "Toggle On Modifiers Press" => "按下修饰键时切换",
        "Toggle Relative Line Numbers" => "切换相对行号",
        "Toggle Right Dock" => "切换右侧停靠栏",
        "Toggle Staged" => "切换暂存",
        "Tree View" => "树状视图",
        "Trust Directory" => "信任目录",
        "Trash" => "移到废纸篓",
        "Trash Untracked Files" => "将未跟踪文件移到废纸篓",
        "Trash these files?" => "要将这些文件移到废纸篓吗？",
        "Trust and Continue" => "信任并继续",
        "Type an action name" => "输入操作名称",
        "Trust all projects in parent directory" => "信任父目录中的所有项目",
        "Trust All Projects By Default" => "默认信任所有项目",
        "Troubleshoot and Quit" => "排查问题并退出",
        "Undo" => "撤销",
        "Update Debounce" => "更新防抖",
        "Update ready to install" => "更新已准备好安装",
        "Unsupported GPU" => "不支持的 GPU",
        "Unable to initialize a git repository" => "无法初始化 Git 仓库",
        "Unified" => "统一视图",
        "Unrecognized Workspace" => "未识别的工作区",
        "Unrecognized Project" => "未识别的项目",
        "Uncommitted Changes" => "未提交的变更",
        "Unnecessary Code Fade" => "非必要代码淡化",
        "Unstage" => "取消暂存",
        "Unstage All" => "全部取消暂存",
        "Unstage all changes" => "取消暂存所有变更",
        "Unstage File" => "取消暂存文件",
        "Unstage file" => "取消暂存文件",
        "Unstage and go to next hunk" => "取消暂存并跳转到下一个变更块",
        "Unfold Directory" => "展开目录",
        "Unlock File" => "解锁文件",
        "Unwrap Content" => "取消内容换行",
        "Use Both" => "两者都用",
        "Use LSP tasks over Zed language extension tasks." => {
            "优先使用 LSP 任务，而不是 Zed 语言扩展任务。"
        }
        "Use Auto Surround" => "使用自动包围",
        "Use Autoclose" => "使用自动闭合",
        "Use On Type Format" => "使用输入时格式化",
        "Use Smartcase Find" => "查找使用智能大小写",
        "Use Smartcase Search" => "搜索使用智能大小写",
        "Use System Clipboard" => "使用系统剪贴板",
        "Use System Window Tabs" => "使用系统窗口标签页",
        "Use gitignored files when searching." => "搜索时使用被 git 忽略的文件。",
        "Use native OS dialogs for 'Open' and 'Save As'." => {
            "“打开”和“另存为”使用操作系统原生对话框。"
        }
        "Use native OS dialogs for confirmations." => "确认操作使用操作系统原生对话框。",
        "Use regex search by default." => "默认使用正则搜索。",
        "Use regex search by default in Vim search." => "Vim 搜索默认使用正则搜索。",
        "Total tracked changes" => "所有已跟踪变更",
        "Visible character used to render space characters when show_whitespaces is enabled (default: \"•\")" => {
            "启用 show_whitespaces 时用于渲染空格字符的可见字符（默认：\"•\"）"
        }
        "Visible character used to render tab characters when show_whitespaces is enabled (default: \"→\")" => {
            "启用 show_whitespaces 时用于渲染制表符字符的可见字符（默认：\"→\"）"
        }
        "Variables" => "变量",
        "Version" => "版本",
        "Vertical Scroll Margin" => "垂直滚动边距",
        "Vertical Scrollbar" => "垂直滚动条",
        "Vertical Split Direction" => "垂直拆分方向",
        "Vim/Emacs Modeline Support" => "Vim/Emacs Modeline 支持",
        "Visibility" => "可见性",
        "View" => "视图",
        "View Branch Diff" => "查看分支差异",
        "View Changes" => "查看变更",
        "View Commit" => "查看提交",
        "View Default Keymap" => "查看默认快捷键",
        "View Diff" => "查看差异",
        "View File History" => "查看文件历史",
        "View Icon Theme Docs" => "查看图标主题文档",
        "View Log" => "查看日志",
        "View Logs" => "查看日志",
        "Variable Height" => "可变高度",
        "View Message" => "查看消息",
        "View History" => "查看历史",
        "View Stash" => "查看贮藏",
        "View Other Projects" => "查看其他项目",
        "View Theme Docs" => "查看主题文档",
        "Copy Commit SHA" => "复制提交 SHA",
        "Copy Tag" => "复制标签",
        "Welcome" => "欢迎",
        "Welcome back to Zed" => "欢迎回到 Zed",
        "Welcome to Zed" => "欢迎使用 Zed",
        "What shell to use when opening a terminal." => "打开终端时使用的 shell。",
        "What to do after closing the current tab." => "关闭当前标签页后的行为。",
        "What to do when multibuffer is double-clicked in some of its excerpts." => {
            "双击多缓冲区某些摘录时的行为。"
        }
        "What working directory to use when launching the terminal." => {
            "启动终端时使用的工作目录。"
        }
        "Warning Text" => "警告文本",
        "When enabled, the :substitute command replaces all matches in a line by default. The 'g' flag then toggles this behavior." => {
            "启用后，:substitute 命令默认替换一行中的所有匹配项。'g' 标志会切换此行为。"
        }
        "When enabled, use folding ranges from the language server instead of indent-based folding." => {
            "启用后，使用语言服务器提供的折叠范围，而不是基于缩进的折叠。"
        }
        "When enabled, use the language server's document symbols for outlines and breadcrumbs instead of tree-sitter." => {
            "启用后，使用语言服务器的文档符号生成大纲和面包屑，而不是 tree-sitter。"
        }
        "When false, forcefully disables the horizontal scrollbar." => {
            "为 false 时强制禁用水平滚动条。"
        }
        "When false, forcefully disables the vertical scrollbar." => {
            "为 false 时强制禁用垂直滚动条。"
        }
        "When fetching LSP completions, determines how long to wait for a response of a particular server (set to 0 to wait indefinitely)." => {
            "获取 LSP 补全时，决定等待特定服务器响应的时间（设为 0 表示无限等待）。"
        }
        "When opening Zed, avoid Restricted Mode by auto-trusting all projects, enabling use of all features without having to give permission to each new project." => {
            "打开 Zed 时自动信任所有项目以避免受限模式，从而无需为每个新项目授权即可使用所有功能。"
        }
        "When to auto save buffer changes." => "何时自动保存缓冲区更改。",
        "When to hide the mouse cursor." => "何时隐藏鼠标光标。",
        "When to populate a new search's query based on the text under the cursor." => {
            "何时根据光标下的文本填充新搜索的查询。"
        }
        "When to scan content of linked directories" => "何时扫描链接目录的内容",
        "When to show indent guides in the outline panel." => "何时在大纲面板中显示缩进辅助线。",
        "When to show the minimap in the editor." => "何时在编辑器中显示小地图。",
        "When to show the minimap thumb." => "何时显示小地图滑块。",
        "When to show the scrollbar in the completion menu." => "何时在补全菜单中显示滚动条。",
        "When to show the scrollbar in the editor." => "何时在编辑器中显示滚动条。",
        "When to show the scrollbar in the terminal." => "何时在终端中显示滚动条。",
        "Where to dock the Git panel." => "Git 面板停靠位置。",
        "Where to dock the outline panel." => "大纲面板停靠位置。",
        "Where to dock the project panel." => "项目面板停靠位置。",
        "Where to dock the terminal panel." => "终端面板停靠位置。",
        "Where to show the minimap in the editor." => "编辑器中小地图的显示位置。",
        "Wrapping Text" => "换行文本",
        "Whether and how to display code lenses from language servers." => {
            "是否以及如何显示来自语言服务器的代码透镜。"
        }
        "Whether alternate scroll mode is active by default (converts mouse scroll to arrow keys in apps like Vim)." => {
            "备用滚动模式是否默认启用（在 Vim 等应用中将鼠标滚动转换为方向键）。"
        }
        "Whether or not to debounce inlay hints updates after buffer edits (set to 0 to disable debouncing)." => {
            "缓冲区编辑后是否对内嵌提示更新进行防抖（设为 0 可禁用防抖）。"
        }
        "Whether or not to debounce inlay hints updates after buffer scrolls (set to 0 to disable debouncing)." => {
            "缓冲区滚动后是否对内嵌提示更新进行防抖（设为 0 可禁用防抖）。"
        }
        "Whether selecting text in the terminal automatically copies to the system clipboard." => {
            "在终端中选择文本是否自动复制到系统剪贴板。"
        }
        "Whether the file finder should skip focus for the active file in search results." => {
            "文件查找器是否在搜索结果中跳过当前活动文件的焦点。"
        }
        "Whether the hover popover sticks when the mouse moves toward it, allowing interaction with its contents." => {
            "鼠标移向悬停弹窗时弹窗是否保持，以允许与内容交互。"
        }
        "Whether the project panel should open on startup." => "项目面板是否在启动时打开。",
        "Whether the terminal panel should use flexible (proportional) sizing when docked to the left or right." => {
            "终端面板停靠在左侧或右侧时是否使用弹性（按比例）尺寸。"
        }
        "Whether to allow horizontal scrolling in the project panel. When disabled, the view is always locked to the leftmost position and long file names are clipped." => {
            "是否允许项目面板水平滚动。禁用时，视图始终锁定在最左侧，长文件名会被截断。"
        }
        "Whether to automatically enable case-sensitive search based on the search query." => {
            "是否根据搜索查询自动启用区分大小写搜索。"
        }
        "Whether to automatically open files after pasting or duplicating them." => {
            "粘贴或复制文件后是否自动打开。"
        }
        "Whether to automatically open files dropped from external sources." => {
            "从外部来源拖放文件后是否自动打开。"
        }
        "Whether to automatically open newly created files in the editor." => {
            "是否在编辑器中自动打开新建文件。"
        }
        "Whether to automatically surround text with characters for you. For example, when you select text and type '(', Zed will automatically surround text with ()." => {
            "是否自动用字符包围文本。例如，选中文本后输入 '('，Zed 会自动用 () 包围文本。"
        }
        "Whether to automatically type closing characters for you. For example, when you type '(', Zed will automatically add a closing ')' at the correct position." => {
            "是否自动输入闭合字符。例如，输入 '(' 时，Zed 会在正确位置自动添加闭合的 ')'。"
        }
        "Whether to center the current match in the editor" => "是否在编辑器中居中当前匹配项",
        "Whether to change focus to a pane when the mouse hovers over it." => {
            "鼠标悬停在窗格上时是否将焦点切换到该窗格。"
        }
        "Whether to collapse untracked files in the diff panel." => {
            "是否在差异面板中折叠未跟踪文件。"
        }
        "Whether to enable drag-and-drop operations in the project panel." => {
            "是否在项目面板中启用拖放操作。"
        }
        "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside." => {
            "当目录内只有一个子目录时，是否自动折叠目录并显示紧凑文件夹。"
        }
        "Whether to fold directories automatically when a directory contains only one subdirectory." => {
            "当目录只包含一个子目录时是否自动折叠目录。"
        }
        "Whether to hide the gitignore entries in the project panel." => {
            "是否在项目面板中隐藏 gitignore 条目。"
        }
        "Whether to hide the hidden entries in the project panel." => {
            "是否在项目面板中隐藏隐藏条目。"
        }
        "Whether to hide the root entry when only one folder is open in the window." => {
            "窗口中只打开一个文件夹时是否隐藏根条目。"
        }
        "Whether to keep tabs in preview mode when code navigation is used to navigate away from them. If `enable_preview_file_from_code_navigation` or `enable_preview_multibuffer_from_code_navigation` is also true, the new tab may replace the existing one." => {
            "使用代码导航离开标签页时，是否让标签页保持预览模式。如果 `enable_preview_file_from_code_navigation` 或 `enable_preview_multibuffer_from_code_navigation` 也为 true，新标签页可能替换现有标签页。"
        }
        "Whether to keep the text selection after copying it to the clipboard." => {
            "复制到剪贴板后是否保留文本选区。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a multibuffer." => {
            "使用代码导航打开多缓冲区时，是否以预览模式打开标签页。"
        }
        "Whether to open tabs in preview mode when code navigation is used to open a single file." => {
            "使用代码导航打开单个文件时，是否以预览模式打开标签页。"
        }
        "Whether to open tabs in preview mode when opened from a multibuffer." => {
            "从多缓冲区打开标签页时，是否以预览模式打开。"
        }
        "Whether to open tabs in preview mode when opened from the project panel with a single click." => {
            "从项目面板单击打开标签页时，是否以预览模式打开。"
        }
        "Whether to open tabs in preview mode when selected from the file finder." => {
            "从文件查找器选择文件时，是否以预览模式打开标签页。"
        }
        "Whether to perform linked edits of associated ranges, if the LS supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well." => {
            "如果语言服务器支持，是否对关联范围执行联动编辑。例如，编辑开始 <html> 标签时，结束 </html> 标签内容也会同步编辑。"
        }
        "Whether to play a sound when the BEL character (`\\a`, `0x07`) is printed" => {
            "打印 BEL 字符（`\\a`，`0x07`）时是否播放声音"
        }
        "Whether to pull for language server-powered diagnostics or not." => {
            "是否拉取由语言服务器提供的诊断。"
        }
        "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active." => {
            "对应项目条目变为活动状态时，是否自动在项目面板中显示该条目。"
        }
        "Whether to reveal when a corresponding outline entry becomes active." => {
            "对应大纲条目变为活动状态时是否显示。"
        }
        "Whether to show a badge on the git panel icon with the count of uncommitted changes." => {
            "是否在 Git 面板图标上显示未提交变更数量徽标。"
        }
        "Whether to show folder icons or chevrons for directories in the git panel." => {
            "是否在 Git 面板中为目录显示文件夹图标或折叠箭头。"
        }
        "Whether to show folder icons or chevrons for directories in the outline panel." => {
            "是否在大纲面板中为目录显示文件夹图标或折叠箭头。"
        }
        "Whether to show folder icons or chevrons for directories in the project panel." => {
            "是否在项目面板中为目录显示文件夹图标或折叠箭头。"
        }
        "Whether to show the addition/deletion change count next to each file in the Git panel." => {
            "是否在 Git 面板中每个文件旁显示新增/删除变更数量。"
        }
        "Whether to sort file and folder names case-sensitively in the project panel." => {
            "是否在项目面板中按大小写敏感方式排序文件和文件夹名称。"
        }
        "Whether to stick parent directories at top of the project panel." => {
            "是否将父目录粘滞在项目面板顶部。"
        }
        "Whether to stick scopes to the top of the editor" => "是否将作用域粘滞在编辑器顶部",
        "Whether to use additional LSP queries to format (and amend) the code after every \"trigger\" symbol input, defined by LSP server capabilities" => {
            "是否在每次输入由 LSP 服务器能力定义的“触发”符号后，使用额外 LSP 查询格式化（并修正）代码"
        }
        "Whether indentation of pasted content should be adjusted based on the context." => {
            "是否根据上下文调整粘贴内容的缩进。"
        }
        "Whether or not to ensure there's a single newline at the end of a buffer when saving it." => {
            "保存时是否确保缓冲区末尾有一个换行符。"
        }
        "Whether or not to perform a buffer format before saving." => "保存前是否格式化缓冲区。",
        "Whether or not to remove any trailing whitespace from lines of a buffer before saving it." => {
            "保存前是否移除缓冲区各行的行尾空白。"
        }
        "Whether or not to show Git blame data inline in the currently focused line." => {
            "是否在当前聚焦行内联显示 Git blame 数据。"
        }
        "Whether other hints should be shown." => "是否显示其他提示。",
        "Whether parameter hints should be shown." => "是否显示参数提示。",
        "Whether tasks are enabled for this language." => "此语言是否启用任务。",
        "Whether the cursor blinks in the editor." => "编辑器中的光标是否闪烁。",
        "Whether the editor search results will loop." => "编辑器搜索结果是否循环。",
        "Whether the editor will scroll beyond the last line." => "编辑器是否可滚动超过最后一行。",
        "Whether the option key behaves as the meta key." => "Option 键是否作为 Meta 键。",
        "Whether the text selection should have rounded corners." => "文本选区是否使用圆角。",
        "Whether to align detail text in code completions context menus left or right." => {
            "代码补全上下文菜单中的详情文本左对齐还是右对齐。"
        }
        "Whether to automatically close JSX tags." => "是否自动闭合 JSX 标签。",
        "Whether to colorize brackets in the editor." => "是否在编辑器中为括号着色。",
        "Whether to display inline and alongside documentation for items in the completions menu." => {
            "是否为补全菜单项显示内联和侧边文档。"
        }
        "Whether to enable word diff highlighting in the editor. When enabled, changed words within modified lines are highlighted to show exactly what changed." => {
            "是否在编辑器中启用词级差异高亮。启用后，修改行中的变更单词会被高亮以精确显示变化。"
        }
        "Whether to fetch LSP completions or not." => "是否获取 LSP 补全。",
        "Whether to follow-up empty Go to definition responses from the language server." => {
            "是否跟进语言服务器返回的空跳转定义结果。"
        }
        "Whether to indent lines using tab characters, as opposed to multiple spaces." => {
            "是否使用制表符缩进行，而不是多个空格。"
        }
        "Whether to pop the completions menu while typing in an editor without explicitly requesting it." => {
            "在编辑器中输入时，是否无需显式请求就弹出补全菜单。"
        }
        "Whether to scroll when clicking near the edge of the visible text area." => {
            "点击可见文本区域边缘附近时是否滚动。"
        }
        "Whether to show diagnostics inline or not." => "是否内联显示诊断。",
        "Whether to show folder names with bold text in the project panel." => {
            "是否在项目面板中用粗体显示文件夹名称。"
        }
        "Whether to show tabs and spaces in the editor." => "是否在编辑器中显示制表符和空格。",
        "Whether to show the stage and restore buttons on diff hunks." => {
            "是否在差异变更块上显示暂存和还原按钮。"
        }
        "Whether to show warnings or not by default." => "默认是否显示警告。",
        "Whether to start a new line with a comment when a previous line is a comment as well." => {
            "上一行也是注释时，换行是否自动以注释开始。"
        }
        "Whether to use language servers to provide code intelligence." => {
            "是否使用语言服务器提供代码智能。"
        }
        "Whether to zoom the editor font size with the mouse wheel while holding the primary modifier key." => {
            "按住主修饰键滚动鼠标滚轮时，是否缩放编辑器字号。"
        }
        "Whether type hints should be shown." => "是否显示类型提示。",
        "Which diagnostic indicators to show in the scrollbar." => "在滚动条中显示哪些诊断指示器。",
        "Which files containing diagnostic errors/warnings to mark in the project panel." => {
            "在项目面板中标记哪些包含诊断错误/警告的文件。"
        }
        "Which files containing diagnostic errors/warnings to mark in the tabs." => {
            "在标签页中标记哪些包含诊断错误/警告的文件。"
        }
        "Which level to use to filter out diagnostics displayed in the editor." => {
            "用于过滤编辑器中显示诊断的级别。"
        }
        "Which settings should be activated only in Preview build of Zed." => {
            "哪些设置只应在 Zed Preview 构建中激活。"
        }
        "Untrusted workspaces are opened in Restricted Mode to protect your system. Review .zed/settings.json for any extensions or commands configured by this project." => {
            "不受信任的工作区会以受限模式打开以保护你的系统。请检查 .zed/settings.json 中此项目配置的扩展或命令。"
        }
        "Untrusted projects are opened in Restricted Mode to protect your system." => {
            "不受信任的项目会以受限模式打开以保护你的系统。"
        }
        "Review .zed/settings.json for any extensions or commands configured by this project." => {
            "请检查 .zed/settings.json 中此项目配置的扩展或命令。"
        }
        "Window" => "窗口",
        "Window Decorations" => "窗口装饰",
        "Whole Word" => "完整单词",
        "Word Diff Enabled" => "启用词级差异",
        "Words" => "单词",
        "Words Min Length" => "单词最小长度",
        "Working Directory" => "工作目录",
        "Wrap Content" => "内容换行",
        "Wrap Guides" => "换行参考线",
        "XLarge Headline" => "特大标题",
        "XSmall Headline" => "特小标题",
        "Zoomed Padding" => "缩放内边距",
        "Yes" => "是",
        "You have outdated settings" => "你的设置已过期",
        "`zed/new-notification-system` created!" => "`zed/new-notification-system` 已创建！",
        "Zed Repository" => "Zed 仓库",
        "Zed failed to launch" => "Zed 启动失败",
        "Zed is running from a temporary location. Move it to Applications to finish installing it." => {
            "Zed 正在临时位置运行。将它移到 Applications 以完成安装。"
        }
        "Zed will reopen when installation is complete." => "安装完成后 Zed 会重新打开。",
        "Zoom" => "缩放",
        "Zoom In" => "放大",
        "Zoom Out" => "缩小",
        "Deleting…" => "正在删除…",
        "Enter a name for this remote…" => "输入此远程的名称…",
        "Enter git ref..." => "输入 Git 引用...",
        "Enter repository URL…" => "输入仓库 URL…",
        "Execute a command..." => "执行命令...",
        "Exit" => "退出",
        "Filter action names…" => "过滤操作名称…",
        "Filter events..." => "过滤事件...",
        "Fit to View" => "适应视图",
        "Find a task, or run a command" => "查找任务，或运行命令",
        "Find a task, or run a command in the central pane" => "查找任务，或在中央窗格运行命令",
        "Hit enter to search. For more options:" => "按 Enter 搜索。更多选项：",
        "Include/exclude specific paths" => "包含/排除特定路径",
        "Loading project…" => "正在加载项目…",
        "No matches" => "无匹配项",
        "No more matches" => "没有更多匹配项",
        "No server selected" => "未选择服务器",
        "No tabs" => "没有标签页",
        "or" => "或",
        "Remote name can't be empty" => "远程名称不能为空",
        "Reopen with encoding..." => "使用编码重新打开...",
        "Replace in project…" => "在项目中替换…",
        "Replace with…" => "替换为…",
        "Replace" => "替换",
        "Search all files…" => "搜索所有文件…",
        "Search all tabs…" => "搜索所有标签页…",
        "Search buffer symbols..." => "搜索缓冲区符号...",
        "Search buffer symbols…" => "搜索缓冲区符号…",
        "Search commits…" => "搜索提交…",
        "Search fonts…" => "搜索字体…",
        "Search icon themes…" => "搜索图标主题…",
        "Search project files..." => "搜索项目文件...",
        "Search project symbols..." => "搜索项目符号...",
        "Search projects…" => "搜索项目…",
        "Search settings…" => "搜索设置…",
        "Search theme…" => "搜索主题…",
        "Search…" => "搜索…",
        "Save file to change encoding" => "保存文件后才能更改编码",
        "Select a language…" => "选择语言…",
        "Select a repository..." => "选择仓库...",
        "Select a settings profile..." => "选择设置配置档...",
        "Select a stash…" => "选择贮藏…",
        "Select a worktree…" => "选择工作树…",
        "Select branch…" => "选择分支…",
        "Select a line ending…" => "选择换行符…",
        "Select snippet scope..." => "选择代码片段作用域...",
        "Switch branch…" => "切换分支…",
        "enabled for all" => "已对所有用户启用",
        "You may need to configure git for Github." => "你可能需要配置 GitHub 的 git 设置。",
        "Action Arguments" => "操作参数",
        "New..." => "新建...",
        "New…" => "新建…",
        _ => return None,
    })
}
