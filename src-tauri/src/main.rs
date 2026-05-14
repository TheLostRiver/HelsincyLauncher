//! Tauri desktop binary 入口。
//!
//! 当前 binary 只委托到可测试的桌面宿主 bootstrap；真实宿主装配与 transport
//! 边界继续收在 `my_epic_launcher_desktop` lib crate 内，避免 main 入口直接持有业务接线。

fn main() {
    my_epic_launcher_desktop::run_desktop_host()
        .expect("desktop host bootstrap should succeed in E2");
}
