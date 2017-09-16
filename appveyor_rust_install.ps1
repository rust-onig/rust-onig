##### Appveyor Rust Install Script #####

# https://github.com/starkat99/appveyor-rust

# This is the most important part of the Appveyor configuration. This installs the version of Rust
# specified by the "channel" and "target" environment variables from the build matrix. By default,
# Rust will be installed to C:\Rust for easy usage, but this path can be overridden by setting the
# RUST_INSTALL_DIR environment variable. The URL to download rust distributions defaults to
# https://static.rust-lang.org/dist/ but can overridden by setting the RUST_DOWNLOAD_URL environment
# variable.
#
# For simple configurations, instead of using the build matrix, you can override the channel and
# target environment variables with the --channel and --target script arguments.
#
# If no channel or target arguments or environment variables are specified, will default to stable
# channel and x86_64-pc-windows-msvc target.

param([string]$channel=${env:channel}, [string]$target=${env:target})

# Initialize our parameters from arguments and environment variables, falling back to defaults
if (!$channel) {
    $channel = "stable"
}
if (!$target) {
    $target = "x86_64-pc-windows-msvc"
}

$toolchain = "${channel}-${target}"

echo "Downloading rustup"

appveyor DownloadFile "https://win.rustup.rs/" -FileName "rustup-init.exe" 2>&1
./rustup-init.exe -y --default-host=x86_64-pc-windows-msvc 2>&1

$env:PATH+=";C:\Users\appveyor\.cargo\bin"

if ($toolchain != "x86_64-pc-windows-msvc") {
	echo "Setting default toolchain to ${toolchain}"

	rustup default $toolchain
}

echo "Installation of $channel Rust $target completed"

# Test and display installed version information for rustc and cargo
rustc -vV
cargo -vV
rustup -vV
