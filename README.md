# rust-tuto

## Environment

- OS: Ubuntu 20.04
- Shell: bash
- Rust: 1.52.1
    - EvCxR Jupyter: 0.10.0 (JupyterLab REPL環境で使いたい場合用)
- Julia: 1.6.1 (Anaconda + JupyterLab 環境用; REPL環境不要であれば必要ない)
    - Conda.jl
        - JupyterLab

### Setup
```bash
# install rustup (rust version manager)
$ wget -qO- https://sh.rustup.rs | sh

# installation options:> 1 ## Proceed with installation
## => install to ~/.cargo/

# setup PATH to ~/.cargo/bin/
$ echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
$ . ~/.bashrc

# confirm rust compiler version
$ rustc --version
rustc 1.52.1 (9bc8c42bb 2021-05-09)
```

### Setup JupyterLab REPL environment
```bash
# download julia-1.6.1 into ~/julua-1.6.1/
$ wget -qO- https://julialang-s3.julialang.org/bin/linux/x64/1.6/julia-1.6.1-linux-x86_64.tar.gz | tar -xzv -C ~/

# install julia binary
$ sudo ln -s ~/julia-1.6.1/julia /usr/local/bin/julia

# install Conda.jl (Anaconda python for Julia)
$ julia -e 'using Pkg; Pkg.add("Conda")'

# install JupyterLab by Conda.jl
# $ conda install -y -c conda-forge jupyterlab
$ julia -e 'using Conda; Conda.add("jupyterlab"; channel="conda-forge")'

# setup PATH to ~/.julia/conda/3/bin
$ echo 'export PATH="$PATH:$HOME/.julia/conda/3/bin"' >> ~/.bashrc
$ . ~/.bashrc

# install IJulia.jl (Jupyter kernel for Julia; optional)
$ julia -e 'using Pkg; Pkg.add("IJulia")'

# --- EvCxR Jupyter kernel (Rust REPL environment)
## @ref https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md

# add rust-src component
$ rustup component add rust-src

# install cmake, gcc ...etc
$ sudo apt install -y cmake build-essential

# install evcxr_repl & evcxr_jupyter crate
$ cargo install evcxr_repl
$ cargo install evcxr_jupyter
$ evcxr_jupyter --install

# confirm jupyter kernels
$ jupyter kernelspec list
Available kernels:
  julia-1.6    ~/.local/share/jupyter/kernels/julia-1.6
  rust         ~/.local/share/jupyter/kernels/rust
  python3      ~/.julia/conda/3/share/jupyter/kernels/python3
```

### Launch JupyterLab
```bash
# launch jupyter lab
## * port: 8888
## * token: empty
## * project path: ./
$ jupyter lab --port=8888 --ServerApp.token='' --project=.

## => http://localhost:8888/lab
```
