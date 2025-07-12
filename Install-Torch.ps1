# https://gist.github.com/TeamDman/f05ec9944b956e21fd1ec120af6adbad
uv sync
$index_url = "https://download.pytorch.org/whl/cu128"
$packages = @(
    "torch",
    "torchvision",
    "torchaudio"
)
$to_add = @(
    "markupsafe<3.0.2" # fuck python package management, windows fails without this
    # https://github.com/astral-sh/uv/issues/11532#issuecomment-2660933703
    # error: Distribution `markupsafe==3.0.2 @ registry+https://download.pytorch.org/whl/cu128` can't be installed because it doesn't have a source distribution or wheel for the current platform
)
foreach ($package in $packages) {
    Write-Host "Checking for '$package'"
    $data = pip3 index versions "$package" --index-url "$index_url" --json | ConvertFrom-Json
    $package_version = "$package==$($data.latest)"
    $to_add += @($package_version)
    Write-Host "Found $package_version"
}

Write-Host "uv add $to_add" --index-url "$index_url"
$confirmation = Read-Host "Proceed with installing the following packages? $($to_add -join ', ') [y/n]"
if ($confirmation -ne 'y') {
    Write-Host "Installation cancelled."
    exit
}
uv add @to_add --index-url "$index_url"