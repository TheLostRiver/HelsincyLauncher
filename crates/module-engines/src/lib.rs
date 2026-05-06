//! Engines 模块的 crate 入口与公开导出边界。
//!
//! 该文件把 contracts、driver、facade 三个子边界收口到同一个稳定入口，
//! 让上层装配只依赖 engines 模块公开表面，而不必分散导入内部子模块路径。

pub mod contracts;
pub mod driver;
pub mod facade;

pub use driver::EngineJobDriver;
pub use facade::{EngineFacade, EngineModuleDeps};
