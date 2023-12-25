// Static link need `__mingw_vfprintf` which is from MinGW,
// therefore MSVC toolchain can't work.
// Only CArchive is allowed on Windows platform.

fn main() {
  let mut build = cgo::Build::new();
  let mut rbuild = build.trimpath(true)
    .ldflags("-s -w")
    .build_mode(cgo::BuildMode::CArchive)
    .change_dir("libxray")
    .package("main.go");

  if cfg!(target_os = "windows") {
    rbuild = rbuild.goos("windows");
  }

  rbuild.build("xray");
}
