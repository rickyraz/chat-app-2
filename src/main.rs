use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // Make a TCP echo server

    // tcp listner
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // exact method for tcp listener
    let (mut socket, _addr) = listener.accept().await.unwrap();

    //Split method on socket to handles i/o , read part and write part
    let (reader, mut writer) = socket.split();

    // read something from the client socket -- let mut buffer = [0u8; 1024];
    // read some new data into memory from network stream, we need a little buffer to store the data
    // BufReader is the same as buf reader in std lib, but more efficient and async
    let mut reader = BufReader::new(reader);
    // Strings to store each line
    let mut line = String::new();

    loop {
        // read data from socket
        // let bytes_read = socket.read(&mut buffer).await.unwrap();
        let bytes_read = reader.read_line(&mut line).await.unwrap();

        // if no bytes read, the client has closed the connection or disconnected
        if bytes_read == 0 {
            break;
        }

        // write data back to socket - write_all is not write a message to every sockets that's connected to TCP listener,
        // it's  write every single byte there's in input buffer out into the output stream(buffer)
        // very common pattern in networking = read from one socket, write to another socket

        // socket.write_all(&buffer[0..bytes_read]).await.unwrap();
        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}

// WHY get repeated message?
// because we are reading from the same socket, and we are not closing the connection -> line.clear()
// it turns out that wehn you call read_line, it just appends the next line that read until string buffer
