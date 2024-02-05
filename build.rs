#[cfg(windows)]
extern crate winres;

#[cfg(all(windows, not(debug_assertions)))]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#,
    );
    res.compile().unwrap();
}

#[cfg(not(all(windows, not(debug_assertions))))]
fn main() {
    println!("Not on Windows, or in debug mode. No need to compile the executable with an admin manifest.")
}
