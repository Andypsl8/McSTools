# 🧰 基于 Tauri v2.0 的高效蓝图工具箱

[![Stars](https://img.shields.io/github/stars/guapi-exe/McSTools?style=flat-square&label=Stars)](https://github.com/guapi-exe/McSTools/stargazers)
[![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Framework-Tauri%202.0-blue)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-GPLv3-green)]()

> 基于 Rust 语言构建的跨平台工具箱，提供卓越的内存安全性与原生级性能

---

## 🚀 项目
- **高性能核心**: 依托 Rust 语言实现极致内存安全与高效计算
- **跨平台支持**: 使用 Tauri 框架构建，支持 Windows/macOS/Linux
- **模块化设计**: 功能组件可自由扩展，满足进阶开发需求

---

## 📌 已实现功能

### 蓝图管理
- **版本控制系统**  
  完整的蓝图迭代历史追踪与差异对比
- **智能材料解析**  
  自动统计建筑材料用量与资源消耗
- **数据洞察**  
  原生源数据

### 蓝图处理
- **格式转换引擎**  
  `支持4种主流蓝图格式互转`（*.blp ↔ *.blueprint ↔ ...）
- **智能方块替换**
    - 简单模式：保留目标方块属性仅替换ID
    - 精准模式：完全覆盖方块ID及属性集

### 创意工具
- **主题**  
  自定义界面主题颜色/布局/视觉元素
- **地图画生成器**  
  ▨ 平面模式：快速像素画转换  
  ▦ 立体模式：三维体素艺术构建

---

## 🔧 开发中功能
- **BE 蓝图适配**  
  基岩版蓝图解析与转换支持
- **蓝图分割器**  
  大型蓝图智能切割与重组模块

---

## 📅 计划开发路线
- **云端同步**  
  跨设备蓝图库同步（AWS S3/阿里云 OSS 支持）

---

![:浏览](https://count.getloli.com/@guapi-exe_McSTools?name=guapi-exe_McSTools&theme=original-new&padding=8&offset=0&align=top&scale=1&pixelated=1&darkmode=auto)
## 🛠️ 构建说明
```bash
# 安装依赖
pnpm install

# 开发模式运行
pnpm run tauri dev

# 生产环境构建
pnpm run tauri build