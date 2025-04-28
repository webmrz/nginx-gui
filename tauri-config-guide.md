# Tauri v2 配置全解析（完整版）

## 配置文件基础

Tauri 支持以下配置文件格式：
- **JSON格式**：就是默认的`tauri.conf.json`
- **JSON5格式**：可以是`tauri.conf.json`或`tauri.conf.json5`（不过得先启用`config-json5`这个Cargo特性）
- **TOML格式**：文件名是`Tauri.toml`（同样需要启用`config-toml`特性）

你还可以为不同平台创建单独的配置文件，比如`tauri.windows.conf.json`、`tauri.macos.conf.json`、`tauri.linux.conf.json`、`tauri.android.conf.json`和`tauri.ios.conf.json`，这些会和主配置文件合并起来用。

## 基础配置项

### 根级配置

| 配置项 | 必填? | 说明 |
|--------|------|------|
| `identifier` | 必填 | 你app的唯一标识，写成反向域名形式，如`com.tauri.example`。只能用字母、数字、连字符和点 |
| `productName` | 选填 | app的显示名称，会出现在菜单栏和标题栏上 |
| `version` | 选填 | app版本号，可以直接写语义化版本号，也可以指向`package.json`文件路径。不写的话就用`Cargo.toml`里的版本 |
| `mainBinaryName` | 选填 | 主程序文件名。默认就是你Cargo项目名称 |

## app配置

`app`这部分包含了app的主要设置：

```json
"app": {
  "security": { ... },
  "windows": [ ... ],
  "withGlobalTauri": false,
  "enableGTKAppId": false,
  "macOSPrivateApi": false
}
```

### app.windows配置

这里定义app窗口的各种属性，是个数组，每个窗口可以有这些设置：

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `width` | 800 | 窗口宽度，单位是像素 |
| `height` | 600 | 窗口高度，单位是像素 |
| `x` | null | 窗口在屏幕上的X坐标 |
| `y` | null | 窗口在屏幕上的Y坐标 |
| `center` | false | 是否让窗口居中显示 |
| `resizable` | true | 能不能调整窗口大小 |
| `title` | "Tauri App" | 窗口标题文字 |
| `fullscreen` | false | 是否全屏显示 |
| `focus` | true | 窗口创建时是否获得焦点 |
| `maximized` | false | 窗口是否默认最大化 |
| `visible` | true | 窗口是否可见 |
| `decorations` | true | 是否显示窗口边框和控制按钮 |
| `alwaysOnTop` | false | 窗口是否总是显示在最上层 |
| `skipTaskbar` | false | 是否在任务栏隐藏图标 |
| `transparent` | false | 窗口是否透明 |
| `shadow` | true | 窗口是否有阴影效果 |
| `theme` | null | 窗口主题，可选"light"或"dark" |
| `minWidth` | null | 窗口最小宽度 |
| `minHeight` | null | 窗口最小高度 |
| `maxWidth` | null | 窗口最大宽度 |
| `maxHeight` | null | 窗口最大高度 |
| `closable` | true | 窗口是否可以被关闭 |
| `fileDropEnabled` | true | 是否允许拖放文件到窗口 |
| `acceptFirstMouse` | false | 在macOS上是否接受首次鼠标点击 |
| `tabbingIdentifier` | null | macOS上的标签标识符 |
| `titleBarStyle` | "Visible" | 标题栏样式，可选"Visible"、"Transparent"或"Overlay" |
| `hiddenTitle` | false | 在macOS上是否隐藏标题 |
| `url` | "index.html" | 窗口加载的URL或本地文件路径 |
| `userAgent` | null | 自定义用户代理字符串 |
| `incognito` | false | 是否使用隐身模式 |
| `additionalBrowserArgs` | null | 传给浏览器的额外参数 |
| `label` | null | 窗口标签，用于通过API找到特定窗口 |
| `contentProtected` | false | 是否保护窗口内容不被截图或录屏 |
| `cursor` | null | 默认的鼠标指针样式 |
| `startupArguments` | [] | 传递给窗口的启动参数 |
| `windowEffects` | null | 窗口视觉效果设置，如毛玻璃效果 |
| `serviceWorkerScript` | null | 自定义Service Worker脚本路径 |
| `maximizable` | true | 窗口是否可以最大化 |
| `minimizable` | true | 窗口是否可以最小化 |
| `webviewAttributes` | {} | WebView的额外属性设置 |
| `parent` | null | 父窗口的标签，用于创建子窗口 |
| `windowEffectsState` | null | 窗口效果的默认状态 |
| `owner` | null | 窗口的所有者标签 |

### app.security配置

这里定义app的安全相关设置：

```json
"security": {
  "csp": null,
  "freezePrototype": false,
  "dangerousDisableAssetCspModification": false,
  "pattern": {
    "use": "brownfield"
  },
  "assetProtocol": {
    "enable": false,
    "scope": []
  },
  "capabilities": []
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `csp` | null | 内容安全策略，控制页面可以加载哪些资源 |
| `freezePrototype` | false | 是否冻结JS原型链防止原型污染 |
| `dangerousDisableAssetCspModification` | false | 是否禁用Tauri对CSP的自动修改（小心用） |
| `pattern.use` | "brownfield" | 安全模式，"brownfield"（默认）或"isolation" |
| `pattern.options` | null | 安全模式的其他选项 |
| `assetProtocol.enable` | false | 是否启用资源协议 |
| `assetProtocol.scope` | [] | 资源协议的作用范围 |
| `capabilities` | [] | app的权限设置，如文件访问、网络请求等 |

#### Capabilities（权限）详解

权限控制app能做什么，这是v2版本新增的重要功能：

```json
"capabilities": [
  {
    "name": "path",
    "path": {
      "allow": ["$DOWNLOAD/*", "$PICTURE/*"],
      "deny": [],
      "requireLiteralLeadingDot": false
    }
  },
  {
    "name": "window"
  },
  {
    "name": "app"
  }
]
```

常用的权限包括：
- `path`：文件系统访问权限
- `window`：窗口操作权限
- `app`：应用操作权限
- `shell`：执行外部命令权限
- `http`：HTTP请求权限
- `notification`：通知权限
- `clipboard`：剪贴板访问权限
- `dialog`：对话框权限
- `os`：操作系统信息权限

### 其他app配置项

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `withGlobalTauri` | false | 是否在全局作用域暴露Tauri API |
| `enableGTKAppId` | false | 是否启用GTK应用ID |
| `macOSPrivateApi` | false | 是否启用macOS私有API |
| `trayIcon` | null | 系统托盘图标设置 |

## build配置

这里定义app的构建相关设置：

```json
"build": {
  "beforeDevCommand": "",
  "beforeBuildCommand": "",
  "devUrl": "http://localhost:3000",
  "frontendDist": "../dist",
  "devServerPort": null,
  "removeUnusedCommands": false
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `beforeDevCommand` | "" | 开发模式前执行的命令，比如启动前端开发服务器 |
| `beforeBuildCommand` | "" | 构建前执行的命令，比如构建前端代码 |
| `devUrl` | "http://localhost:3000" | 开发服务器的URL |
| `frontendDist` | "../dist" | 前端构建后文件所在目录 |
| `devServerPort` | null | 开发服务器端口，null表示自动选择 |
| `devPath` | null | 开发时前端静态文件目录（优先级高于devUrl） |
| `removeUnusedCommands` | false | 是否删除未使用的命令 |

## bundle配置

这里定义app的打包设置：

```json
"bundle": {
  "active": false,
  "targets": "all",
  "icon": [],
  "resources": [],
  "copyright": "",
  "category": null,
  "shortDescription": "",
  "longDescription": "",
  "createUpdaterArtifacts": false,
  "useLocalToolsDir": false,
  "publisher": null,
  "windows": { ... },
  "macOS": { ... },
  "linux": { ... },
  "android": { ... },
  "iOS": { ... }
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `active` | false | 是否启用打包功能 |
| `targets` | "all" | 打包的目标平台，可以是"all"或特定平台数组 |
| `icon` | [] | app图标文件路径数组 |
| `resources` | [] | 需要包含的额外资源文件路径 |
| `copyright` | "" | 版权信息 |
| `category` | null | 应用类别 |
| `shortDescription` | "" | 应用简短描述 |
| `longDescription` | "" | 应用详细描述 |
| `createUpdaterArtifacts` | false | 是否创建自动更新所需文件 |
| `useLocalToolsDir` | false | 是否使用本地工具目录 |
| `publisher` | null | 发布者信息 |
| `externalBin` | [] | 要包含的外部二进制文件 |
| `appimage` | {} | AppImage特定配置 |
| `deb` | {} | DEB特定配置 |
| `rpm` | {} | RPM特定配置 |

### bundle.windows配置

Windows平台的打包设置：

```json
"windows": {
  "allowDowngrades": true,
  "certificateThumbprint": null,
  "digestAlgorithm": null,
  "signCommand": null,
  "timestampUrl": null,
  "tsp": false,
  "webviewInstallMode": {
    "silent": true,
    "type": "downloadBootstrapper"
  },
  "wix": null,
  "nsis": null,
  "installer": null,
  "installerArgs": [],
  "installerIcon": null,
  "languages": [],
  "skipWebviewInstall": false,
  "signingArguments": [],
  "shortcutName": null,
  "signAndInstall": false,
  "providers": {}
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `allowDowngrades` | true | 是否允许安装低版本覆盖高版本 |
| `certificateThumbprint` | null | 签名证书指纹 |
| `digestAlgorithm` | null | 摘要算法 |
| `signCommand` | null | 签名命令 |
| `timestampUrl` | null | 时间戳服务器URL |
| `tsp` | false | 是否使用TSP时间戳 |
| `webviewInstallMode.type` | "downloadBootstrapper" | WebView2安装方式，可选"downloadBootstrapper"、"offlineInstaller"或"fixedRuntime" |
| `webviewInstallMode.silent` | true | 是否静默安装WebView2 |
| `wix` | null | WiX安装程序设置 |
| `nsis` | null | NSIS安装程序设置 |
| `installer` | null | 自定义安装程序路径 |
| `installerArgs` | [] | 安装程序参数 |
| `installerIcon` | null | 安装程序图标 |
| `languages` | [] | 安装程序支持的语言 |
| `skipWebviewInstall` | false | 是否跳过WebView2安装 |
| `signingArguments` | [] | 签名额外参数 |
| `shortcutName` | null | 快捷方式名称 |
| `signAndInstall` | false | 签名后是否立即安装 |
| `providers` | {} | 文件类型关联提供程序 |

#### bundle.windows.wix配置

WiX安装程序的详细设置：

```json
"wix": {
  "language": ["en-US"],
  "template": null,
  "fragmentPaths": [],
  "componentRefs": [],
  "disableWixPdbOutput": false,
  "license": null,
  "bannerPath": null,
  "dialogImagePath": null
}
```

| 配置项 | 说明 |
|--------|------|
| `language` | 安装程序语言 |
| `template` | 自定义WiX模板文件路径 |
| `fragmentPaths` | WiX片段文件路径 |
| `componentRefs` | 组件引用列表 |
| `disableWixPdbOutput` | 是否禁用PDB输出 |
| `license` | 许可证文件路径 |
| `bannerPath` | 安装程序横幅图片路径 |
| `dialogImagePath` | 对话框图片路径 |

#### bundle.windows.nsis配置

NSIS安装程序的详细设置：

```json
"nsis": {
  "installerIcon": null,
  "headerImage": null,
  "sidebarImage": null,
  "installMode": "currentUser",
  "license": null,
  "languages": [],
  "installDir": null,
  "template": null,
  "displayLanguageSelector": false,
  "scriptTemplate": null
}
```

| 配置项 | 说明 |
|--------|------|
| `installerIcon` | 安装程序图标 |
| `headerImage` | 顶部图片 |
| `sidebarImage` | 侧边栏图片 |
| `installMode` | 安装模式，"currentUser"或"perMachine" |
| `license` | 许可证文件路径 |
| `languages` | 支持的语言列表 |
| `installDir` | 安装目录 |
| `template` | 自定义模板 |
| `displayLanguageSelector` | 是否显示语言选择器 |
| `scriptTemplate` | 自定义脚本模板 |

### bundle.macOS配置

macOS平台的打包设置：

```json
"macOS": {
  "frameworks": null,
  "files": {},
  "bundleVersion": null,
  "minimumSystemVersion": "10.13",
  "exceptionDomain": null,
  "signingIdentity": null,
  "hardenedRuntime": true,
  "providerShortName": null,
  "entitlements": null,
  "licenseFile": null,
  "appCategory": null,
  "license": null,
  "appAttr": {},
  "signingFlags": [],
  "notarizeTeamId": null,
  "dmg": { ... }
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `frameworks` | null | 需要打包的macOS框架列表 |
| `files` | {} | 要包含在app中的额外文件 |
| `bundleVersion` | null | 构建版本号，对应CFBundleVersion |
| `minimumSystemVersion` | "10.13" | 支持的最低macOS版本 |
| `exceptionDomain` | null | app通信域名 |
| `signingIdentity` | null | 代码签名身份 |
| `hardenedRuntime` | true | 是否启用强化运行时保护 |
| `providerShortName` | null | 公证提供商简称 |
| `entitlements` | null | 权限配置文件路径 |
| `licenseFile` | null | 许可证文件路径 |
| `appCategory` | null | 应用类别，如"public.app-category.developer-tools" |
| `license` | null | 许可证文本 |
| `appAttr` | {} | 应用属性 |
| `signingFlags` | [] | 签名额外参数 |
| `notarizeTeamId` | null | 公证团队ID |

#### bundle.macOS.dmg配置

DMG镜像的特定设置：

```json
"dmg": {
  "background": null,
  "windowPosition": null,
  "windowSize": {
    "width": 660,
    "height": 400
  },
  "appPosition": {
    "x": 180,
    "y": 170
  },
  "applicationFolderPosition": {
    "x": 480,
    "y": 170
  },
  "formats": ["ULFO"],
  "compressionLevel": null,
  "compressionMethod": null,
  "signing": null
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `background` | null | DMG背景图片路径 |
| `windowPosition` | null | DMG安装窗口位置 |
| `windowSize.width` | 660 | DMG窗口宽度 |
| `windowSize.height` | 400 | DMG窗口高度 |
| `appPosition.x` | 180 | 应用图标X坐标 |
| `appPosition.y` | 170 | 应用图标Y坐标 |
| `applicationFolderPosition.x` | 480 | 应用程序文件夹X坐标 |
| `applicationFolderPosition.y` | 170 | 应用程序文件夹Y坐标 |
| `formats` | ["ULFO"] | DMG格式 |
| `compressionLevel` | null | 压缩级别 |
| `compressionMethod` | null | 压缩方法 |
| `signing` | null | 签名设置 |

### bundle.linux配置

Linux平台的打包设置：

```json
"linux": {
  "appimage": {
    "bundleMediaFramework": false,
    "files": {}
  },
  "deb": {
    "files": {},
    "depends": [],
    "section": null,
    "priority": "optional",
    "desktopTemplate": null,
    "scripts": {
      "postinst": null,
      "preinst": null,
      "postrm": null,
      "prerm": null
    }
  },
  "rpm": {
    "epoch": 0,
    "release": "1",
    "files": {},
    "depends": [],
    "scripts": {
      "pretrans": null,
      "posttrans": null,
      "preinstall": null,
      "postinstall": null,
      "preremove": null,
      "postremove": null
    }
  },
  "flatpak": {
    "runtime": "org.freedesktop.Platform",
    "runtimeVersion": "23.08",
    "sdk": "org.freedesktop.Sdk",
    "base": null,
    "baseVersion": null,
    "metainfo": null,
    "finishArgs": []
  }
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `appimage.bundleMediaFramework` | false | 是否打包媒体框架 |
| `appimage.files` | {} | AppImage包含的额外文件 |
| `deb.files` | {} | DEB包含的额外文件 |
| `deb.depends` | [] | DEB依赖包 |
| `deb.section` | null | 软件包分类 |
| `deb.priority` | "optional" | 软件包优先级 |
| `deb.desktopTemplate` | null | 自定义桌面模板 |
| `deb.scripts.*` | null | DEB安装脚本 |
| `rpm.epoch` | 0 | RPM的epoch值 |
| `rpm.release` | "1" | RPM的release值 |
| `rpm.files` | {} | RPM包含的额外文件 |
| `rpm.depends` | [] | RPM依赖包 |
| `rpm.scripts.*` | null | RPM安装脚本 |
| `flatpak.*` | - | Flatpak包设置 |

### bundle.android配置

Android平台的打包设置：

```json
"android": {
  "minSdkVersion": 24,
  "versionCode": null,
  "icon": null,
  "appPermissions": null,
  "signingConfig": {
    "storeFile": null,
    "storePassword": null,
    "keyAlias": null,
    "keyPassword": null
  },
  "useDebug": null,
  "androidManifestPath": null
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `minSdkVersion` | 24 | 最低支持的Android SDK版本 |
| `versionCode` | null | 应用版本代码，必须小于21亿 |
| `icon` | null | 应用图标 |
| `appPermissions` | null | 应用权限列表 |
| `signingConfig.storeFile` | null | 密钥库文件路径 |
| `signingConfig.storePassword` | null | 密钥库密码 |
| `signingConfig.keyAlias` | null | 密钥别名 |
| `signingConfig.keyPassword` | null | 密钥密码 |
| `useDebug` | null | 是否使用调试模式 |
| `androidManifestPath` | null | 自定义AndroidManifest.xml路径 |

### bundle.iOS配置

iOS平台的打包设置：

```json
"iOS": {
  "template": null,
  "frameworks": null,
  "developmentTeam": null,
  "bundleVersion": null,
  "minimumSystemVersion": "13.0",
  "appPermissions": null,
  "infoPlist": null,
  "xcodeProjPath": null,
  "iCloudContainerEnvironment": null,
  "codesigningIdentity": null,
  "provisioningProfile": null
}
```

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `template` | null | 自定义XcodeGen模板路径 |
| `frameworks` | null | 需要打包的iOS框架列表 |
| `developmentTeam` | null | 开发团队ID（这个必须填） |
| `bundleVersion` | null | 构建版本号，对应CFBundleVersion |
| `minimumSystemVersion` | "13.0" | 支持的最低iOS版本 |
| `appPermissions` | null | 应用权限列表 |
| `infoPlist` | null | 自定义Info.plist设置 |
| `xcodeProjPath` | null | 自定义Xcode项目路径 |
| `iCloudContainerEnvironment` | null | iCloud容器环境设置 |
| `codesigningIdentity` | null | 代码签名身份 |
| `provisioningProfile` | null | 预配置文件 |

## plugins配置

插件配置，可以为不同插件设置不同参数：

```json
"plugins": {
  "shell": {
    "open": true,
    "scope": [
      {
        "name": "open-browser",
        "cmd": "open",
        "args": true
      }
    ]
  },
  "http": {
    "all": true,
    "request": true,
    "scope": [
      "https://api.github.com/*"
    ]
  }
}
```

常见插件配置项：

1. **shell插件**：控制命令行执行权限
   - `open`：是否允许打开外部程序
   - `scope`：允许执行的命令范围

2. **http插件**：控制网络请求权限
   - `all`：是否允许所有请求
   - `request`：是否允许发送请求
   - `scope`：允许的URL范围

3. **fs插件**：控制文件系统访问
   - `scope`：允许访问的路径
   - `allow`：允许的操作类型

4. **dialog插件**：控制文件选择对话框
   - `open`：是否允许打开文件对话框
   - `save`：是否允许保存文件对话框

## 实用技巧

1. **开发环境设置**：
   - 用`build.devUrl`指向前端开发服务器
   - 用`build.beforeDevCommand`自动启动前端服务器，如`"npm run dev"`
   - 开发时可以用`tauri dev`命令启动应用

2. **打包部署设置**：
   - 用`build.frontendDist`指向前端构建好的文件夹
   - 用`build.beforeBuildCommand`自动构建前端代码，如`"npm run build"`
   - 配置`bundle.targets`选择要打包的平台
   - 打包时用`tauri build`命令

3. **安全设置**：
   - 配置`app.security.csp`设置内容安全策略
   - 用`app.security.capabilities`控制app权限，最小权限原则
   - 使用`security.freezePrototype`防止原型污染

4. **多窗口app**：
   - 在`app.windows`数组中定义多个窗口
   - 用`label`给每个窗口设置唯一标识
   - 可以设置窗口之间的父子关系

5. **自动更新设置**：
   - 把`bundle.createUpdaterArtifacts`设为`true`启用自动更新
   - 确保版本号格式正确，遵循语义化版本规则
   - 需要配合适当的更新服务器

6. **跨平台适配**：
   - 使用平台特定配置文件微调不同平台行为
   - 图标准备多种大小和格式，适配不同平台
   - 考虑不同操作系统的权限模型

7. **性能优化**：
   - 使用`window.contentProtected`减少不必要的截图能力
   - 合理设置`window.transparent`和特效，避免性能损耗
   - 使用`window.shadow`可以提高视觉效果但会影响性能

## 示例配置

### 最小配置

```json
{
  "identifier": "com.example.myapp",
  "build": {
    "devUrl": "http://localhost:3000",
    "frontendDist": "../dist"
  },
  "app": {
    "security": {
      "csp": "default-src 'self'"
    },
    "windows": [
      {
        "width": 800,
        "height": 600,
        "title": "我的Tauri应用"
      }
    ]
  }
}
```

### 生产环境配置

```json
{
  "identifier": "com.example.myapp",
  "productName": "我的应用",
  "version": "1.0.0",
  "build": {
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "security": {
      "csp": "default-src 'self'; connect-src 'self' https://api.example.com",
      "capabilities": [
        {
          "name": "window"
        },
        {
          "name": "path",
          "path": {
            "allow": ["$DOWNLOAD/*"]
          }
        },
        {
          "name": "http",
          "http": {
            "scope": ["https://api.example.com/*"]
          }
        }
      ]
    },
    "windows": [
      {
        "width": 900,
        "height": 700,
        "title": "我的应用",
        "center": true,
        "resizable": true
      }
    ]
  },
  "bundle": {
    "active": true,
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png"],
    "targets": ["deb", "appimage", "msi", "app", "dmg"],
    "publisher": "我的公司",
    "copyright": "© 2023 我的公司",
    "shortDescription": "一个很棒的Tauri应用",
    "longDescription": "这是一个使用Tauri和Web技术构建的跨平台应用"
  }
}
```

## 参考链接

- [Tauri官方配置文档](https://tauri.app/v2/api/config)
- [Tauri配置JSON Schema](https://schema.tauri.app/config/2) 