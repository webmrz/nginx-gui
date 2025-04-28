# Nginx GUI

一个基于 Tauri 的 Nginx 图形化管理工具，提供直观的界面来管理 Nginx 配置和监控服务状态。

## 功能特性

### 配置文件管理
- 查看和编辑 Nginx 配置文件
- 语法高亮和自动格式化
- 配置文件测试和验证
- 支持创建、导入、导出和删除配置文件
- 配置文件类型：主配置、站点配置、自定义配置

### 日志查看
- 支持查看访问日志、错误日志和系统日志
- 实时日志自动刷新
- 日志搜索和过滤功能
- 按日志级别过滤
- 分页浏览大量日志
- 支持日志下载和清除

### 性能监控
- 实时监控 CPU 和内存使用率
- 显示请求数和连接数统计
- 性能趋势图表展示
- 自动刷新监控数据

### 系统设置
- 主题设置（浅色/深色/跟随系统）
- 语言设置（中文/英文）
- 开机自动启动
- 日志级别设置
- 最大日志大小设置

## 技术栈

- 前端框架：Vue 3 + TypeScript
- UI 组件库：Naive UI
- 构建工具：Tauri
- 代码编辑器：Monaco Editor
- 图表库：ECharts

## 开发环境要求

- Node.js 16+
- Rust 1.70+
- Tauri CLI
- 操作系统：Windows/macOS/Linux

## 安装和运行

1. 克隆项目
```bash
git clone https://github.com/yourusername/nginx-gui.git
cd nginx-gui
```

2. 安装依赖
```bash
bun install
```

3. 启动开发服务器
```bash
bun tauri dev
```

4. 构建应用
```bash
bun tauri build
```

## 项目结构

```
src/
├── components/        # 组件目录
│   ├── Layout/       # 布局组件
│   ├── Nginx/        # Nginx 相关组件
│   └── Utils/        # 工具组件
├── views/            # 页面组件
├── stores/           # 状态管理
├── router/           # 路由配置
└── App.vue           # 根组件
```

## 主要功能模块

### 配置文件管理
- `ConfigEditor.vue`: 配置文件编辑器组件
- `Configs.vue`: 配置文件管理页面

### 日志查看
- `Logs.vue`: 日志查看页面
- `LogViewer.vue`: 日志查看器组件

### 性能监控
- `Metrics.vue`: 性能监控页面
- `MetricsChart.vue`: 性能图表组件

### 系统设置
- `Settings.vue`: 系统设置页面

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情 
 