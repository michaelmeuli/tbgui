(function() {
    var implementors = Object.fromEntries([["async_fs",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_fs/struct.File.html\" title=\"struct async_fs::File\">File</a>"]]],["async_io",[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_io/struct.Async.html\" title=\"struct async_io::Async\">Async</a>&lt;T&gt;"]]],["async_net",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/struct.TcpListener.html\" title=\"struct async_net::TcpListener\">TcpListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/struct.TcpStream.html\" title=\"struct async_net::TcpStream\">TcpStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/struct.UdpSocket.html\" title=\"struct async_net::UdpSocket\">UdpSocket</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/unix/struct.UnixDatagram.html\" title=\"struct async_net::unix::UnixDatagram\">UnixDatagram</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/unix/struct.UnixListener.html\" title=\"struct async_net::unix::UnixListener\">UnixListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_net/unix/struct.UnixStream.html\" title=\"struct async_net::unix::UnixStream\">UnixStream</a>"]]],["async_process",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_process/struct.ChildStderr.html\" title=\"struct async_process::ChildStderr\">ChildStderr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_process/struct.ChildStdin.html\" title=\"struct async_process::ChildStdin\">ChildStdin</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_process/struct.ChildStdout.html\" title=\"struct async_process::ChildStdout\">ChildStdout</a>"]]],["async_signal",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_signal/struct.Signals.html\" title=\"struct async_signal::Signals\">Signals</a>"]]],["async_std",[["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/fs/struct.File.html\" title=\"struct async_std::fs::File\">File</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/io/struct.Stderr.html\" title=\"struct async_std::io::Stderr\">Stderr</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/io/struct.Stdin.html\" title=\"struct async_std::io::Stdin\">Stdin</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/io/struct.Stdout.html\" title=\"struct async_std::io::Stdout\">Stdout</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.TcpListener.html\" title=\"struct async_std::net::TcpListener\">TcpListener</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.TcpStream.html\" title=\"struct async_std::net::TcpStream\">TcpStream</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.UdpSocket.html\" title=\"struct async_std::net::UdpSocket\">UdpSocket</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixDatagram.html\" title=\"struct async_std::os::unix::net::UnixDatagram\">UnixDatagram</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixListener.html\" title=\"struct async_std::os::unix::net::UnixListener\">UnixListener</a>"],["impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.AsRawFd.html\" title=\"trait async_std::os::unix::io::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixStream.html\" title=\"struct async_std::os::unix::net::UnixStream\">UnixStream</a>"]]],["calloop",[["impl&lt;'l, Data&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"calloop/struct.EventLoop.html\" title=\"struct calloop::EventLoop\">EventLoop</a>&lt;'l, Data&gt;"]]],["mio",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.TcpListener.html\" title=\"struct mio::net::TcpListener\">TcpListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UdpSocket.html\" title=\"struct mio::net::UdpSocket\">UdpSocket</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixDatagram.html\" title=\"struct mio::net::UnixDatagram\">UnixDatagram</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixListener.html\" title=\"struct mio::net::UnixListener\">UnixListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixStream.html\" title=\"struct mio::net::UnixStream\">UnixStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/struct.Poll.html\" title=\"struct mio::Poll\">Poll</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/struct.Registry.html\" title=\"struct mio::Registry\">Registry</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Receiver.html\" title=\"struct mio::unix::pipe::Receiver\">Receiver</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Sender.html\" title=\"struct mio::unix::pipe::Sender\">Sender</a>"]]],["polling",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"polling/struct.Poller.html\" title=\"struct polling::Poller\">Poller</a>"]]],["rustix",[]],["smithay_client_toolkit",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"smithay_client_toolkit/data_device_manager/struct.ReadPipe.html\" title=\"struct smithay_client_toolkit::data_device_manager::ReadPipe\">ReadPipe</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"smithay_client_toolkit/data_device_manager/struct.WritePipe.html\" title=\"struct smithay_client_toolkit::data_device_manager::WritePipe\">WritePipe</a>"]]],["socket2",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>"]]],["tokio",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/fs/struct.File.html\" title=\"struct tokio::fs::File\">File</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/io/struct.Stderr.html\" title=\"struct tokio::io::Stderr\">Stderr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/io/struct.Stdin.html\" title=\"struct tokio::io::Stdin\">Stdin</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/io/struct.Stdout.html\" title=\"struct tokio::io::Stdout\">Stdout</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.TcpListener.html\" title=\"struct tokio::net::TcpListener\">TcpListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.TcpSocket.html\" title=\"struct tokio::net::TcpSocket\">TcpSocket</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.TcpStream.html\" title=\"struct tokio::net::TcpStream\">TcpStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.UdpSocket.html\" title=\"struct tokio::net::UdpSocket\">UdpSocket</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.UnixDatagram.html\" title=\"struct tokio::net::UnixDatagram\">UnixDatagram</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.UnixListener.html\" title=\"struct tokio::net::UnixListener\">UnixListener</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.UnixSocket.html\" title=\"struct tokio::net::UnixSocket\">UnixSocket</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.UnixStream.html\" title=\"struct tokio::net::UnixStream\">UnixStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/unix/pipe/struct.Receiver.html\" title=\"struct tokio::net::unix::pipe::Receiver\">Receiver</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/net/unix/pipe/struct.Sender.html\" title=\"struct tokio::net::unix::pipe::Sender\">Sender</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/process/struct.ChildStderr.html\" title=\"struct tokio::process::ChildStderr\">ChildStderr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/process/struct.ChildStdin.html\" title=\"struct tokio::process::ChildStdin\">ChildStdin</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/process/struct.ChildStdout.html\" title=\"struct tokio::process::ChildStdout\">ChildStdout</a>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"tokio/io/unix/struct.AsyncFd.html\" title=\"struct tokio::io::unix::AsyncFd\">AsyncFd</a>&lt;T&gt;"]]],["winit",[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"winit/event_loop/struct.EventLoop.html\" title=\"struct winit::event_loop::EventLoop\">EventLoop</a>&lt;T&gt;"]]],["x11rb",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"x11rb/rust_connection/struct.DefaultStream.html\" title=\"struct x11rb::rust_connection::DefaultStream\">DefaultStream</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"x11rb/xcb_ffi/struct.XCBConnection.html\" title=\"struct x11rb::xcb_ffi::XCBConnection\">XCBConnection</a>"]]],["zvariant",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"enum\" href=\"zvariant/enum.Fd.html\" title=\"enum zvariant::Fd\">Fd</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"zvariant/struct.OwnedFd.html\" title=\"struct zvariant::OwnedFd\">OwnedFd</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[271,442,1720,881,293,2684,316,2725,275,14,708,275,5180,319,618,536]}