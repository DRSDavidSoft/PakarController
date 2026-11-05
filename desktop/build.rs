#[path = "src/application_identity.rs"]
mod application_identity;
#[path = "src/brand_image.rs"]
mod brand_image;
#[path = "src/platform/windows_quick_actions.rs"]
mod windows_quick_actions;
#[path = "src/platform/windows_shell_icons.rs"]
mod windows_shell_icons;
fn main() {
    println!("cargo:rerun-if-env-changed=PAKAR_ENFORCE_PEER_VERSIONS");
    println!("cargo:rerun-if-env-changed=PAKAR_BUILD_ID");
    println!("cargo:rerun-if-changed=../interfaces/peer-policy.json");
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(
        out.join("pakarcontroller.ico"),
        brand_image::native_icon_bytes(brand_image::icon_bytes()).expect("shared application ICO"),
    )
    .unwrap();
    for size in [32, 64] {
        std::fs::write(
            out.join(format!("pakar-icon-{size}.rgba")),
            brand_image::rgba_at_size(size),
        )
        .unwrap();
    }
    for action in windows_quick_actions::WINDOWS_QUICK_ACTIONS {
        std::fs::write(
            out.join(format!("quick-action-{}.rgba", action.icon_resource_id)),
            windows_shell_icons::shell_icon_rgba(action.icon_glyph, 16),
        )
        .unwrap();
    }
    #[cfg(windows)]
    {
        let mut resources = winres::WindowsResource::new();
        resources
            .set("FileDescription", application_identity::APP_NAME)
            .set("ProductName", application_identity::APP_NAME)
            .set("InternalName", application_identity::WINDOWS_APP_ID)
            .set("CompanyName", application_identity::APP_NAME)
            .set_icon(out.join("pakarcontroller.ico").to_str().unwrap());
        for action in windows_quick_actions::WINDOWS_QUICK_ACTIONS {
            let file = out.join(format!("quick-action-{}.ico", action.icon_resource_id));
            std::fs::write(
                &file,
                windows_shell_icons::shell_icon_ico(action.icon_glyph),
            )
            .unwrap();
            resources
                .set_icon_with_id(file.to_str().unwrap(), &action.icon_resource_id.to_string());
        }
        resources
            .compile()
            .expect("PakarController executable resources");
    }
    println!("cargo:rerun-if-changed=src/application_identity.rs");
    println!("cargo:rerun-if-changed=src/platform/windows_shell_icons.rs");
    println!("cargo:rerun-if-changed=src/platform/windows_quick_actions.rs");
    println!("cargo:rerun-if-changed=src/brand_image.rs");
    println!("cargo:rerun-if-changed=../assets/branding/app-icon.png.base64");
    println!("cargo:rerun-if-changed=../assets/branding/app-icon-1024.png");
}
