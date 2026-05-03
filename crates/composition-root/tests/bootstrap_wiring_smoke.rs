use std::path::Path;

use launcher_composition_root::{build_desktop_services, DesktopBootstrapConfig};

#[test]
fn bootstrap_wiring_smoke() {
    let services = build_desktop_services(DesktopBootstrapConfig::default())
        .expect("composition root should assemble desktop app services in D2");

    assert_eq!(
        services.fab.deps().projection_repo.config().database_path(),
        Path::new("launcher.sqlite3")
    );
    assert_eq!(
        services.downloads.deps().job_repo.config().database_path(),
        Path::new("launcher.sqlite3")
    );
    assert_eq!(
        services
            .fab
            .deps()
            .catalog_provider
            .config()
            .client_name(),
        "my-epic-launcher-desktop"
    );
    let _ = services.startup.clone();
}