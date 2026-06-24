# 10 开发进度记录

## 1. 文档目的

本文档用于记录 Nvy 第一版 MVP 开发过程中的实际进度、已修改文件、验证结果和阻塞项。

它不替代 `docs/09_mvp_development_plan.md`。开发计划负责说明“接下来应该怎么做”，本文档负责记录“已经做了什么、验证到了什么程度、还卡在哪里”。

## 2. 记录规则

每次开始或完成一个开发任务时，记录以下信息：

1. 任务编号和任务名称。
2. 当前状态：未开始 / 进行中 / 已完成 / 阻塞 / 待复验。
3. 实际修改或创建的文件。
4. 已执行的验证命令。
5. 验证结果。
6. 未完成事项或阻塞原因。
7. 下一步建议。

## 3. 当前总览

| 任务 | 名称 | 状态 | 说明 |
|---|---|---|---|
| 0.1 | 初始化 Tauri + React + TypeScript 工程 | 已完成 | 前端骨架和 Tauri 配置已创建；前端构建通过；Rust 侧 `cargo check` 通过；`tauri:dev` 已成功启动 Nvy 空壳窗口 |
| 0.2 | 接入 Tailwind CSS、Radix UI primitives 和 CSS 变量主题骨架 | 已完成 | Tailwind 4、Radix Dialog / Dropdown / Tooltip、基础 Button / Input、CSS 变量主题已接入；前端构建、Rust 检查和 Tauri 窗口启动验证通过 |
| 0.3 | 建立目录边界 | 已完成 | `src/app`、`src/pages`、`src/components`、`src/domain`、`src/services`、`src/lib`、`src-tauri/src/commands` 边界已建立；页面不直接调用 Tauri 或 SQLite |
| 0.4 | 建立统一 `desktopApi` 适配层 | 已完成 | 前端已通过 `src/services/desktopApi` 封装 Tauri command；Rust 侧已注册 app 和 database 命令 |
| 0.5 | 建立最小测试基线 | 已完成 | 已接入 Vitest，新增纯 TypeScript 单元测试；`pnpm test` 通过 |
| 1.1 | 设计 SQLite schema 与迁移机制 | 已完成 | 已新增 SQLite 初始迁移、Rust 初始化逻辑和幂等测试；`cargo test` 通过 |
| 1.2 | 实现本地账号表和密码哈希 | 已完成 | 已实现账号注册、输入校验和 Argon2 密码哈希 |
| 1.3 | 实现登录和当前账号上下文 | 已完成 | 已实现登录校验和前端当前账号上下文 |
| 1.4 | 实现账号数据隔离基础查询 | 已完成 | 已建立按 `account_id` 隔离的后端查询基础 |
| 1.5 | 初始化默认账号设置和默认提示词 | 已完成 | 新账号自动拥有默认推荐参考数 30、默认推荐提示词和翻译提示词 |
| 1.6 | 初始化预置标签库 | 已完成 | 新账号自动拥有影片预置标签和女优预置标签 |
| 2.1 | 登录 / 注册页 | 已完成 | 已提供最小登录 / 注册页，错误提示清楚，不暴露底层错误 |
| 2.2 | 登录态路由保护 | 已完成 | 已建立最小路由保护，未登录只能看到登录页 |
| 2.3 | 全局左侧折叠状态栏 | 已完成 | 已建立登录后全局左侧折叠状态栏，默认折叠，hover 有 tooltip，点击 logo 可展开，展开后显示中文按钮名和账号名；暂只接入首页占位壳 |
| UI-1 | 按 `docs/08_page_wireframes.md` 启动 UI 页面重构 | 进行中 | 已完成第一轮结构性重构：16:9 舞台、菜单栏固定、首页双层弧线轮播、影片库 / 女优库与详情页分离、库页顶部工具条和网格重做；仍需人工视觉回归和后续细节打磨 |

## 4. 任务记录

## 任务 UI-1 按 `docs/08_page_wireframes.md` 启动 UI 页面重构

- 开始时间：2026-06-23
- 当前状态：进行中
- 任务目标：遵循 `docs/08_page_wireframes.md`，先完成 UI 结构性重构，解决当前实现与用户草图和文字规则差异过大的问题。

### 已修改的文件

| 文件 | 作用 |
|---|---|
| `src/app/App.tsx` | 调整详情页状态，点击库页项目后保持独立详情视图；切换导航或返回时清空详情状态 |
| `src/components/layout/AppShell.tsx` | 建立登录后全局 16:9 应用舞台 |
| `src/components/layout/Sidebar.tsx` | 菜单栏改为舞台内固定定位，展开时不推动页面内容；固定 logo、图标、头像尺寸 |
| `src/components/ui/Tooltip.tsx` | tooltip 固定从右侧弹出 |
| `src/pages/HomePage.tsx` | 删除首页标题和轮播文字，改为双层弧线轮播和 AI Studio 风格底部输入框 |
| `src/pages/VideosPage.tsx` | 影片库改为独立列表页；点击封面进入独立影片详情页；顶部工具条和 3 列封面网格重做 |
| `src/pages/ActressesPage.tsx` | 女优库改为独立列表页；点击头像进入独立女优详情页；顶部工具条、Cup 下拉、5 列头像网格重做 |
| `src/styles.css` | 增加 16:9 舞台、舞台外暗色氛围背景、页面滚动和顶部渐隐遮罩 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过；8 个测试文件、25 个测试用例全部通过 |
| `corepack pnpm build` | 通过；`tsc --noEmit` 和 `vite build` 均成功 |
| 启动 `corepack pnpm dev --host 127.0.0.1 --port 1420` | 通过；`http://127.0.0.1:1420/` 返回 200 |

### 未完成事项

1. 浏览器自动化环境当前返回 `Browser is not available: iab`，尚未完成截图级视觉回归。
2. 详情页字段布局已经脱离库页侧栏，但仍需要继续按 `K1/K2` 和页面草图做更细的视觉打磨。
3. 设置页尚未按新版 16:9 舞台做专项视觉整理。
4. 需要用户在真实 Tauri 桌面窗口中人工查看首页、影片库、女优库、详情页的比例、位置、滚动和菜单栏行为。

### 下一步建议

1. 先在 Tauri 桌面窗口人工回归首页、影片库、女优库、影片详情、女优详情。
2. 根据视觉回归结果继续修正组件尺寸、间距、轮播位置和详情页字段布局。
3. 在浏览器工具可用后补截图级回归。

## 任务 0.1 初始化 Tauri + React + TypeScript 工程

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：创建最小 Tauri + React + TypeScript + pnpm 工程骨架，只实现可启动的 Nvy 空壳窗口，不接数据库，不实现账号、影片、女优、大模型或导入导出业务。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `.gitignore` | 忽略 `node_modules/`、`.pnpm-store/`、`dist/`、Rust target 等生成物 |
| `package.json` | 定义 pnpm 工程、React/Vite/Tauri 依赖和基础脚本 |
| `pnpm-lock.yaml` | pnpm 锁文件 |
| `index.html` | Vite 前端入口 HTML |
| `tsconfig.json` | TypeScript 配置 |
| `vite.config.ts` | Vite + React 配置，开发端口为 1420 |
| `src/main.tsx` | React 应用入口 |
| `src/App.tsx` | 最小 Nvy 空壳界面 |
| `src/styles.css` | 最小暗色视觉样式 |
| `src/app/.gitkeep` | 预留应用层目录 |
| `src/components/.gitkeep` | 预留组件目录 |
| `src/domain/.gitkeep` | 预留领域规则目录 |
| `src/lib/.gitkeep` | 预留通用工具目录 |
| `src/pages/.gitkeep` | 预留页面目录 |
| `src/services/.gitkeep` | 预留服务目录 |
| `src-tauri/Cargo.toml` | Tauri Rust 工程依赖配置 |
| `src-tauri/build.rs` | Tauri 构建脚本 |
| `src-tauri/src/main.rs` | Tauri 程序入口 |
| `src-tauri/src/lib.rs` | Tauri 应用启动逻辑 |
| `src-tauri/tauri.conf.json` | Tauri 应用配置 |
| `src-tauri/capabilities/default.json` | Tauri 默认权限配置 |
| `src-tauri/icons/icon.ico` | Tauri Windows 资源所需的最小占位图标 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm install` | 通过；依赖安装成功，生成 `pnpm-lock.yaml` |
| `corepack pnpm build` | 通过；`tsc --noEmit` 和 `vite build` 均成功 |
| `corepack pnpm tauri info` | 通过；WebView2、MSVC、Rust、Cargo、rustup 和 Tauri 配置可识别 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 未通过；需要访问 crates.io 下载 Rust 依赖，当前环境返回 SSL / 凭据相关错误 |
| 2026-06-22 再次执行 `cargo check --manifest-path src-tauri/Cargo.toml` | 仍未通过；错误仍为 crates.io `config.json` 下载失败，schannel 返回 `SEC_E_NO_CREDENTIALS` |
| 授权模式执行 `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；Rust/Tauri 依赖下载完成并完成编译检查 |
| `corepack pnpm tauri:dev` | 通过；Vite 启动在 `http://127.0.0.1:1420/`，Tauri 编译完成并运行 `target\debug\nvy.exe` |
| `Get-Process -Name nvy` | 通过；确认存在 `nvy.exe` 进程 |

### 当前阻塞

已解决。第一次执行 `cargo check --manifest-path src-tauri/Cargo.toml` 时，Cargo 无法从 crates.io 下载依赖，错误表现为 SSL / 凭据相关失败。

已尝试申请授权模式重试，但审批系统返回工作区额度不足，命令未执行。按照安全规则，不能通过绕过审批的方式继续下载依赖。

2026-06-22 再次直接执行同一命令，错误仍然存在，说明当前环境仍无法完成 Rust 依赖下载。

后续用户要求继续任务时，授权模式执行 `cargo check` 成功下载依赖并通过编译检查。随后执行 `corepack pnpm tauri:dev` 成功启动 Nvy 空壳窗口。

### 当前判断

任务 0.1 已完成。当前只包含最小工程骨架和空壳界面，没有实现账号、数据库、影片、女优、大模型、元数据匹配或导入导出业务。

补充说明：`tauri:dev` 验证结束后，为避免后台进程继续占用端口和窗口，已手动停止本次验证启动的 `nvy.exe`。因此开发会话最后显示生命周期命令退出，但窗口启动验证已完成。

### 下一步建议

1. 可以进入任务 0.2：接入 Tailwind CSS、Radix UI primitives 和 CSS 变量主题骨架。
2. 也可以按计划进入阶段 1 前先补充最小测试基线。

## 任务 0.2 接入 Tailwind CSS、Radix UI primitives 和 CSS 变量主题骨架

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：接入已确认的 Tailwind CSS + Radix UI primitives + CSS 变量 UI 基础方案，建立暗色主题、基础按钮、输入框、弹窗、下拉栏和 tooltip 骨架，不实现任何业务功能。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `package.json` | 新增 Tailwind CSS、Tailwind Vite 插件和 Radix UI primitives 依赖 |
| `pnpm-lock.yaml` | 更新依赖锁文件 |
| `vite.config.ts` | 接入 `@tailwindcss/vite` 插件 |
| `src/styles.css` | 接入 Tailwind 入口、CSS 变量主题、暗色背景、文本和状态色 |
| `src/lib/cn.ts` | 提供最小 className 拼接工具 |
| `src/components/ui/Button.tsx` | 基础按钮组件，支持 primary / secondary / ghost / danger |
| `src/components/ui/Input.tsx` | 暗色圆角输入框组件 |
| `src/components/ui/Dialog.tsx` | 基于 Radix Dialog 的基础弹窗 |
| `src/components/ui/Dropdown.tsx` | 基于 Radix Dropdown Menu 的基础下拉栏 |
| `src/components/ui/Tooltip.tsx` | 基于 Radix Tooltip 的基础悬浮提示 |
| `src/App.tsx` | 更新空壳页面，用于预览主题和基础 UI primitives |

### 已安装依赖

| 依赖 | 用途 |
|---|---|
| `tailwindcss` | Tailwind CSS 样式系统 |
| `@tailwindcss/vite` | Vite 中启用 Tailwind CSS |
| `@radix-ui/react-dialog` | 可访问弹窗基础能力 |
| `@radix-ui/react-dropdown-menu` | 可访问下拉菜单基础能力 |
| `@radix-ui/react-tooltip` | 可访问 tooltip 基础能力 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm add @radix-ui/react-dialog @radix-ui/react-dropdown-menu @radix-ui/react-tooltip` | 通过；Radix primitives 安装完成 |
| `corepack pnpm add -D tailwindcss @tailwindcss/vite` | 通过；Tailwind CSS 和 Vite 插件安装完成 |
| `corepack pnpm build` | 通过；TypeScript 和 Vite 生产构建成功 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；Rust/Tauri 桌面壳检查成功 |
| `corepack pnpm tauri:dev` | 通过；Tauri 窗口启动，能加载更新后的 UI 空壳页面 |
| `Get-Process -Name nvy` | 通过；确认存在 `nvy.exe` 进程 |

### 当前阻塞

无。

### 当前判断

任务 0.2 已完成。当前 UI 基础设施只用于空壳预览，没有实现登录、资料库、数据库、大模型、元数据匹配或导入导出业务。

补充说明：`tauri:dev` 验证结束后，为避免后台进程继续占用端口和窗口，已手动停止本次验证启动的 `nvy.exe`。因此开发会话最后显示生命周期命令退出，但窗口启动验证已完成。

### 下一步建议

1. 按计划进入任务 0.3：建立目录边界。
2. 或进入任务 0.5：建立最小测试基线，便于后续纯逻辑模块测试。

## 任务 0.3 建立目录边界

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立前端应用层、页面层、组件层、领域规则层、服务层和 Tauri 命令层的清晰边界，避免后续页面组件直接依赖 SQLite、文件系统或 Tauri 细节。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/app/App.tsx` | 应用层根组件，负责组合当前页面 |
| `src/App.tsx` | 保留根层重导出，兼容已有导入习惯 |
| `src/main.tsx` | 改为从 `src/app/App.tsx` 导入应用根组件 |
| `src/pages/AppShellPreview.tsx` | 把空壳预览移动到页面层 |
| `src/components/layout/PreviewPanel.tsx` | 新增布局组件，承载预览面板外观 |
| `src/app/README.md` | 记录应用层职责 |
| `src/pages/README.md` | 记录页面层职责 |
| `src/components/README.md` | 记录组件层职责 |
| `src/domain/README.md` | 记录领域规则层职责 |
| `src/services/README.md` | 记录服务层职责 |
| `src/services/desktopApi/README.md` | 预留统一桌面 API 适配层 |
| `src-tauri/src/commands/README.md` | 预留 Tauri command 模块目录 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm build` | 通过；TypeScript 和 Vite 生产构建成功 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；Rust/Tauri 桌面壳检查成功 |

### 当前阻塞

无。

### 当前判断

任务 0.3 已完成。当前只是建立目录和职责边界，没有实现业务规则、数据访问、账号、资料库、大模型或导入导出。

### 下一步建议

1. 进入任务 0.4：建立统一 `desktopApi` 适配层。
2. 随后进入任务 0.5：建立最小测试基线。

## 任务 0.4 建立统一 `desktopApi` 适配层

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立前端统一桌面能力访问入口，避免页面组件直接散落调用 Tauri API，并为后续迁移 Electron 保留边界。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/services/desktopApi/index.ts` | 封装 `app_info` 和 `initialize_database` Tauri command |
| `src/services/desktopApi/types.ts` | 定义前端桌面 API 返回类型 |
| `src-tauri/src/commands/mod.rs` | Rust command 模块入口 |
| `src-tauri/src/commands/app.rs` | 提供 `app_info` 命令 |
| `src-tauri/src/commands/database.rs` | 提供 `initialize_database` 命令 |
| `src-tauri/src/lib.rs` | 注册 Tauri command handler |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm build` | 通过；前端类型检查和构建成功 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；Rust/Tauri command 编译成功 |

### 当前阻塞

无。

### 当前判断

任务 0.4 已完成。当前只建立命令桥接和适配层，不包含业务页面调用，也没有实现账号或资料库 CRUD。

### 下一步建议

1. 后续页面只通过 `desktopApi` 调用桌面能力。
2. 后续新增 Tauri 命令时同步扩展 `src/services/desktopApi/`。

## 任务 0.5 建立最小测试基线

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立最小可运行的 TypeScript 单元测试基线，后续搜索、标签、姓名归一化等纯逻辑模块可以持续加测试。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `package.json` | 新增 `test` 脚本和 `vitest` 开发依赖 |
| `pnpm-lock.yaml` | 更新依赖锁文件 |
| `src/domain/appMetadata.ts` | 新增纯 TypeScript 示例逻辑 |
| `src/domain/appMetadata.test.ts` | 新增 Vitest 单元测试 |
| `docs/03_tech_stack.md` | 同步记录 Vitest 已用于纯 TypeScript 单元测试 |
| `docs/07_decision_log.md` | 记录采用 Vitest 的实现决策 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过；1 个测试文件、2 个测试用例通过 |
| `corepack pnpm build` | 通过；前端类型检查和构建成功 |

### 当前阻塞

无。

### 当前判断

任务 0.5 已完成。当前测试只覆盖最小纯逻辑示例，不涉及 UI、Tauri、SQLite 或业务流程。

### 下一步建议

1. 后续实现搜索语法、标签规则、姓名归一化、年龄标签计算时优先补充 Vitest 测试。
2. Playwright 暂不接入，等页面流程更稳定后再处理端到端测试。

## 任务 1.1 设计 SQLite schema 与迁移机制

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立第一版 SQLite 初始 schema、迁移记录表和幂等初始化机制，为账号、影片、女优、关联、标签、设置和元数据候选提供数据结构基础。

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/Cargo.toml` | 新增 `rusqlite`，启用 `bundled` 特性 |
| `src-tauri/migrations/001_initial_schema.sql` | 第一版初始数据库 schema |
| `src-tauri/src/db/mod.rs` | 数据库路径、初始化、迁移执行和 Rust 单元测试 |
| `src-tauri/src/commands/database.rs` | 暴露 `initialize_database` Tauri command |
| `src/services/desktopApi/index.ts` | 提供前端 `initializeDatabase` 适配函数 |
| `src/services/desktopApi/types.ts` | 定义 `DatabaseStatus` 类型 |
| `docs/03_tech_stack.md` | 同步记录 `rusqlite` 作为 SQLite 访问库 |
| `docs/07_decision_log.md` | 记录采用 `rusqlite` 的实现决策 |

### 已覆盖的数据表

| 表 | 作用 |
|---|---|
| `schema_migrations` | 记录已应用迁移版本 |
| `accounts` | 本地账号基础表 |
| `account_settings` | 当前账号非敏感设置 |
| `videos` | 影片资料表 |
| `actresses` | 女优资料表 |
| `video_actresses` | 影片和女优关联表 |
| `tag_library` | 影片 / 女优标签库 |
| `video_tags` | 影片标签关联 |
| `actress_tags` | 女优标签关联 |
| `metadata_candidates` | 元数据匹配候选缓存表 |

### 已执行验证

| 命令 | 结果 |
|---|---|
| 授权模式执行 `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；下载并编译 `rusqlite` / bundled SQLite 依赖 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通过；无警告 |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 通过；数据库初始化幂等测试通过 |
| `corepack pnpm build` | 通过；前端类型检查和构建成功 |

### 当前阻塞

无。

### 当前判断

任务 1.1 已完成。当前只实现 schema、迁移和初始化命令，没有实现账号注册、登录、资料 CRUD、默认设置写入、预置标签写入或 Stronghold。

### 下一步建议

1. 进入任务 1.2：实现本地账号表和密码哈希。
2. 在实现账号功能前先确认密码哈希库选择，并同步记录依赖决策。

## 5. 后续记录模板

```markdown
## 任务 X.X 任务名称

- 开始时间：
- 完成时间：
- 当前状态：
- 任务目标：

### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|

### 已执行验证

| 命令 | 结果 |
|---|---|

### 当前阻塞

无 / 具体阻塞原因。

### 下一步建议

1. ...
```

## 6. 2026-06-22 补充进度记录

### 任务 1.2 本地账号表和密码哈希

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：在既有 `accounts` 表基础上实现本地账号注册、输入校验和密码哈希保存，不保存明文密码。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/Cargo.toml` | 新增 `argon2`、`rand_core`、`uuid` 依赖 |
| `src-tauri/Cargo.lock` | 锁定新增 Rust 依赖版本 |
| `src-tauri/src/auth/mod.rs` | 新增账号注册、用户名 / 密码校验、Argon2 密码哈希和单元测试 |
| `src-tauri/src/commands/auth.rs` | 新增 `register_account` Tauri command |
| `src-tauri/src/commands/mod.rs` | 注册 auth command 模块 |
| `src-tauri/src/lib.rs` | 注册账号相关 Tauri command |
| `src/services/desktopApi/index.ts` | 新增 `registerAccount` 前端适配函数 |
| `src/services/desktopApi/types.ts` | 新增 `AccountSession` 类型 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 合法用户名 / 密码可注册 | 通过，Rust 单元测试覆盖 |
| 密码不明文保存 | 通过，测试确认数据库保存值不是原密码，并以 Argon2 格式保存 |
| 非法输入被拒绝 | 通过，测试覆盖用户名长度、非法字符、密码长度和密码非法字符 |

### 任务 1.3 登录和当前账号上下文

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：实现账号登录校验，并在前端提供最小当前账号上下文，为后续登录页和路由保护提供基础。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/auth/mod.rs` | 新增 `login_account`，使用 Argon2 校验密码 |
| `src-tauri/src/commands/auth.rs` | 新增 `login_account` Tauri command |
| `src/services/desktopApi/index.ts` | 新增 `loginAccount` 前端适配函数 |
| `src/services/auth/currentAccount.ts` | 新增内存态当前账号上下文 |
| `src/services/auth/currentAccount.test.ts` | 新增当前账号上下文单元测试 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 登录后能获得当前账号 id | 通过，`AccountSession` 返回 `accountId` 和 `username` |
| 密码错误不能登录 | 通过，Rust 单元测试覆盖 |
| 未登录不能读取当前账号上下文 | 通过，Vitest 覆盖 `requireCurrentAccount` 抛错行为 |

### 任务 1.4 账号数据隔离基础查询

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立按 `account_id` 查询账号数据的最小后端基础，避免后续资料库只在 UI 层过滤。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/auth/mod.rs` | 新增 `account_data_summary`，按 `account_id` 统计影片和女优记录 |
| `src-tauri/src/commands/auth.rs` | 新增 `account_data_summary` Tauri command |
| `src/services/desktopApi/index.ts` | 新增 `getAccountDataSummary` 前端适配函数 |
| `src/services/desktopApi/types.ts` | 新增 `AccountDataSummary` 类型 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 账号 A 的数据不会被账号 B 的基础查询读到 | 通过，Rust 单元测试使用两个账号分别插入数据并验证统计隔离 |
| 隔离约束落在后端查询层 | 通过，查询使用 `WHERE account_id = ?1` |

### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过，2 个测试文件，4 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过，授权模式下完成新增 Rust 依赖下载 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，5 个 Rust 测试用例 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 未执行成功，当前 toolchain 未安装 `rustfmt` 组件 |

### 当前阻塞

无代码实现阻塞。仅有一个环境问题：当前 Rust toolchain 未安装 `rustfmt`，因此本轮不能执行 `cargo fmt`。

### 下一步建议

1. 进入任务 1.5：初始化默认账号设置和默认提示词。
2. 在后续实现资料 CRUD 时，所有 repository 必须继续把 `account_id` 作为后端查询条件。

## 7. 2026-06-22 第二批补充进度记录

### 任务 1.5 初始化默认账号设置和默认提示词

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：新账号创建后自动拥有默认推荐参考总数 30、默认首页推荐提示词和默认翻译提示词。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/defaults/mod.rs` | 新增账号默认设置初始化、默认提示词常量、读取默认初始化结果和单元测试 |
| `src-tauri/src/auth/mod.rs` | 注册账号成功后调用默认初始化 |
| `src-tauri/src/commands/defaults.rs` | 新增 `initialize_account_defaults` Tauri command |
| `src-tauri/src/commands/mod.rs` | 注册 defaults command 模块 |
| `src-tauri/src/lib.rs` | 注册默认初始化 command |
| `src/services/desktopApi/index.ts` | 新增 `initializeAccountDefaults` 前端适配函数 |
| `src/services/desktopApi/types.ts` | 新增 `AccountDefaults` 类型 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 新账号拥有默认推荐参考总数 30 | 通过，Rust 单元测试覆盖 |
| 新账号拥有默认首页推荐提示词 | 通过，Rust 单元测试覆盖 |
| 新账号拥有默认翻译提示词 | 通过，Rust 单元测试覆盖 |
| 重复初始化不破坏已有默认设置 | 通过，使用 `INSERT OR IGNORE` 保持幂等 |

### 任务 1.6 初始化预置标签库

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：新账号创建后拥有文档确认的影片预置标签和女优预置标签，并且重复初始化不产生重复项。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/defaults/mod.rs` | 新增预置影片标签、预置女优标签和幂等写入逻辑 |
| `src-tauri/src/auth/mod.rs` | 注册账号成功后自动初始化预置标签 |

#### 当前预置标签

| 标签库 | 预置标签 |
|---|---|
| 影片标签库 | `#按摩`、`#NTR`、`#巨乳`、`#贫乳`、`#精油`、`#人妻`、`#乱伦`、`#熟女` |
| 女优标签库 | `#苗条`、`#强气` |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 新账号有影片预置标签 | 通过，Rust 单元测试覆盖 |
| 新账号有女优预置标签 | 通过，Rust 单元测试覆盖 |
| 重复初始化不产生重复标签 | 通过，测试确认 `#按摩` 只保留一份 |

### 任务 2.1 登录 / 注册页

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：提供最小登录 / 注册页面，用户可注册和登录本地账号；错误提示清楚，不暴露底层错误。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/pages/AuthPage.tsx` | 新增登录 / 注册页面，接入 `registerAccount` 和 `loginAccount` |
| `src/app/App.tsx` | 未登录时显示登录 / 注册页，登录后进入现有空壳预览 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 用户能注册本地账号 | 通过，页面调用 `registerAccount` |
| 用户能登录本地账号 | 通过，页面调用 `loginAccount` |
| 错误提示清楚 | 通过，页面把底层错误映射为用户可读文案 |
| 不暴露底层错误 | 通过，未直接渲染原始异常字符串 |

### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过，2 个测试文件，4 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，7 个 Rust 测试用例 |

### 当前阻塞

无代码实现阻塞。2026-06-22 后续验证中已确认 `rustfmt` 可用，并已成功执行 `cargo fmt` 和 `cargo fmt --check`。

### 下一步建议

1. 进入任务 2.2：登录态路由保护。
2. 任务 2.2 应把当前 `App.tsx` 的内存态页面切换收敛为明确的路由 / 会话保护机制。

## 8. 2026-06-22 第三批补充进度记录

### 任务 2.2 登录态路由保护

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立最小登录态路由保护。未登录时只能进入登录 / 注册页，登录成功后进入首页占位壳。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/app/router.ts` | 新增最小路由保护规则，当前支持 `auth` 和 `home` |
| `src/app/router.test.ts` | 新增路由保护单元测试 |
| `src/app/App.tsx` | 接入路由保护，登录成功后跳转到 `home` |
| `src/services/auth/sessionStore.ts` | 新增最小会话 store |
| `src/services/auth/sessionStore.test.ts` | 新增会话 store 单元测试 |
| `src/services/auth/currentAccount.ts` | 改为兼容包装，复用 `sessionStore` |
| `src-tauri/**/*.rs` | 执行 `cargo fmt` 后的 Rust 格式化调整 |

#### 验收结果

| 验收标准 | 结果 |
|---|---|
| 未登录只能看到登录页 | 通过，`resolveRoute("home", null)` 会返回 `auth` |
| 登录后进入首页 | 通过，登录成功后 App 设置 session 并请求 `home` |
| 受保护路由依赖会话状态 | 通过，`sessionStore.requireSession` 无会话时抛错 |

### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `corepack pnpm test` | 通过，4 个测试文件，9 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，7 个 Rust 测试用例 |

### 当前阻塞

无。

### 下一步建议

1. 进入任务 2.3：全局左侧折叠状态栏。
2. 任务 2.4 基础页面路由应在 2.3 之后接入左侧导航。

## 9. 2026-06-22 第四批补充进度记录

### 任务 2.3 全局左侧折叠状态栏

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：为登录后的页面建立全局左侧状态栏。默认折叠只显示图标，hover 有 tooltip，点击 logo 可展开，展开后显示中文按钮名和账号名。本阶段只接入首页占位壳，不实现完整页面路由切换（路由是后续 2.4）。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/components/layout/icons.tsx` | 新增侧栏内联 SVG 图标集（首页星火、女优人像、影片影像、设置齿轮、账号头像、收起箭头） |
| `src/components/layout/Sidebar.tsx` | 新增全局左侧折叠状态栏组件，含折叠 / 展开切换、图标 tooltip、选中态和账号名展示 |
| `src/components/layout/AppShell.tsx` | 新增登录后布局壳，组合 Sidebar 和页面内容 |
| `src/app/App.tsx` | 登录后用 `AppShell` 包裹首页占位壳 |

#### 设计说明

| 规则 | 实现方式 |
|---|---|
| 默认折叠只显示图标 | `Sidebar` 内部 `expanded` 状态默认为 `false`，折叠态宽度 `w-16` |
| hover 有 tooltip | 折叠态下每个图标按钮和账号头像用既有 `Tooltip` 包裹；展开态不重复显示 |
| 点击 logo 可展开 | logo 按钮点击切换 `expanded`；hover logo 时 tooltip 提示"展开 / 收起状态栏" |
| 展开后显示中文按钮名 | 展开态宽度 `w-56`，按钮显示中文标签 |
| 展开后显示账号名 | 展开态底部账号区显示用户名和首字母头像 |
| 当前选中项有选中态 | 选中项使用淡紫半透明底色 `rgba(159,136,219,0.18)` |
| 设置和账号固定底部 | 通过 `NAV_ITEMS`（首页 / 女优库 / 影片库）和 `FOOTER_ITEMS`（设置 / 账号）分区 |
| 展开 / 收起平滑动效 | 使用 `transition-[width] duration-200` |
| 不引入新图标库 | 全部图标为内联 SVG，符合"现有样式先完成最小状态栏" |
| 暂不实现完整路由切换 | `AppShell` 内部维护 `activeId` 仅供选中态切换；`onSelect` 不切换页面，页面路由留给 2.4 |

#### 暂未实现（明确属于后续任务）

| 内容 | 归属任务 |
|---|---|
| 首页 / 女优库 / 影片库 / 设置页真实页面路由切换 | 2.4 基础页面路由 |
| 全局错误提示与确认弹窗 | 2.5 全局错误提示与确认弹窗基础 |
| 首页轮播、AI 对话框 | 阶段 9 视觉完成 |

#### 验收结果

| 验收标准（来自 `docs/09_mvp_development_plan.md` 2.3） | 结果 |
|---|---|
| 默认折叠只显示图标 | 通过，折叠态 `w-16` 且按钮只渲染图标 |
| hover 有 tooltip | 通过，折叠态下每个图标按钮和账号头像都接入 `Tooltip` |
| 点击 logo 可展开 | 通过，logo 按钮 `onClick` 切换 `expanded` |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `corepack pnpm test` | 通过，4 个测试文件，9 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过，无警告 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，7 个 Rust 测试用例 |

#### 当前阻塞

无。本轮未启动 `tauri:dev` 窗口验证，按任务约束避免遗留长期运行进程；视觉验收留给后续阶段 9 或用户手动确认。

#### 下一步建议

1. 进入任务 2.4：基础页面路由。
2. 2.4 应把 `AppShell` 内部的 `activeId` 替换为真实路由 store，并接入 `HomePage`、`VideosPage`、`ActressesPage`、`SettingsPage`。
3. 后续真实页面接入后，再决定是否需要为 `Sidebar` 补充 Vitest 单元测试（如选中态、展开 / 折叠切换的纯逻辑提取）。

## 10. 2026-06-22 第五批补充进度记录

### 任务 2.4 基础页面路由

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：在任务 2.3 的全局左侧状态栏基础上，接入首页、女优库、影片库、设置页四个登录后基础页面，并让左侧状态栏点击后切换真实页面内容。本任务只建立页面路由和页面占位，不实现列表、详情、设置表单或首页推荐业务。

#### 2.3 验收复核结论

| 检查项 | 结论 |
|---|---|
| 默认折叠只显示图标 | 通过，`Sidebar` 默认 `expanded=false`，折叠宽度为 `w-16` |
| hover 有 tooltip | 通过，折叠态按钮和账号头像使用现有 `Tooltip` |
| 点击 logo 可展开 | 通过，logo 按钮切换 `expanded` 并设置 `aria-expanded` |
| 展开后显示中文按钮名和账号名 | 通过，展开态显示首页、女优库、影片库、设置和用户名 |
| 工程校验 | 通过，`pnpm test`、`pnpm build`、`cargo fmt --check`、`cargo check`、`cargo test` 均通过 |
| 质量评价 | 完成质量合格，范围控制符合 2.3；未提前实现完整页面路由。2.4 中已把账号从可点击路由按钮收敛为账号展示，避免出现尚未定义的账号页面入口 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/app/router.ts` | 扩展登录后受保护路由为 `home`、`actresses`、`videos`、`settings`，新增默认登录后路由常量 |
| `src/app/router.test.ts` | 补充四个受保护路由的登录 / 未登录解析测试 |
| `src/app/App.tsx` | 根据当前受保护路由渲染对应页面，并把侧栏点击接入路由状态 |
| `src/components/layout/AppShell.tsx` | 改为接收 `activeRoute` 和 `onRouteChange`，不再维护临时 activeId |
| `src/components/layout/Sidebar.tsx` | 改为使用真实受保护路由驱动选中态和点击切换；保留折叠 / 展开、tooltip 和账号展示 |
| `src/pages/PagePlaceholder.tsx` | 新增基础页面占位组件 |
| `src/pages/HomePage.tsx` | 新增首页基础页面 |
| `src/pages/VideosPage.tsx` | 新增影片库基础页面 |
| `src/pages/ActressesPage.tsx` | 新增女优库基础页面 |
| `src/pages/SettingsPage.tsx` | 新增设置基础页面 |
| `docs/05_feature_backlog.md` | 更新 F011 进度说明 |
| `docs/10_development_progress.md` | 记录 2.3 复核和 2.4 完成情况 |

#### 验收结果

| 验收标准（来自 `docs/09_mvp_development_plan.md` 2.4） | 结果 |
|---|---|
| 首页可切换 | 通过，侧栏首页按钮切换到 `HomePage` |
| 影片库可切换 | 通过，侧栏影片库按钮切换到 `VideosPage` |
| 女优库可切换 | 通过，侧栏女优库按钮切换到 `ActressesPage` |
| 设置页可切换 | 通过，侧栏设置按钮切换到 `SettingsPage` |
| 不扩大 MVP 范围 | 通过，页面仅为占位，不实现库列表、详情、设置表单、首页推荐或轮播 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `corepack pnpm test` | 通过，4 个测试文件，9 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，7 个 Rust 测试用例 |

#### 当前阻塞

无。

#### 下一步建议

1. 进入任务 2.5：全局错误提示与确认弹窗基础。
2. 2.5 应继续复用现有 Radix primitives，不引入新的通知库。

## 11. 2026-06-22 第六批补充进度记录

### 任务 2.5 全局错误提示与确认弹窗基础

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立可复用的全局错误 / 状态提示基础和可复用二次确认弹窗基础。本任务只提供 UI 基础设施，不提前接入删除、导入导出、元数据匹配或大模型调用等业务流程。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/components/ui/toastState.ts` | 新增 toast 纯状态逻辑，包括创建、追加、限制可见数量和关闭消息 |
| `src/components/ui/toastState.test.ts` | 新增 toast 纯逻辑单元测试 |
| `src/components/ui/Toast.tsx` | 新增 `ToastProvider`、`useToast` 和全局提示展示队列 |
| `src/components/ui/ConfirmDialog.tsx` | 新增基于 Radix Dialog 的受控确认弹窗组件 |
| `src/main.tsx` | 在应用根节点挂载 `ToastProvider` |
| `docs/10_development_progress.md` | 记录 2.5 完成情况 |

#### 验收结果

| 验收标准（来自 `docs/09_mvp_development_plan.md` 2.5） | 结果 |
|---|---|
| 错误提示可复用 | 通过，`useToast().notify({ variant: "error", ... })` 可在后续页面 / 业务组件中复用 |
| 普通状态提示可复用 | 通过，toast 支持 `info`、`success`、`error` 三种基础类型 |
| 提示消息可关闭 | 通过，每条 toast 有关闭按钮，并支持自动关闭；传入 `durationMs <= 0` 时可保持显示 |
| 二次确认可复用 | 通过，`ConfirmDialog` 支持受控 open、确认 / 取消文案、确认回调和 danger 样式 |
| 不引入未经确认的新依赖 | 通过，继续复用 React、Radix Dialog、Tailwind 和 CSS 变量 |
| 不扩大 MVP 范围 | 通过，未提前实现删除、导入导出、元数据匹配或大模型调用业务 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过，5 个测试文件，12 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe check --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，7 个 Rust 测试用例 |

#### 当前阻塞

无。

#### 下一步建议

1. 进入阶段 3：影片资料最小 CRUD。
2. 下一步最小任务应先从任务 3.1 影片领域类型与表单草稿模型开始，避免直接做完整资料库页面。

## 12. 2026-06-22 第七批补充进度记录

### 任务 3.1-3.4 影片 / 女优基础数据服务与最小列表页面

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：建立影片和女优的最小本地数据链路：后端按账号创建、读取、更新基础字段；前端通过 `desktopApi` 调用；影片库和女优库页面能展示当前账号记录、添加最小记录，并点击列表项进入详情占位区域。本轮不实现完整详情编辑、删除、标签、搜索筛选、影片女优关联或元数据匹配。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/library/mod.rs` | 新增影片 / 女优基础 CRUD 服务、输入校验、账号隔离测试 |
| `src-tauri/src/commands/library.rs` | 新增影片 / 女优 Tauri command 适配层 |
| `src-tauri/src/commands/mod.rs` | 注册 library command 模块 |
| `src-tauri/src/lib.rs` | 将影片 / 女优 commands 注册到 Tauri invoke handler |
| `src/services/desktopApi/types.ts` | 新增 `VideoRecord`、`VideoInput`、`ActressRecord`、`ActressInput` 和 `WorkType` |
| `src/services/desktopApi/index.ts` | 新增影片 / 女优 list、create、update 的前端调用封装和字段映射 |
| `src/pages/VideosPage.tsx` | 替换占位页为影片库最小列表、新增表单、选中详情占位 |
| `src/pages/ActressesPage.tsx` | 替换占位页为女优库最小列表、新增表单、选中详情占位 |
| `docs/05_feature_backlog.md` | 更新 F004 / F007 为已部分实现 |
| `docs/10_development_progress.md` | 记录 3.1-3.4 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 3.1 影片数据 repository 和 service | 可创建、读取、更新影片基础字段 | 通过，Rust `library` 服务和 Tauri command 已支持 `code`、`title`、`cover_path`、`work_type` |
| 3.2 女优数据 repository 和 service | 可创建、读取、更新女优基础字段 | 通过，Rust `library` 服务和 Tauri command 已支持 `name`、`avatar_path` |
| 3.3 影片列表最小页面 | 能展示当前账号影片；点击进入详情 | 通过，`VideosPage` 加载当前账号影片，点击卡片后显示详情占位区域 |
| 3.4 女优列表最小页面 | 能展示当前账号女优；点击进入详情 | 通过，`ActressesPage` 加载当前账号女优，点击卡片后显示详情占位区域 |
| 账号数据隔离 | 不显示其他账号资料 | 通过，Rust 测试覆盖影片 / 女优跨账号隔离 |
| 不扩大 MVP 范围 | 不提前实现后续任务 | 通过，详情编辑、删除、标签、关联、搜索和元数据匹配仍留给后续任务 |

#### 暂未实现（归属后续任务）

| 内容 | 归属 |
|---|---|
| 影片详情完整编辑，包括简介、发行日期、片长、来源链接、作品类型、影评 | 3.5 影片详情基础编辑 |
| 女优详情完整编辑，包括别名、多语种名称、三围、生日、备注 | 3.6+ 或后续女优详情任务 |
| 删除影片 / 女优与二次确认 | 4.8 删除二次确认 |
| 影片与女优关联 | 阶段 4 |
| 标签、搜索、排序、筛选 | 阶段 5 |
| 元数据匹配和候选确认 | 阶段 7 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，11 个 Rust 测试用例 |
| `corepack pnpm test` | 通过，5 个测试文件，12 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |

#### 当前阻塞

无。

#### 下一步建议

1. 下一步适合进入任务 3.5：影片详情基础编辑。
2. 建议单独完成 3.5，不和更多任务合并，因为详情编辑会涉及字段更多、保存策略和后续影评时间戳入口。

## 13. 2026-06-22 第八批补充进度记录

### 任务 3.5-3.6 影片 / 女优详情基础编辑

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：在 3.1-3.4 的最小列表和基础 CRUD 之上，接入影片详情与女优详情的基础字段编辑能力。继续保持 MVP 边界，不提前实现删除、标签、影片女优关联、元数据匹配、翻译按钮、时间戳插入或外部链接打开。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/library/mod.rs` | 扩展影片和女优 record / input / SQL 映射，支持详情字段保存；补充新增字段测试 |
| `src-tauri/src/commands/library.rs` | 扩展 Tauri command 输入字段 |
| `src/services/desktopApi/types.ts` | 扩展前端影片 / 女优 record 和 input 类型 |
| `src/services/desktopApi/index.ts` | 扩展前端字段映射和 command 输入转换 |
| `src/pages/VideosPage.tsx` | 将右侧影片详情占位升级为可编辑表单 |
| `src/pages/ActressesPage.tsx` | 将右侧女优详情占位升级为可编辑表单 |
| `docs/05_feature_backlog.md` | 更新 F004 / F005 / F007 的实现状态 |
| `docs/10_development_progress.md` | 记录 3.5-3.6 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 3.5 影片详情基础编辑 | 番号、标题、简介、发行日期、片长、来源链接、作品类型、影评可编辑 | 通过，`VideosPage` 右侧详情表单已支持这些字段保存 |
| 3.6 女优详情基础编辑 | 名字、多语种姓名、三围、生日、身高、出道日期、维基链接、评价可编辑 | 通过，`ActressesPage` 右侧详情表单已支持这些字段保存 |
| 后端持久化 | 详情字段写入 SQLite 并可重新读取 | 通过，Rust 测试覆盖影片 / 女优新增详情字段创建和更新 |
| 账号隔离 | 不跨账号读取或更新详情 | 通过，沿用 3.1-3.4 的账号过滤和测试 |
| 不扩大 MVP 范围 | 不提前实现后续任务 | 通过，删除、标签、关联、搜索、翻译、时间戳和维基打开仍留给后续任务 |

#### 暂未实现（归属后续任务）

| 内容 | 归属 |
|---|---|
| 影评时间戳插入 | 后续影评编辑任务 |
| 影片与女优关联 | 阶段 4 |
| 删除影片 / 女优与二次确认 | 4.8 删除二次确认 |
| 标签、搜索、排序、筛选 | 阶段 5 |
| 中文维基链接打开 | 3.8 维基百科链接打开 |
| 影片标题 / 简介自动翻译和重新翻译 / 取消翻译 | 阶段 8 |
| 元数据匹配和候选确认 | 阶段 7 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，11 个 Rust 测试用例 |
| `corepack pnpm test` | 通过，5 个测试文件，12 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |

#### 当前阻塞

无。

#### 下一步建议

1. 下一步适合进入任务 3.7：如果开发计划中定义了列表/详情间更明确的导航状态，应先补齐；否则进入 3.8 维基百科链接打开。
2. 如果继续合并任务，建议最多合并两个，因为后续会开始触及外部链接、删除确认、关联和搜索等更容易互相影响的交互。

## 14. 2026-06-22 第九批补充进度记录
### 任务 3.7-3.8 自动保存与维基百科链接打开

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成
- 任务目标：在影片详情和女优详情基础编辑能力之上接入本地延迟自动保存，并为女优中文维基百科链接提供系统浏览器打开入口。本轮不扩大到标签、关联、搜索、删除、元数据匹配或设置页自动保存。

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src/services/autosave/useAutosave.ts` | 新增通用延迟自动保存 hook，负责跳过初始加载、避免相同草稿重复保存，并暴露保存成功 / 失败回调 |
| `src/pages/VideosPage.tsx` | 影片详情表单接入自动保存状态；按影片 ID remount 表单，避免切换记录时串保存 |
| `src/pages/ActressesPage.tsx` | 女优详情表单接入自动保存状态；新增“打开维基”按钮；按女优 ID remount 表单，避免切换记录时串保存 |
| `src/services/desktopApi/index.ts` | 新增 `openExternalUrl`，复用现有 Tauri opener 插件打开外部链接 |
| `docs/05_feature_backlog.md` | 更新 F007 / F009 的实现状态 |
| `docs/10_development_progress.md` | 记录 3.7-3.8 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 3.7 自动保存机制 | 编辑后无需手动保存；重启后数据仍存在 | 通过：影片详情和女优详情基础字段在编辑后会延迟调用现有更新接口持久化；保存状态在详情区显示 |
| 3.7 记录切换边界 | 切换影片 / 女优时不应把上一条草稿保存到下一条记录 | 通过：详情表单使用记录 ID 作为 `key`，切换记录时自动保存内部状态会重新初始化 |
| 3.8 维基百科链接打开 | 女优详情中的中文维基百科链接可打开 | 通过：女优详情支持通过 `openExternalUrl` 调用 Tauri opener 插件打开链接 |
| 3.8 无链接状态 | 无链接时不报错 | 通过：链接为空时按钮禁用，处理函数也会直接返回 |
| 不扩大 MVP 范围 | 不提前实现后续任务 | 通过：本轮未实现标签、关联、搜索、删除、元数据匹配、设置页自动保存或外部链接校验增强 |

#### 暂未实现（归属后续任务）

| 内容 | 归属 |
|---|---|
| 标签、关联、设置页等其他编辑流的自动保存复用 | 后续对应功能任务 |
| 外部链接格式的更完整校验和链接预览 | 后续体验优化任务 |
| 影片与女优关联 | 阶段 4 |
| 删除影片 / 女优与二次确认 | 4.8 删除二次确认 |
| 搜索、排序和筛选 | 阶段 5 |
| 元数据匹配和候选确认 | 阶段 7 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过：5 个测试文件，12 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：11 个 Rust 测试用例 |

#### 当前阻塞

无。

#### 下一步建议

1. 下一步适合进入阶段 4：影片与女优关联。
2. 最小下一个任务建议是 4.1：建立影片-女优关联表和后端服务，不先做完整 UI，先把数据关系和账户隔离打稳。

## 15. 2026-06-22 第十批补充进度记录
### 任务 4.1-4.8 作品关联、规则、时间戳和删除

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力
- 任务目标：补齐阶段 4 的本地记录核心闭环，包括影片与女优关联、作品类型必填规则、素人手动关联、多人作品演员名单文本、已收藏女优自动关联、后续新增女优补关联确认、影评时间戳插入和删除二次确认。本轮不扩展到标签、搜索、元数据匹配、详情页视觉精修或跨页面详情跳转。

#### 任务关联程度判断

| 任务范围 | 关联程度 | 本轮处理 |
|---|---|---|
| 4.1-4.6 影片 / 女优关联与演员名单 | 高 | 共用 `video_actresses` 表、影片 `actor_names` 字段和关联服务，一次实现更稳 |
| 4.7 影评时间戳插入 | 中 | 依赖影片详情影评编辑，独立于关联后端，但与阶段 4 同页实现 |
| 4.8 删除二次确认 | 中 | 依赖现有 `ConfirmDialog` 和 repository 删除方法，和关联表的级联行为需要一起验证 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/library/mod.rs` | 扩展影片 `actor_names` 字段映射；新增关联、补关联建议、删除服务；补充后端单元测试 |
| `src-tauri/src/commands/library.rs` | 新增关联、补关联建议和删除相关 Tauri command |
| `src-tauri/src/lib.rs` | 注册阶段 4 新增 commands |
| `src/services/desktopApi/types.ts` | 新增 `actorNames` 和 `AssociationSuggestion` 前端类型 |
| `src/services/desktopApi/index.ts` | 新增关联、补关联建议和删除的前端 API 封装 |
| `src/pages/VideosPage.tsx` | 影片详情支持演员名单文本、女优关联勾选、单人作品关联校验提示、时间戳插入和删除确认 |
| `src/pages/ActressesPage.tsx` | 女优详情支持相关影片查看、删除确认；新增女优后支持多人作品补关联确认 |
| `docs/05_feature_backlog.md` | 更新 F004-F008、F023-F025 的实现状态 |
| `docs/10_development_progress.md` | 记录 4.1-4.8 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 4.1 影片与女优关联表和服务 | 影片可关联一个或多个已存在女优；女优详情可查看相关影片 | 通过：后端 `set_video_actresses` / `list_actress_videos` 已实现，前端影片详情和女优详情已接入 |
| 4.2 作品类型必填规则 | 番号和作品类型必填；单人作品必须关联女优；多人和素人不强制 | 通过：番号和作品类型沿用现有校验；单人作品清空关联会被后端拒绝；多人和素人允许无关联 |
| 4.3 素人手动关联 | 素人作品允许手动关联已收藏女优 | 通过：影片详情关联组件不限制素人作品手动勾选已收藏女优 |
| 4.4 多人作品演员姓名文本保存 | 多人作品可保存演员名单文本；未收藏女优不自动创建 | 通过：影片详情可编辑演员名单文本；后端只匹配现有女优，不创建新女优 |
| 4.5 多人作品已有女优自动关联 | 导入演员名单时只自动关联已收藏且匹配的女优 | 通过：保存多人作品演员名单时自动关联已收藏且姓名匹配的女优；测试覆盖未收藏女优不会被创建 |
| 4.6 后续新增女优补关联确认 | 新增女优匹配既有多人作品演员名单时弹出确认；取消则不关联 | 通过：新增女优后读取补关联建议并弹出确认；只有确认后才调用 `add_video_actress` |
| 4.7 影评时间戳插入 | 有效时分秒在当前光标插入 `[01:23:45]`；无效输入不污染影评 | 通过：影片详情时间戳控件按当前 textarea 光标插入；无效输入 toast 提示并返回 |
| 4.8 删除二次确认 | 删除影片 / 女优前必须确认；删除女优不删除影片 | 通过：影片和女优详情均使用 `ConfirmDialog`；后端测试覆盖删除女优后影片仍保留 |
| 账号隔离 | 关联和删除不得跨账号操作 | 通过：后端所有关联 / 删除服务均按 `account_id` 校验记录归属 |
| 不扩大 MVP 范围 | 不提前实现后续任务 | 通过：本轮未实现标签、搜索、排序、筛选、元数据匹配、图片缓存或视觉精修 |

#### 暂未实现（归属后续任务）

| 内容 | 归属 |
|---|---|
| 关联区域的更完整视觉布局、跨页面点击进入详情 | 阶段 9 页面视觉完成 |
| 影片 / 女优搜索、筛选、排序和标签关联 | 阶段 5 |
| 关联后的自动年龄标签和 Cup 标签 | 阶段 5 |
| 元数据匹配结果写入演员名单和候选确认 | 阶段 7 |
| 删除后的撤销 / 回收站 | 第一版不做 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：13 个 Rust 测试用例 |
| `corepack pnpm test` | 通过：5 个测试文件，12 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |

#### 当前阻塞

无。

#### 下一步建议

1. 下一步适合进入阶段 5：标签库、姓名归一化、搜索和自动标签。
2. 建议先做 5.1-5.2：标签库 CRUD 和标签校验，因为后续搜索、自动年龄标签、Cup 标签都依赖标签基础模型。

## 16. 2026-06-22 第十一批补充进度记录
### 任务 5.1-5.11 标签库、姓名归一化、搜索和自动标签

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力
- 任务目标：在阶段 4 的影片 / 女优关联闭环之上，让资料库具备基础可检索能力，包括标签库 CRUD、标签校验、同义词 / 关联词检索、女优姓名归一化、多语种搜索、显示名 fallback、影片组合搜索、年龄标签解析、自动年龄标签、自动 Cup 标签和女优罩杯多选筛选。本轮不做最终视觉精修、虚拟列表、复杂搜索语法扩展或元数据候选确认。

#### 任务关联程度判断

| 任务范围 | 关联程度 | 本轮处理 |
|---|---|---|
| 5.1-5.3 标签库、校验、同义词和关联词 | 高 | 共用 `tag_library`、`video_tags`、`actress_tags`，一次完成后端服务、设置页入口和记录标签关联 |
| 5.4-5.6 女优姓名归一化、搜索、显示名 fallback | 高 | 共用女优多语种字段和前端筛选规则，做成纯 TypeScript 模块并接入女优库 |
| 5.7-5.8 影片组合搜索和年龄区间解析 | 高 | 搜索解析依赖标签和演员关联数据，接入影片库基础筛选 |
| 5.9-5.10 自动年龄标签和 Cup 标签 | 高 | 依赖影片发行日期、单人作品关联女优、生日和罩杯，放入后端标签服务并测试 |
| 5.11 女优罩杯多选筛选 | 中 | 依赖女优字段和前端列表筛选，和 5.4-5.6 一起实现 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/tags/mod.rs` | 新增标签库 CRUD、标签校验、同义词 / 关联词匹配、记录标签关联、自动年龄 / Cup 标签服务和 Rust 测试 |
| `src-tauri/src/commands/tags.rs` | 新增标签相关 Tauri commands |
| `src-tauri/src/commands/mod.rs` | 注册 tags command 模块 |
| `src-tauri/src/lib.rs` | 注册标签、记录标签和自动标签 commands |
| `src/services/desktopApi/types.ts` | 新增 `TagRecord`、`TagInput`、`TagMatch` 和 `TagScope` |
| `src/services/desktopApi/index.ts` | 新增标签 CRUD、匹配、记录标签关联和自动标签的前端 API 封装 |
| `src/domain/actressName.ts` | 新增女优姓名归一化、多语种搜索和显示名 fallback 规则 |
| `src/domain/videoSearch.ts` | 新增影片组合搜索解析、年龄区间解析、标签 / 女优 / 作品类型筛选和排序规则 |
| `src/domain/actressFilter.ts` | 新增女优标签搜索、罩杯筛选和 Cup 合法值规则 |
| `src/domain/stage5Rules.test.ts` | 新增阶段 5 前端规则单元测试 |
| `src/pages/SettingsPage.tsx` | 设置页从占位升级为影片 / 女优标签库最小管理入口 |
| `src/pages/VideosPage.tsx` | 接入影片搜索、作品类型筛选、排序、影片标签勾选和自动标签按钮 |
| `src/pages/ActressesPage.tsx` | 接入女优搜索、显示名选择、Cup 多选筛选、女优标签勾选和标签搜索 |
| `docs/05_feature_backlog.md` | 更新 F026-F032 的实现状态 |
| `docs/10_development_progress.md` | 记录 5.1-5.11 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 5.1 标签库 CRUD | 影片标签库和女优标签库独立；可新增、修改、删除 | 通过：后端按 `scope` 区分；设置页提供两个标签分区 |
| 5.2 标签校验 | 标签原则上必须中文；`#NTR` 例外；未成年人相关标签被拒绝或提示 | 通过：Rust 测试覆盖 `#NTR` 例外、非中文拒绝和未成年人相关词拒绝 |
| 5.3 标签同义词和关联词 | 搜索同义词 / 繁体 / 关联词能命中规范标签记录 | 通过：标签服务和前端搜索均读取 aliases / relatedTags；Rust 测试覆盖 `#媚藥`、`#春药` 命中 |
| 5.4 女优姓名归一化规则 | 繁简、大小写、全角半角、空格和常见分隔符归一化 | 通过：`actressName.ts` 实现 NFKC、大小写、空格和分隔符处理；Vitest 覆盖 |
| 5.5 女优多语种搜索 | 简中、繁中、日文、罗马音、曾用名和别名均参与搜索 | 通过：女优库搜索使用多字段归一化匹配；Vitest 覆盖 |
| 5.6 女优显示名 fallback | 显示名类型和单个女优默认显示类型生效；缺失时按文档顺序回退 | 通过：列表支持全局显示名类型和 fallback；单个女优默认显示类型已接入后端字段、前端 API、女优详情编辑入口和显示规则 |
| 5.7 影片组合搜索解析 | 支持关键词、`#标签`、`+` 条件叠加、作品类型筛选、排序 | 通过：影片库支持关键词、标签、出演女优、演员名单、作品类型筛选、番号升序、发行日期倒序 |
| 5.8 年龄标签解析和检索 | 支持 `#[22-33]`、`#[22-]`、`#[25]`、`#[-25]` | 通过：`videoSearch.ts` 解析并按 `#xx岁` 标签筛选；Vitest 覆盖 |
| 5.9 自动年龄标签 | 单人作品且生日 / 发行日期存在时自动添加年龄标签；缺失任一日期不添加 | 通过：后端 `auto_tag_video` 仅在单人作品、关联女优生日和发行日期存在时生成；Rust 测试覆盖 |
| 5.10 自动 Cup 标签 | 单人作品且女优罩杯存在时自动添加 `#xxCup` | 通过：后端生成 `#FCup` 等标签，且 A-K 排除 I/J；Rust / Vitest 覆盖 |
| 5.11 女优罩杯多选筛选 | A-K 且排除 I/J；支持全选和多选 | 通过：女优库提供 A/B/C/D/E/F/G/H/K 多选；未选择等同全选 |
| 不扩大 MVP 范围 | 不提前实现后续任务 | 通过：本轮未做元数据匹配、导入导出、大模型推荐、首页轮播或最终视觉精修 |

#### 暂未实现（归属后续任务）

| 内容 | 归属 |
|---|---|
| 搜索结果高亮、复杂搜索语法提示、空状态视觉优化 | 阶段 9 页面视觉完成 |
| 标签批量编辑、标签排序和更完整的标签管理体验 | 阶段 9 设置页视觉 |
| 元数据匹配后的标签 / 演员候选确认 | 阶段 7 |
| 1999 条规模下搜索和列表性能验证 | 阶段 10 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：16 个 Rust 测试用例 |
| `corepack pnpm test` | 通过：6 个测试文件，17 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |

#### 当前阻塞

无。

#### 下一步建议

1. 下一步适合进入阶段 6：图片缓存和数据导入导出。
2. 建议先做 6.1-6.3：应用数据目录、图片缓存服务、本地图片选择 / 替换。这三项直接支撑后续元数据匹配、导入导出和首页 / 详情页视觉。

## 17. 2026-06-22 第十二批补充进度记录

### 阶段五验收修正与任务 6.1-6.3 图片缓存基础能力

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力
- 任务目标：先验收阶段五已完成内容，再在不引入未确认依赖的前提下完成阶段 6 中与图片缓存强相关的 6.1-6.3。导入导出相关 6.4-6.8 需要压缩、校验和、加密格式和依赖确认，本轮不实现。

#### 阶段五验收结论

| 项目 | 结论 |
|---|---|
| 5.1-5.5 | 通过：标签库、标签校验、同义词 / 关联词、女优姓名归一化和多语种搜索与前次记录一致 |
| 5.6 | 原记录为“部分通过”，本轮发现并修复缺口：单个女优默认显示名类型已接入数据库字段、后端输入输出、前端 API、女优详情编辑入口和列表显示规则 |
| 5.7-5.11 | 通过：影片组合搜索、年龄标签解析、自动年龄标签、自动 Cup 标签和女优罩杯多选筛选与前次记录一致 |

#### 任务关联程度判断

| 任务范围 | 关联程度 | 本轮处理 |
|---|---|---|
| 6.1 应用数据目录适配 | 高 | 与图片缓存目录直接相关，已完成 |
| 6.2 图片缓存服务 | 高 | 依赖 6.1 的目录结构，已完成 |
| 6.3 本地图片选择 / 替换 | 高 | 依赖 6.2 的缓存服务，已完成最小手动路径版本 |
| 6.4-6.8 普通 / 加密导出与导入恢复 | 高，但属于独立导入导出链路 | 暂缓：需要确认 zip、checksum、加密实现依赖和导入事务策略，避免擅自引入新依赖或确定文件格式细节 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/assets/mod.rs` | 新增应用数据路径、图片缓存复制、缓存相对路径解析和 Rust 单元测试 |
| `src-tauri/src/commands/assets.rs` | 新增 `app_data_paths`、`cache_local_image`、`resolve_cached_asset` Tauri commands |
| `src-tauri/src/commands/mod.rs` | 注册 assets command 模块 |
| `src-tauri/src/lib.rs` | 注册图片缓存相关 commands |
| `src/services/desktopApi/types.ts` | 新增 `AppDataPaths`、`ImageCacheKind`、`CachedImage` 类型 |
| `src/services/desktopApi/index.ts` | 新增应用数据路径、缓存本地图片、解析缓存资源和转换前端 asset URL 的 API 封装 |
| `src/pages/VideosPage.tsx` | 影片详情支持输入本地封面路径、复制到缓存、保存相对路径并预览 |
| `src/pages/ActressesPage.tsx` | 女优详情支持输入本地头像路径、复制到缓存、保存相对路径并预览；补齐默认显示名类型保存链路 |
| `src/pages/SettingsPage.tsx` | 设置页展示应用数据目录、数据库路径、图片缓存目录、封面目录和头像目录 |
| `src-tauri/src/library/mod.rs` | 补齐 `default_display_name_type` 的后端读写、校验和测试 |
| `src-tauri/src/commands/library.rs` | 补齐女优默认显示名类型 command 输入映射 |
| `src-tauri/src/tags/mod.rs` | 同步测试 helper 的女优输入字段 |
| `src/domain/actressName.ts` | 显示名 fallback 默认读取单个女优的默认显示名类型 |
| `docs/05_feature_backlog.md` | 更新 F021 图片本地缓存状态 |
| `docs/10_development_progress.md` | 记录阶段五验收修正和 6.1-6.3 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 6.1 应用数据目录适配 | 使用应用专属数据目录；数据库、缓存、导出路径清晰 | 通过：后端返回 app data、database、cache、covers、actresses 路径；设置页可查看 |
| 6.2 图片缓存服务 | 封面和头像以内部 id 命名；数据库保存相对路径 | 通过：封面缓存到 `cache/covers/{id}.{ext}`，头像缓存到 `cache/actresses/{id}.{ext}`；前端保存相对路径 |
| 6.3 本地图片选择 / 替换 | 用户可手动设置封面 / 头像；重启后仍显示 | 基础通过：用户可输入本地图片完整路径并缓存，详情页按相对路径解析为本地 asset URL 预览；原生文件选择器待确认插件后再做 |
| 非法路径和格式保护 | 不允许路径穿越；不支持的图片格式拒绝 | 通过：后端拒绝绝对缓存相对路径、`..` 路径和非 jpg/jpeg/png/webp/gif 扩展名 |
| 不扩大 MVP 范围 | 不提前实现导入导出和未确认依赖 | 通过：本轮未实现 `.nvyzip`、`.nvyenc`、checksums、导入恢复或加密导出 |

#### 暂未实现（需要后续确认或归属后续任务）

| 内容 | 归属 / 原因 |
|---|---|
| 原生文件选择器 | 需要确认是否引入 Tauri dialog 插件；当前先用手动路径满足最小替换能力 |
| 元数据匹配成功后的图片自动缓存 | 阶段 7：依赖元数据候选和图片来源适配 |
| 普通导出 `.nvyzip` | 阶段 6.4：需要确认 zip 与 checksum 依赖和 manifest 细节 |
| 加密导出 `.nvyenc` | 阶段 6.5：需要确认加密算法、KDF、依赖和文件头格式 |
| 导入校验与恢复 | 阶段 6.6-6.8：需要确认事务化恢复、冲突策略和损坏包处理 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：18 个 Rust 测试用例 |
| `corepack pnpm test` | 通过：6 个测试文件，17 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |

#### 当前阻塞

6.4-6.8 不适合继续直接实现，原因是导入导出链路会确定长期兼容的包格式，并且需要引入压缩、校验和、加密相关依赖。按照当前项目规则，未确认依赖和格式前不应擅自实现。

#### 下一步建议

1. 若继续阶段 6，先确认 `.nvyzip` / `.nvyenc` 的压缩、checksum、加密依赖和 manifest 字段，再做 6.4。
2. 若暂不确认导入导出格式，可以先进入阶段 7 的元数据匹配接口与候选确认；但阶段 7.6 会复用本轮完成的图片缓存服务。

## 18. 2026-06-22 第十三批补充进度记录

### 任务 6.4-6.8 普通 / 加密导出与导入恢复

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力
- 任务目标：完成当前账号资料库的普通导出、加密导出、导入校验、普通导入恢复和加密导入恢复。导出包必须包含 manifest、data、assets 和 checksums；导入失败不得破坏当前数据库。

#### 实现边界

| 范围 | 本轮处理 |
|---|---|
| `.nvyzip` | 使用 Windows PowerShell `Compress-Archive` 生成真实 zip 包，包含 `manifest.json`、`data.json`、`assets/`、`checksums.json` |
| `.nvyenc` | 对完整 zip 包体加密；密码经 Argon2 派生主密钥后生成加密流 |
| 导出数据 | 导出当前账号的影片、女优、影评、关联、标签、账号非敏感设置、元数据候选和图片缓存资源 |
| 敏感数据 | 不导出账号密码、密码哈希或明文 API Key |
| 导入目标 | 导入恢复到当前登录账号；如果导入包记录 id 与数据库其他账号冲突，会重映射导入目标记录 id 并修正关联 |
| 前端入口 | 设置页新增普通导出、加密导出、普通导入、加密导入；导入前二次确认 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/backup/mod.rs` | 新增导出数据收集、zip 包生成 / 读取、checksums 校验、加密 / 解密、导入事务恢复和 Rust 测试 |
| `src-tauri/src/commands/backup.rs` | 新增导入导出 Tauri commands |
| `src-tauri/src/commands/mod.rs` | 注册 backup command 模块 |
| `src-tauri/src/lib.rs` | 注册导入导出 commands |
| `src/services/desktopApi/types.ts` | 新增 `BackupResult` 和 `ImportResult` 类型 |
| `src/services/desktopApi/index.ts` | 新增普通 / 加密导出和普通 / 加密导入 API 封装 |
| `src/pages/SettingsPage.tsx` | 新增数据导入导出设置分区 |
| `docs/05_feature_backlog.md` | 更新 F010、F020、F022、F041 状态 |
| `docs/07_decision_log.md` | 记录第一版导入导出基础实现方式 |
| `docs/10_development_progress.md` | 记录 6.4-6.8 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 6.4 普通导出 `.nvyzip` | 导出包含 manifest、data、assets、checksums；不含明文 API Key | 通过：真实 zip 包包含四类内容；当前导出数据不包含 API Key 字段 |
| 6.5 加密导出 `.nvyenc` | 加密导出需要密码；导出内容不可直接明文读取 | 通过：导出前要求至少 6 位密码；`.nvyenc` 存储加密后的 zip 包体 |
| 6.6 导入校验 | manifest 或 checksums 缺失 / 损坏时拒绝导入，不破坏现有数据 | 通过：导入前解析 manifest、data、checksums 并逐项校验；损坏包测试覆盖目标账号数据不被写入 |
| 6.7 普通导入恢复 | 导入后账号、影片、女优、影评、关联、标签和图片引用保持完整 | 通过：普通导入恢复当前账号影片、女优、关联、标签和缓存资源；同库 id 冲突时自动重映射 |
| 6.8 加密导入恢复 | 密码正确可导入，密码错误拒绝且不破坏现有数据 | 通过：错误密码测试覆盖目标账号原数据保留；正确密码可恢复 |
| 设置页入口 | 用户可在设置页执行普通 / 加密导入导出 | 通过：设置页新增四个操作入口；导入前使用 `ConfirmDialog` 二次确认 |

#### 暂未实现或后续风险

| 内容 | 归属 / 原因 |
|---|---|
| 原生文件选择器 | 仍需确认是否引入 Tauri dialog 插件；当前使用手动路径 |
| 跨平台 zip 实现 | 第一版 Windows 目标下使用 PowerShell；未来跨平台需要替换为 Rust zip 依赖 |
| 更强 checksum / 加密 | 当前 checksum 为 `fnv1a64`，加密为 Argon2 派生密钥流；后续如提高安全等级，应确认 SHA-256 / HMAC / AEAD 依赖 |
| 1999 条规模性能 | 阶段 10 统一验证导入导出耗时、包大小和 UI 可用性 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：21 个 Rust 测试用例 |
| `corepack pnpm test` | 通过：6 个测试文件，17 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |

#### 当前阻塞

无。

#### 下一步建议

1. 阶段 6 的 P0 基础能力已完成；下一步适合进入阶段 7：元数据匹配和候选确认。
2. 阶段 7.6 的图片缓存写入可以复用 6.1-6.3 的缓存服务，导出恢复会覆盖这些缓存资源。

## 19. 2026-06-22 第十四批补充进度记录

### 任务 7.1-7.7 元数据匹配候选闭环

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力
- 任务目标：在不直接写死第三方站点抓取逻辑的前提下，先完成元数据匹配的统一候选结构、候选缓存、详情页匹配入口、候选确认、确认写入和失败兜底。真实 gfriends、DMM/FANZA、JavSP 联网适配仍需后续验证接口和许可边界。

#### 任务关联程度判断

| 任务范围 | 关联程度 | 本轮处理 |
|---|---|---|
| 7.1 元数据匹配统一接口 | 高 | 已完成：后端统一 `MetadataCandidate`，前端统一 `MetadataCandidate` 类型和 API 封装 |
| 7.2 女优头像来源适配 | 高 | 完成基础接口：候选 payload 支持 `avatar_source_path` 并可写入缓存；真实 gfriends 联网适配待验证 |
| 7.3 女优资料来源适配 | 高 | 完成基础接口：候选 payload 支持姓名、三围、生日、身高、出道日期等字段；真实 DMM/FANZA 适配待验证 |
| 7.4 影片资料来源适配 | 高 | 完成基础接口：候选 payload 支持番号、标题、封面、简介、演员名单、发行日期、片长等字段；真实 JavSP / DMM/FANZA 适配待验证 |
| 7.5 候选确认弹窗 | 高 | 已完成基础能力：详情页展示候选，用户可使用候选或跳过 |
| 7.6 匹配结果写入和图片缓存 | 高 | 已完成基础能力：确认候选后写入当前账号记录；候选带本地图片源时复用图片缓存服务 |
| 7.7 匹配失败兜底 | 高 | 已完成基础能力：无结果、失败时提示并保留手动编辑 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/metadata/mod.rs` | 新增元数据候选生成、候选缓存、候选应用、图片缓存写入和 Rust 测试 |
| `src-tauri/src/commands/metadata.rs` | 新增元数据匹配和应用候选 Tauri commands |
| `src-tauri/src/commands/mod.rs` | 注册 metadata command 模块 |
| `src-tauri/src/lib.rs` | 注册元数据 commands |
| `src/services/desktopApi/types.ts` | 新增 `MetadataCandidate` 类型 |
| `src/services/desktopApi/index.ts` | 新增影片 / 女优匹配和应用候选 API 封装 |
| `src/pages/VideosPage.tsx` | 影片详情新增匹配资料、候选展示、使用候选、跳过和失败提示 |
| `src/pages/ActressesPage.tsx` | 女优详情新增匹配资料、候选展示、使用候选、跳过和失败提示 |
| `docs/05_feature_backlog.md` | 更新 F014、F015、F018 状态 |
| `docs/06_acceptance_tests.md` | 更新 A009、A010、A013、A015、E009 状态 |
| `docs/07_decision_log.md` | 记录阶段 7 先完成候选闭环的实现决策 |
| `docs/10_development_progress.md` | 记录 7.1-7.7 完成情况 |

#### 验收结果

| 任务 | 验收标准 | 结果 |
|---|---|---|
| 7.1 元数据匹配统一接口 | 影片和女优匹配都返回统一候选结构 | 通过：统一返回 `MetadataCandidate`，payload 用 JSON 承载不同类型字段 |
| 7.2 女优头像来源适配 | 输入女优名可尝试匹配头像候选；失败允许手动编辑 | 基础通过：接口支持头像候选和缓存写入；真实 gfriends 来源未接入 |
| 7.3 女优资料来源适配 | 输入女优名可尝试匹配三围、生日、头像等候选资料 | 基础通过：本地占位适配器可返回候选并写入姓名字段；完整字段等待真实来源 |
| 7.4 影片资料来源适配 | 输入番号可尝试匹配封面、标题、女优、发行日期等候选资料 | 基础通过：本地占位适配器可返回候选并写入番号 / 标题；完整字段等待真实来源 |
| 7.5 候选确认弹窗 | 多候选时用户可选择、跳过或手动编辑；不静默覆盖 | 通过：候选需要用户点击“使用候选”才写入，跳过后继续手动编辑 |
| 7.6 匹配结果写入和图片缓存 | 确认候选后资料写入当前账号，封面 / 头像缓存本地 | 基础通过：写入当前账号记录；payload 带本地图片路径时复用缓存服务 |
| 7.7 匹配失败兜底 | 网络失败、无结果、接口失败时提示并允许继续手动录入 | 通过：无结果和异常均 toast 提示，不阻塞手动编辑 |

#### 暂未实现或后续风险

| 内容 | 归属 / 原因 |
|---|---|
| gfriends 真实头像匹配 | 需要验证接口、许可证、访问限制和返回格式 |
| DMM/FANZA 女优资料适配 | 需要验证 API 获取方式、地区限制和字段映射 |
| JavSP / DMM/FANZA 影片资料适配 | 需要验证多站点逻辑、许可证边界和失败策略 |
| 远程图片下载 | 当前图片缓存服务支持本地路径；真实远程图片下载需要网络客户端或 Tauri 能力确认 |
| 多候选排序和置信度 | 当前结构可承载多个候选；真实来源接入后再实现排序规则 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过：24 个 Rust 测试用例 |
| `corepack pnpm test` | 通过：6 个测试文件，17 个测试用例 |
| `corepack pnpm build` | 通过：TypeScript 和 Vite 构建成功 |

#### 当前阻塞

真实外部元数据来源尚未接入。阻塞点不是本地候选流程，而是 gfriends、DMM/FANZA、JavSP 或替代来源的可用性、许可边界、访问限制、地区限制和返回格式需要联网验证。

#### 下一步建议

1. 若继续阶段 7，应优先验证一个真实来源的最小可用请求和许可证边界，再替换本地占位候选适配器。
2. 若暂不处理外部来源，可以进入阶段 8：大模型设置、Stronghold、推荐和翻译；但真实元数据质量会影响后续推荐候选的完整度。
## 20. 2026-06-22 第十五批补充进度记录

### 任务 8.1-8.12 大模型设置、推荐 payload 和翻译入口
- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成基础能力；Stronghold 和真实模型调用待接入
- 任务目标：在不明文保存 API Key、不擅自引入新依赖的前提下，尽可能完成阶段 8 的本地可验证部分，包括大模型非敏感设置、提示词、推荐参考总数校验、首页推荐候选构造、推荐发送边界、翻译入口和失败兜底。

#### 任务完成情况

| 任务 | 本轮状态 | 说明 |
|---|---|---|
| 8.1 大模型设置 UI | 已完成 | 设置页新增大模型设置区，可编辑接口类型、Base URL、模型、供应商备注、温度、最大输出长度、推荐参考总数上限 |
| 8.2 推荐参考总数校验 | 已完成 | 前后端均校验 0-999，默认 30；非数字、负数、大于 999 被拒绝 |
| 8.3 提示词管理 | 已完成 | 翻译提示词和首页推荐提示词可编辑并保存 |
| 8.4 API Key Stronghold 存储 | 待接入 | 当前命令明确拒绝保存，不把 API Key 写入 SQLite；仍需接入 Tauri Stronghold |
| 8.5 大模型客户端抽象 | 已预留边界 | 设置支持三种接口类型；真实 HTTP 客户端和接口分流待接入 |
| 8.6 首页推荐候选构造 | 已完成 | 后端从本地影片 / 女优构造候选，并按参考总数上限截取；0 表示不限 |
| 8.7 首页推荐调用 | 部分完成 | 首页可输入偏好并构造候选预览；真实模型调用待 API Key 和 HTTP 客户端接入 |
| 8.8 推荐发送边界保护 | 已完成基础能力 | payload 包含用户输入、候选元数据和用户文本字段，不包含密码、API Key、本地绝对路径 |
| 8.9 自动翻译触发 | 待接入 | 已保存启用配置，真实自动触发和模型调用仍待实现 |
| 8.10 重新翻译 | 部分完成 | 影片详情已有重新翻译入口和失败提示；真实调用待接入 |
| 8.11 取消翻译 | 已完成基础能力 | 取消翻译优先回退原始标题 / 简介，没有原始字段时保留当前字段 |
| 8.12 大模型失败处理 | 已完成基础提示 | API Key 缺失、客户端未接入和设置保存失败都有清晰提示；真实网络失败待接入后复验 |

#### 已创建或修改的文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/src/llm/mod.rs` | 新增大模型设置读写、推荐 payload 构造、推荐参考总数校验、取消翻译和 Rust 测试 |
| `src-tauri/src/commands/llm.rs` | 新增大模型设置、API Key 占位、推荐 payload、翻译入口 Tauri commands |
| `src-tauri/src/commands/mod.rs` | 注册 llm command 模块 |
| `src-tauri/src/lib.rs` | 注册大模型相关 commands |
| `src-tauri/src/metadata/mod.rs` | 元数据候选写入影片时同步保留 `original_title` 和 `original_summary`，供取消翻译回退 |
| `src/services/desktopApi/types.ts` | 新增大模型设置、推荐 payload、翻译状态等前端类型 |
| `src/services/desktopApi/index.ts` | 新增大模型相关 desktopApi 封装和字段映射 |
| `src/domain/settingsValidation.ts` | 新增推荐参考总数上限解析与默认值规则 |
| `src/domain/settingsValidation.test.ts` | 新增推荐参考总数上限单元测试 |
| `src/pages/SettingsPage.tsx` | 新增大模型设置、提示词、API Key 输入入口和保存提示 |
| `src/pages/HomePage.tsx` | 新增首页推荐输入、候选 payload 构造和候选预览 |
| `src/pages/VideosPage.tsx` | 新增重新翻译 / 取消翻译入口和失败兜底提示 |
| `docs/04_architecture_notes.md` | 更新大模型和元数据真实接入边界 |
| `docs/05_feature_backlog.md` | 更新阶段 7 待接入来源和阶段 8 功能状态 |
| `docs/06_acceptance_tests.md` | 更新大模型相关验收状态 |
| `docs/07_decision_log.md` | 新增 D025 阶段 8 实现边界决策 |
| `docs/10_development_progress.md` | 记录本轮阶段 8 完成情况 |

#### 本轮验证命令

| 命令 | 结果 |
|---|---|
| `C:\Users\Chanis\.cargo\bin\cargo.exe fmt --manifest-path src-tauri\Cargo.toml` | 通过 |
| `C:\Users\Chanis\.cargo\bin\cargo.exe test --manifest-path src-tauri\Cargo.toml` | 通过，26 个 Rust 测试用例 |
| `corepack pnpm test` | 通过，7 个测试文件，20 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |

#### 当前阻塞

| 阻塞项 | 原因 |
|---|---|
| Tauri Stronghold API Key 保存 | 需要接入 `tauri-plugin-stronghold`，并明确账号级 key、解锁方式、清除策略和迁移策略 |
| 真实大模型 HTTP 调用 | 当前未接入 Rust HTTP 客户端或前端安全调用方案；需要实现 Responses API、Chat Completions 和自定义接口分流 |
| 自动翻译缓存和重新翻译真实结果 | 依赖真实模型调用和 API Key；当前只有入口、取消和失败兜底 |
| 真实推荐结果展示 | 当前只构造候选 payload；待模型返回结构确定后再渲染推荐影片、女优和理由 |
| 外部元数据来源 | 阶段 7 真实 gfriends / DMM/FANZA / JavSP 仍未接入，会影响推荐候选的资料完整度 |

#### 下一步建议

1. 优先接入 8.4：Tauri Stronghold API Key 存储。没有它，真实推荐和翻译都不应继续发送请求。
2. Stronghold 完成后接入 8.5 的真实 HTTP 客户端，再补 8.7、8.9、8.10 的真实模型调用。
3. 接入真实模型后，重点复验 A012、A034、A035、A037、A039 和 E019-E023。

## 21. 2026-06-22 Stronghold 最小闭环与阶段 1-8 闭环复查

### 任务 8.4 API Key Stronghold 保存最小闭环

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已实现，待 Rust 新依赖下载后复验
- 任务目标：完成 API Key 本地加密保存的最小闭环，包括保存、查询、清除、设置页状态展示和不写入 SQLite / 导出数据。

#### 本轮实现内容

| 项目 | 状态 | 说明 |
|---|---|---|
| 引入 Stronghold 插件 | 已声明，待下载验证 | `src-tauri/Cargo.toml` 新增 `tauri-plugin-stronghold = "2"` |
| 注册 Tauri Stronghold 插件 | 已实现，待编译验证 | `src-tauri/src/lib.rs` 在 setup 中使用 `Builder::with_argon2` 注册插件，salt 位于应用本地数据目录 |
| Stronghold 权限 | 已实现 | `src-tauri/capabilities/default.json` 新增 `stronghold:default` 和 `stronghold:allow-remove-store-record` |
| API Key 保存 | 已实现前端封装 | `src/services/stronghold/llmSecrets.ts` 调用 Stronghold 插件保存 record |
| API Key 查询 | 已实现前端封装 | `getLlmSettings` 会合并 Stronghold `hasApiKey` 状态 |
| API Key 清除 | 已实现前端封装 | 设置页新增清除 API Key 按钮，调用 Stronghold 删除 record |
| SQLite 明文保护 | 保持通过 | 本轮未新增 SQLite API Key 字段，导入导出数据仍不包含 API Key |
| HTTP 客户端调用 | 未纳入本任务 | 真实推荐和翻译请求仍待后续任务接入 |

#### 已修改文件

| 文件 / 目录 | 作用 |
|---|---|
| `src-tauri/Cargo.toml` | 新增 Tauri Stronghold 插件依赖 |
| `src-tauri/src/lib.rs` | 注册 Stronghold 插件并配置本地 salt 路径 |
| `src-tauri/capabilities/default.json` | 开放 Stronghold 默认权限和删除 record 权限 |
| `src/services/stronghold/llmSecrets.ts` | 新增 API Key 保存、读取、查询状态和清除封装 |
| `src/services/desktopApi/index.ts` | 将 `saveLlmApiKey` / `clearLlmApiKey` 接到 Stronghold，并在读取设置时合并 API Key 状态 |
| `src/pages/SettingsPage.tsx` | 设置页显示 Stronghold 保存说明，新增清除 API Key 按钮 |
| `docs/04_architecture_notes.md` | 更新 Stronghold 最小实现、文件位置和安全取舍 |
| `docs/05_feature_backlog.md` | 更新 F034 / F038 状态 |
| `docs/06_acceptance_tests.md` | 更新 A032 / A035 / E019 状态 |
| `docs/07_decision_log.md` | 新增 D026 Stronghold 最小闭环决策 |
| `docs/10_development_progress.md` | 记录本轮进度和阶段闭环复查 |

#### 验证结果

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过，7 个测试文件，20 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `cargo fmt --manifest-path src-tauri\Cargo.toml --check` | 通过 |
| `cargo test --manifest-path src-tauri\Cargo.toml` | 未通过，原因是当前环境无法下载 crates.io index / 新 Rust 依赖 |
| 授权模式执行 `cargo test --manifest-path src-tauri\Cargo.toml` | 未执行，审批通道返回自动拒绝，无法下载 `tauri-plugin-stronghold` 依赖 |

#### 当前待复验

| 项目 | 原因 |
|---|---|
| Rust 桌面端编译 | 需要下载 `tauri-plugin-stronghold` 及其依赖 |
| 设置页真实保存 API Key | 需要 Tauri 桌面端编译并运行后手动验证 |
| 本地 Stronghold 文件不含明文 API Key | 需要真实保存一次临时 Key 后检查本地文件 |
| HTTP 客户端调用 | 本轮未实现，仍是阶段 8 后续任务 |

### 阶段 1-8 闭环复查

| 阶段 | 当前闭环判断 | 主要依据 | 未闭环 / 风险 |
|---|---|---|---|
| 阶段 1：数据基础与账号 | 基础闭环完成 | SQLite schema、迁移、本地账号、Argon2 密码哈希、登录、账号隔离、默认设置和预置标签均已实现并测试通过 | 仍需后续全量回归确认所有新增查询持续带 `account_id` |
| 阶段 2：应用外壳与导航 | 基础闭环完成 | 登录 / 注册页、登录态路由保护、全局左侧折叠状态栏和基础路由已实现 | 首页轮播和最终视觉精修仍属于后续 UI 阶段 |
| 阶段 3：影片 / 女优资料 CRUD | 基础闭环完成 | 影片、女优、影评、时间戳、删除确认、基础详情编辑、关联关系已实现 | 复杂编辑体验和视觉细节仍待后续阶段 |
| 阶段 4：自动保存与关联规则 | 基础闭环完成 | 基础字段延迟保存、单人 / 多人 / 素人关联规则、多人作品已收藏女优自动关联已实现 | 大量交互下的 autosave 冲突仍需阶段 10 回归 |
| 阶段 5：标签、搜索和姓名规则 | 基础闭环完成 | 标签 CRUD、校验、同义词 / 关联词、女优姓名归一化、多语种搜索、组合搜索、年龄 / Cup 标签均已实现并测试通过 | 1999 条规模下搜索性能待阶段 10 验证 |
| 阶段 6：图片缓存和导入导出 | 基础闭环完成 | 应用数据目录、图片缓存、普通 `.nvyzip`、加密 `.nvyenc`、导入校验和恢复已实现 | 加密实现仍是 MVP 级方案；SHA-256 / AEAD 等更强方案暂未引入 |
| 阶段 7：元数据匹配候选 | 本地候选闭环完成，真实来源未闭环 | 统一候选结构、候选确认、写入、图片缓存和失败兜底已实现 | gfriends、DMM/FANZA、JavSP 等真实来源、许可边界和远程图片下载仍待验证 |
| 阶段 8：大模型设置、推荐和翻译 | 设置 / payload / Stronghold 基础闭环部分完成；真实调用未闭环 | 非敏感设置、提示词、推荐候选 payload、发送边界、翻译入口、取消翻译和 Stronghold 最小保存闭环已实现 | Rust Stronghold 依赖下载待复验；HTTP 客户端、真实首页推荐、自动翻译和重新翻译真实结果仍未实现 |

#### 复查结论

阶段 1-6 的 MVP 基础闭环相对完整，可以继续作为后续功能的基础。阶段 7 已完成本地候选闭环，但真实外部元数据来源仍是明确风险。阶段 8 经过本轮后，API Key 保存从“待接入”推进到“已实现最小闭环，待依赖下载后复验”；真正阻塞大模型完整闭环的下一项变为 HTTP 客户端调用和模型返回结果解析。

## 22. 2026-06-22 第十六批补充进度记录

### 任务 9.1-9.10 首页、库页面、详情页和设置页视觉完成

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：已完成前端视觉与交互闭环；Rust Stronghold 构建问题按用户要求暂时忽略
- 任务目标：在不扩大 MVP 范围、不引入新依赖的前提下，完成阶段 9 的暗色主题精修、全局下拉、首页轮播、首页推荐框、女优库、影片库、女优详情、影片详情和设置页视觉整理。

#### 任务完成情况

| 任务 | 本轮状态 | 说明 |
|---|---|---|
| 9.1 全局主题精修 | 已完成 | 收紧全局 CSS 变量，背景改为暗灰层级，淡紫 / 淡粉仅作为强调色，统一 focus、滚动条、checkbox 和低饱和 danger 色 |
| 9.2 全局下拉栏 | 已完成 | `Dropdown` 支持选中态、描述文本、打开态视觉和回调，阶段 9 页面不再使用原生 `<select>` |
| 9.3 首页女优轮播 | 已完成 | 首页读取当前账号全部女优，按 1 秒节奏左移轮播，中心项放大，点击进入对应女优详情 |
| 9.4 首页影片轮播 | 已完成 | 首页读取当前账号全部影片，展示封面 / 番号 / 两行标题，按 1 秒节奏左移轮播，点击进入对应影片详情 |
| 9.5 首页 AI 对话框视觉 | 已完成 | 首页推荐框改为 AI Studio 倾向的暗色输入区域，并保留 API Key / HTTP 客户端未接入提示 |
| 9.6 女优库视觉 | 已完成 | 搜索区固定顶部，左侧列表改为 3 列头像网格，列表区域独立滚动并带顶部渐隐 |
| 9.7 影片库视觉 | 已完成 | 搜索区固定顶部，左侧列表改为 3 列封面网格，封面固定 3:4 比例，标题两行截断 |
| 9.8 女优详情视觉 | 已完成 | 详情区改为独立暗色编辑面板，标签 / 输入 / 头像 / 相关影片保持统一层级，相关影片改为 3 列卡片 |
| 9.9 影片详情视觉 | 已完成 | 详情区改为独立暗色编辑面板，封面使用稳定比例，作品类型改为项目下拉，简介 / 影评 / 时间戳布局保持完整 |
| 9.10 设置页视觉 | 已完成 | 设置页拆成 LLM 主配置、存储、导入导出和标签库独立区域，API 类型改为项目下拉 |

#### 已修改文件

| 文件 | 作用 |
|---|---|
| `src/styles.css` | 精修全局主题变量、背景、滚动条、选区和 checkbox 状态 |
| `src/components/ui/Dropdown.tsx` | 增强 Dropdown 的选中态、描述、打开态样式和选择回调 |
| `src/app/App.tsx` | 新增首页轮播点击后跳转到指定影片 / 女优详情的最小状态通道 |
| `src/pages/HomePage.tsx` | 重做首页为女优轮播、影片轮播、推荐输入和候选预览组合 |
| `src/pages/ActressesPage.tsx` | 接入指定女优聚焦，替换显示名下拉，调整库页网格和详情视觉 |
| `src/pages/VideosPage.tsx` | 接入指定影片聚焦，替换作品类型 / 筛选 / 排序下拉，调整库页网格和详情视觉 |
| `src/pages/SettingsPage.tsx` | 调整设置页区域层级，替换 API 类型下拉 |
| `docs/10_development_progress.md` | 记录阶段 9 完成情况和验证结果 |

#### 验证结果

| 命令 | 结果 |
|---|---|
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `corepack pnpm test` | 通过，7 个测试文件，20 个测试用例 |
| `rg -n "<select\|rounded-3xl\|filter\\(Boolean\\)" src\\pages src\\components\\ui src\\styles.css` | 阶段 9 页面未发现原生 `<select>`；剩余 `rounded-3xl` 位于 ConfirmDialog，剩余 `filter(Boolean)` 为推荐候选描述拼接 |

#### 当前未复验 / 风险

| 项目 | 原因 |
|---|---|
| Tauri 桌面端运行截图验收 | 用户要求暂时忽略 Stronghold / libsodium 构建问题，当前未运行 Tauri 桌面端 |
| 首页真实模型推荐结果 | 阶段 9 只做视觉和候选 payload 预览；真实 HTTP 客户端调用仍属阶段 8 后续未闭环项 |
| 1999 条记录下滚动性能 | 当前只完成布局和前端构建验证；大规模数据性能仍需阶段 10 回归 |
| 移动 / 窄窗口视觉细节 | 当前通过响应式类处理，但尚未做 Playwright 截图级验证 |

#### 结论

阶段 9 的前端页面视觉和基础交互已经完成，可以进入阶段 10 的回归、性能和闭环验证。由于 Stronghold 依赖的 Rust 构建问题本轮按用户要求跳过，桌面端完整运行验收需要稍后单独处理。

## 23. 2026-06-22 第十七批补充进度记录

### 任务 10.1-10.8 规模、性能、隐私与 MVP 回归验收

- 开始时间：2026-06-22
- 完成时间：2026-06-22
- 当前状态：自动化可验证部分已完成；桌面端、离线、导入导出大数据和 Windows 10/11 双环境验收待 Stronghold / libsodium 构建问题解决后复验
- 任务目标：围绕 1999 条个人规模资料库目标，建立前端纯逻辑规模基线，检查首页轮播、列表搜索、推荐 payload 隐私边界，并记录无法在当前环境完成的回归项。

#### 任务完成情况

| 任务 | 本轮状态 | 说明 |
|---|---|---|
| 10.1 构造 1999 条测试数据 | 已完成 | 新增内存规模数据生成器，构造 1400 条影片和 599 条女优，合计 1999 条；不写入真实数据库 |
| 10.2 列表和搜索性能验证 | 部分完成 | 影片组合搜索、女优多语种 / 标签 / 罩杯筛选在 1999 条内存数据下通过 Vitest 基线；真实 DOM 滚动和详情打开仍需桌面端复验 |
| 10.3 首页轮播性能验证 | 已完成前端纯逻辑基线 | 首页轮播旋转逻辑抽成纯函数并在 1999 条规模下测试通过；真实动画帧率仍待桌面端截图 / 交互验收 |
| 10.4 导入导出性能和完整性验证 | 未完全执行 | Rust / Tauri 端受 Stronghold libsodium 构建问题影响，本轮未运行大数据导入导出闭环；保留为后续风险 |
| 10.5 离线功能验证 | 未完全执行 | 本轮未切换系统离线环境；本地纯逻辑不依赖网络，真实 Tauri 离线使用仍待手动验收 |
| 10.6 隐私泄露检查 | 部分完成 | 自动化测试确认推荐 payload 不包含密码、密码哈希、API Key 或本地绝对路径；仓库扫描未发现用户临时 API Key；Stronghold 本地文件仍待桌面端真实保存后复验 |
| 10.7 Windows 10 / 11 验证 | 未完成 | 当前环境无法同时验证 Windows 10 和 Windows 11，且 Tauri 构建仍受 libsodium 问题影响 |
| 10.8 MVP 回归验收 | 部分完成 | `corepack pnpm test` 和 `corepack pnpm build` 通过；`docs/06_acceptance_tests.md` 已记录 A044 与性能验收的当前通过范围和剩余风险 |

#### 已创建或修改的文件

| 文件 | 作用 |
|---|---|
| `src/domain/carousel.ts` | 抽出首页轮播旋转纯函数，便于规模测试和后续复用 |
| `src/pages/HomePage.tsx` | 改为使用 `rotateItems` 纯函数，页面行为不改变 |
| `src/domain/stage10ScaleData.ts` | 新增 1999 条规模数据生成器和推荐 payload 样本生成器 |
| `src/domain/stage10Scale.test.ts` | 新增阶段 10 规模、搜索、轮播和隐私边界自动化测试 |
| `docs/02_product_spec.md` | 修正旧的 `api_key_encrypted` 数据字典表述，明确 API Key 使用 Stronghold 且不写入 SQLite / 导出 data.json |
| `docs/06_acceptance_tests.md` | 更新 A044、性能验收和阶段 10 验收记录 |
| `docs/10_development_progress.md` | 记录阶段 10 完成情况、阻塞项和验证命令 |

#### 验证结果

| 命令 | 结果 |
|---|---|
| `corepack pnpm test` | 通过，8 个测试文件，25 个测试用例 |
| `corepack pnpm build` | 通过，TypeScript 和 Vite 构建成功 |
| `rg -n <redacted-temporary-api-key> .` | 首次扫描未发现用户临时 API Key；进度文档中的命令记录已脱敏，复扫后仓库不再包含完整临时 Key |
| `rg -n "passwordHash\|apiKey\|api_key\|absolutePath" src src-tauri docs` | 仅发现类型、函数参数、Stronghold 封装、缓存路径类型和文档说明；推荐 payload 测试确认不包含这些敏感字段 |

#### 当前未闭环 / 风险

| 项目 | 原因 |
|---|---|
| Tauri / Rust 端完整测试 | 当前 Stronghold 依赖链中的 libsodium 构建问题按用户要求暂时忽略，本轮未执行 `cargo test` / `tauri dev` |
| 大量数据和图片导入导出 | 需要真实桌面端和文件系统闭环，当前只完成前端纯逻辑规模基线 |
| 离线验收 | 需要手动断网或受控网络环境验证 |
| Windows 10 / 11 双环境 | 当前工作环境不能同时代表两个系统版本 |
| 首页真实动画帧率和列表滚动 | 需要桌面端运行和可视化 / 手动验收，Vitest 只能覆盖纯逻辑性能 |

#### 结论

阶段 10 中可以在当前环境准确完成的 10.1、10.2 前端搜索基线、10.3、10.6 推荐 payload 隐私边界和 10.8 自动化回归已经完成。10.4、10.5、10.7 以及 A044 中涉及真实桌面端、文件导入导出、离线和 Windows 10/11 的部分保持为明确待复验项，不应在 Stronghold / libsodium 构建问题解决前标记为完全通过。

## 24. 2026-06-22 开发启动黑屏 / 首屏加载过慢修复记录

### 问题现象

- 使用 `corepack pnpm tauri:dev` 启动后，Tauri 窗口黑屏。
- 浏览器访问 `http://127.0.0.1:1420/` 长时间转圈，约 5 分钟后才显示。

### 根因

- Tailwind CSS v4 默认自动扫描源码。
- 仓库根目录下存在大量非源码目录：`Temp/` 约 107317 个文件，`src-tauri/target/` 约 16621 个文件，另有 `.research-*` 研究目录。
- `src/styles.css` 原先使用 `@import "tailwindcss";`，没有限制 Tailwind 扫描范围。
- Vite dev server 的文件监听也没有显式忽略这些大型目录。
- 首次请求 CSS / 前端模块时，开发服务器会被大量无关文件拖慢，表现为浏览器转圈和 Tauri 黑屏。

### 修复内容

| 文件 | 修改 |
|---|---|
| `src/styles.css` | 改为 `@import "tailwindcss" source(none);`，并用 `@source ".";` 只扫描 `src/` 目录 |
| `vite.config.ts` | `server.watch.ignored` 忽略 `src-tauri/target/`、`Temp/`、`.research-*`、`.pnpm-store/`、`dist/` |
| `.gitignore` | 忽略 `Temp/` 和 `.research-*/`，避免临时研究文件继续进入项目扫描范围 |

### 验证结果

| 验证项 | 结果 |
|---|---|
| `corepack pnpm build` | 通过，Vite build 约 2.15 秒 |
| `corepack pnpm test` | 通过，8 个测试文件，25 个测试用例 |
| `cargo test --manifest-path .\src-tauri\Cargo.toml` | 通过，33 个测试 |
| `corepack pnpm tauri:dev` | Vite ready 约 791ms，Rust dev 编译约 1.32s |
| `http://127.0.0.1:1420/` 首次请求 | 约 165ms |
| `http://127.0.0.1:1420/src/styles.css` 首次请求 | 约 854ms |
| `http://127.0.0.1:1420/src/main.tsx` 首次请求 | 约 676ms |

### 结论

首屏黑屏 / 浏览器长时间转圈的主要原因是开发服务器扫描了大量非源码文件。当前已通过限制 Tailwind 扫描范围、忽略 Vite watcher 大目录、忽略临时研究目录完成修复。
