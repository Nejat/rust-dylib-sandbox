param (
    [bool] $release = $true,
    [bool] $run = $true
)

[string] $build = if ($release) {
    "--release"
} else {
    ""
}

Write-Host "Clean target"
cargo clean

if ($LastExitCode -ne 0)
{
    Write-Host
    Write-Error "clean command failed"
    exit -1
}

Write-Host "Clippy $build"
Write-Host
cargo clippy $build -- -D clippy::all -D clippy::pedantic -D clippy::nursery -A clippy::missing-errors-doc -A clippy::must-use-candidate

if ($LastExitCode -ne 0)
{
    Write-Host
    Write-Error "clippy command failed"
    exit -2
}

Write-Host
Write-Host "Tests $build"
Write-Host
cargo test --all $build

if ($LastExitCode -ne 0)
{
    Write-Host
    Write-Error "build command failed"
    exit -3
}

Write-Host
Write-Host "Build $build"
Write-Host
cargo build $build

if ($LastExitCode -ne 0)
{
    Write-Host
    Write-Error "run command failed"
    exit -4
}

if ($run) {
    Write-Host
    Write-Host "Run $build"
    Write-Host
    cargo run $build
    Write-Host
}