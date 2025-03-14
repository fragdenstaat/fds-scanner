fn main() {
    tauri_build::build();
    let associated_domains = [
        "applinks:fragdenstaat.de",
        "applinks:app.fragdenstaat.de",
        "webcredentials:fragdenstaat.de",
        "webcredentials:app.fragdenstaat.de",
    ];
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        tauri_plugin::mobile::update_entitlements(|entitlements| {
            entitlements.insert(
                "com.apple.developer.associated-domains".into(),
                associated_domains
                    .into_iter()
                    .map(|d| d.into())
                    .collect::<Vec<_>>()
                    .into(),
            );
        })
        .expect("failed to update entitlements");
    }
}
