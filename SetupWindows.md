# Rust | Setup for Windows 11

## Environment

- OS: Windows 11
- Shell: PowerShell
    - PackageManager: Chocolatey
- Rust: 1.55.0
    - EvCxR Jupyter: 0.10.0 (JupyterLab REPL環境で使いたい場合用)
- Julia: 1.6.3 (Anaconda + JupyterLab 環境用; REPL環境不要であれば必要ない)
    - Conda.jl
        - JupyterLab

### Setup
- Windowsスタートボタン 右クリック > `Windows ターミナル (管理者)`
    - |> 管理者権限 PowerShell 起動

```powershell
# if you don't have chocolatey package manager, install it
> Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# install msvc 2019 build tools
## installation will take a long time, wait a while
> choco install -y visualstudio2019buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --includeOptional --passive"

# reboot computer
> shutdown /r /t 0

# after rebooting computer, install rustup (rust version manager)
> choco install --pre -y rustup

# => install to ~/.cargo/
# => restart powershell for reflecting environmental variables: `PATH` etc

# confirm rust compiler version
> rustc --version
rustc 1.55.0 (c8dfcfe04 2021-09-06)

# install openssl for building cargo-generate
> choco install -y openssl

## => installed to: C:\Program Files\OpenSSL-Win64

# => restart powershell for reflecting environmental variables

# after restarting powershell, upgrade rust to the latest version
> rustup update

# install cargo-generate crate
## * cargo-generate: enable `cargo generate` command to generate rust project from template
## * require `openssl`: (`choco install -y openssl`)
> cargo install cargo-generate

# install cargo-edit crate
## * cargo-edit: a utility for managing cargo dependencies from the command line
### e.g.) `cargo add <crate>`: add crate dependencies into Cargo.toml automatically
> cargo install cargo-edit
```

### Setup JupyterLab REPL environment
```powershell
# install julia
> choco install -y julia

# install Conda.jl (Anaconda python for Julia)
> julia -e 'using Pkg; Pkg.add("Conda")'

# install JupyterLab by Conda.jl
# $ conda install -y -c conda-forge jupyterlab
> julia -e 'using Conda; Conda.add("jupyterlab"; channel="conda-forge")'

# setup PATH to ~/.julia/conda/3/Library/bin
> [System.Environment]::SetEnvironmentVariable("PATH", [System.Environment]::GetEnvironmentVariable("PATH", "User") + ";${ENV:USERPROFILE}\.julia\conda\3\Scripts;${ENV:USERPROFILE}\.julia\conda\3\Library\bin", "User")
> [System.Environment]::SetEnvironmentVariable("JUPYTER_PATH", "${ENV:USERPROFILE}\.julia\conda\3\share\jupyter", "User")

# set powershell script (.ps1) execution policy for anaconda activation profile
> Set-ExecutionPolicy -ExecutionPolicy RemoteSigned

# create profile for activating anaconda environment
> New-Item -ItemType Directory "$env:USERPROFILE\Documents\WindowsPowerShell"

> echo "(& `"$env:USERPROFILE\.julia`\conda\3\Scripts\conda.exe`" `"shell.powershell`" `"hook`") | Out-String | Invoke-Expression" | Out-File -Append -Encoding utf8 -FilePath "$env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1"

# => restart poweshell for reflecting environmental settings

# install IJulia.jl (Jupyter kernel for Julia; optional)
> julia -e 'using Pkg; Pkg.add("IJulia")'

# --- EvCxR Jupyter kernel (Rust REPL environment)
## @ref https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md

# start command prompt of msvc 2019 build tools
## * to use `cmake` command
> cmd /k "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# install evcxr_jupyter crate (`cmake` must be needed)
> cargo install evcxr_jupyter
> evcxr_jupyter --install

# exit command prompt
> exit

# confirm jupyter kernels
> jupyter kernelspec list
Available kernels:
  python3              C:\Users\{user}\.julia\conda\3\share\jupyter\kernels\python3
  rust                 C:\Users\{user}\.julia\conda\3\share\jupyter\kernels\rust
  julia-1.6            C:\Users\{user}\AppData\Roaming\jupyter\kernels\julia-1.6
```
