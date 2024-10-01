# rs-axum-admin

Rust API 快速开发脚手架

- 路由使用 [axum](https://github.com/tokio-rs/axum)
- ORM使用 [sea-orm](https://github.com/SeaQL/sea-orm)
- 日志使用 [tracing](https://github.com/tokio-rs/tracing)
- 配置使用 [config-rs](https://github.com/mehcode/config-rs)
- 命令行使用 [clap](https://github.com/clap-rs/clap)
- 异步运行时使用 [tokio](https://github.com/tokio-rs/tokio)
- 参数验证器使用 [validator](https://github.com/Keats/validator)
- 包含基于 JWT 的登录授权功能
- 包含 认证、请求日志、跨域 中间价
- 包含 Hash、时间格式化 等实用封装
- 包含 基础的权限模块、角色模块、菜单模块、部门模块、用户模块、岗位模块
- 简单好用的 API Result 统一输出方式

#### 1. 模块说明

- app => 应用模块
- pkg => 公共模块

#### 2. 本地运行

```sh
# 数据库
demo_rs.sql

# 配置文件
mv config.toml.example config.toml

# 启动服务
cargo run -- serve

#### 3. 代码持续更新中
```
