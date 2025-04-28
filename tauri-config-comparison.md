# Tauri v1 与 v2 配置对比

本文档对比 Tauri v1 和 v2 版本的配置结构和内容差异，帮助开发者了解版本变更并进行迁移。

## 配置结构变化

### v1 配置结构
```json
{
  "package": { ... },
  "tauri": { 
    "allowlist": { ... },
    "bundle": { ... },
    "security": { ... },
    "updater": { ... },
    "windows": [ ... ]
  },
  "build": { ... },
  "plugins": { ... }
}
```

### v2 配置结构
```json
{
  "identifier": "...",
  "productName": "...", 
  "version": "...",
  "app": {
    "security": { ... },
    "windows": [ ... ]
  },
  "build": { ... },
  "bundle": { ... }, 
  "plugins": { ... }
}
```

## 主要变化

### 根级配置

| v1 | v2 | 说明 |
|----|----|------|
| `package.productName` | `productName` | 移至根级 |
| `package.version` | `version` | 移至根级 |
| - | `identifier` | 新增，必填项，应用唯一标识符 |
| - | `mainBinaryName` | 新增，可选，主二进制文件名 |

### 核心配置项

| v1 | v2 | 说明 |
|----|----|------|
| `tauri` | `app` | 重命名 |
| `tauri.allowlist` | `app.security.capabilities` | 权限系统重构 |
| `tauri.bundle` | `bundle` | 移至根级 |
| `tauri.security` | `app.security` | 移动位置 |
| `tauri.updater` | `bundle.createUpdaterArtifacts` | 简化为布尔值 |
| `tauri.windows` | `app.windows` | 移动位置 |

### 开发构建配置

| v1 | v2 | 说明 |
|----|----|------|
| `build.devPath` | `build.devUrl` | 重命名 |
| `build.distDir` | `build.frontendDist` | 重命名 |
| `build.withGlobalTauri` | `app.withGlobalTauri` | 移至app配置 |
| - | `build.removeUnusedCommands` | 新增 |

### 安全配置

| v1 | v2 | 说明 |
|----|----|------|
| `tauri.security.csp` | `app.security.csp` | 位置变更 |
| `tauri.security.freezePrototype` | `app.security.freezePrototype` | 位置变更 |
| `tauri.security.dangerousDisableAssetCspModification` | `app.security.dangerousDisableAssetCspModification` | 位置变更 |
| `tauri.security.dangerousRemoteDomainIpcAccess` | - | 已移除 |
| `tauri.security.dangerousUseHttpScheme` | - | 已移除 |
| - | `app.security.pattern` | 新增安全模式配置 |
| - | `app.security.assetProtocol` | 新增资源协议安全配置 |

### 打包配置

| v1 | v2 | 说明 |
|----|----|------|
| `tauri.bundle.active` | `bundle.active` | 位置变更 |
| `tauri.bundle.targets` | `bundle.targets` | 位置变更 |
| `tauri.bundle.icon` | `bundle.icon` | 位置变更 |
| - | `bundle.createUpdaterArtifacts` | 新增，替代v1的updater配置 |
| - | `bundle.useLocalToolsDir` | 新增 |

### 平台特定配置

| v1 | v2 | 说明 |
|----|----|------|
| `tauri.bundle.windows` | `bundle.windows` | 位置变更 |
| `tauri.bundle.macOS` | `bundle.macOS` | 位置变更 |
| `tauri.bundle.linux` | `bundle.linux` | 位置变更 |
| - | `bundle.android` | 新增，支持Android平台 |
| - | `bundle.iOS` | 新增，支持iOS平台 |

### 窗口配置

v1和v2的窗口配置选项基本相同，但在v2中有以下新增属性：

| 新增属性 | 说明 |
|----------|------|
| `shadow` | 窗口是否有阴影 |
| `theme` | 窗口主题（light/dark） |
| `incognito` | 是否使用隐身模式 |
| `fileDropEnabled` | 是否允许文件拖放 |
| `label` | 窗口标签，用于API引用 |

## 文件格式与平台特定配置

### 文件格式
两个版本都支持：
- **JSON**: `tauri.conf.json`（默认）
- **JSON5**: `tauri.conf.json`或`tauri.conf.json5`
- **TOML**: `Tauri.toml`

### 平台特定配置

| v1 | v2 | 说明 |
|----|----|------|
| `tauri.linux.conf.json` | `tauri.linux.conf.json` | 无变化 |
| `tauri.windows.conf.json` | `tauri.windows.conf.json` | 无变化 |
| `tauri.macos.conf.json` | `tauri.macos.conf.json` | 无变化 |
| - | `tauri.android.conf.json` | 新增，Android平台配置 |
| - | `tauri.ios.conf.json` | 新增，iOS平台配置 |

## 迁移建议

从v1迁移到v2时，建议以下步骤：

1. 添加必需的`identifier`字段，采用反向域名表示法
2. 将`package`下的配置移至根级
3. 将`tauri`更改为`app`
4. 将`bundle`移至根级
5. 更新构建配置的字段名称
6. 将`allowlist`迁移到新的`capabilities`权限系统
7. 适配新的安全模型配置

## 参考资料

- [Tauri v1配置文档](https://tauri.app/v1/api/config)
- [Tauri v2配置文档](https://tauri.app/v2/api/config)
- [Tauri v1配置Schema](https://schema.tauri.app/config/1)
- [Tauri v2配置Schema](https://schema.tauri.app/config/2) 