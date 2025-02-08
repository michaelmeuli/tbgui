# tbgui

A simple GUI front-end for [TB-Profiler](https://github.com/jodyphelan/TBProfiler) using SSH written in [Rust](https://www.rust-lang.org/) 

For now only works with [AuthMethod::with_key_file(key_path, None)](https://docs.rs/async-ssh2-tokio/latest/async_ssh2_tokio/client/enum.AuthMethod.html#method.with_key_file).<br>
This private key file has to be in [directories_next::UserDirs::home_dir](https://docs.rs/directories-next/latest/directories_next/struct.UserDirs.html#method.home_dir): e.g.: C:\Users\Alice\\.ssh\id_rsa<br>
The scripts executed on the remote cluster can be found here: [tbgui-server-scripts](https://github.com/michaelmeuli/tbgui-server-scripts).<br><br>
Uses environment variables as defaults if set:
- TBGUI_USERNAME
- REMOTE_RAW_DIR
- TB_PROFILER_SCRIPT
- REMOTE_RESULTS_DIR
- DEFAULT_TEMPLATE_REMOTE
- USER_TEMPLATE_REMOTE <br>

## cargo doc

[https://michaelmeuli.github.io/tbgui/tbgui/](https://michaelmeuli.github.io/tbgui/tbgui/index.html)<br><br>


### Built with:
- [iced](https://iced.rs/): A cross-platform GUI library for Rust focused on simplicity and type-safety.
- [async_ssh2_tokio](https://docs.rs/async-ssh2-tokio/latest/async_ssh2_tokio/): This library is an asynchronous and easy-to-use high level ssh client library for rust with the tokio runtime.
<br>Powered by the rust ssh implementation [russh](https://docs.rs/russh/latest/russh/).

### Starting point:
[Iced Todos Showcase](https://github.com/iced-rs/iced/tree/master/examples/todos)