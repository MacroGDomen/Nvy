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

## 4. 任务记录

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
