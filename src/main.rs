use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

fn _example_turbofish() {
    fn give_me_default<T>() -> T
    where
        T: Default,
    {
        T::default()
    }
    // turbofish syntax - ::<u32>
    let value = give_me_default::<u32>();
}

#[tokio::main]
async fn main() {
    // Make A TCP ECHO SERVER

    // multiple consumer or producer, send message or receive message
    // special type of channel that can have multiple sender and multiple receiver on a single channel
    let (tx, _rx) = broadcast::channel(10);

    // tcp listner
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // why loop?
    // because we want to keep accepting new connections
    loop {
        // exact method for tcp listener
        let (mut socket, addr) = listener.accept().await.unwrap();

        // clone the sender
        let tx = tx.clone();
        // clone the receiver
        let mut rx = tx.subscribe();

        // spawn a new task for each client = ASYNC BLOCK
        tokio::spawn(async move {
            //Split method on socket to handles i/o , read part and write part
            let (reader, mut writer) = socket.split();

            // read something from the client socket -- let mut buffer = [0u8; 1024];
            // read some new data into memory from network stream, we need a little buffer to store the data
            // BufReader is the same as buf reader in std lib, but more efficient and async
            let mut reader = BufReader::new(reader);
            // Strings to store each line
            let mut line = String::new();

            loop {
                //allow to runs multiple async tasks concurrently at the same time- automatically yield execution back to the runtime
                // using 'await' keyword to wait for the result of the future and then continue the execution of the code
                tokio::select! {
                    // read data from socket
                    result = reader.read_line(&mut line) => {

                        // if no bytes read, the client has closed the connection or disconnected
                        if result.unwrap() == 0 {
                            break;
                        }

                        // send message to all receiver
                        tx.send((line.clone(), addr)).unwrap();
                        // to not read the same line again
                        line.clear();
                    }

                    // receive message from all sender
                    result = rx.recv() => {
                        let( msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            // write data back to socket - write_all is not write a message to every sockets that's connected to TCP listener,
                            // it's  write every single byte there's in input buffer out into the output stream(buffer)
                            // very common pattern in networking = read from one socket, write to another socket
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }


                    }
                }
            }
        });
    }
}

// WHY get repeated message?
// because we are reading from the same socket, and we are not closing the connection -> line.clear()
// it turns out that wehn you call read_line, it just appends the next line that read until string buffer

// HOW to handle multiple client independently?
// we need to spawn a new task for each client
// standaratd way is to put excepted code in a function and call it in a loop

// IT'S not a chat server yet, because we are not handle multple client in one server
// we need to use a channel to send message from one client to another
// communicate a line of text from one client to another = using broadcast channel

// read data from socket (SEBELUM DIPECAH MENJADI MULTIPLE ASYNC TASK)
// let bytes_read = socket.read(&mut buffer).await.unwrap();
// let bytes_read = reader.read_line(&mut line).await.unwrap();

// write data to socket (SEBELUM DIPECAH MENJADI MULTIPLE ASYNC TASK)
// socket.write_all(&buffer[0..bytes_read]).await.unwrap();
// writer.write_all(line.as_bytes()).await.unwrap();

// HOW To solve multiple sending?
// by using address of socket

//What is turbofish?
// it's a way to specify the type of a generic function or method

// HOW DO I know when to use tokio::spawn?
// when you want to run a task in the background

// HOW DO I NOW when use tokio::select!?
// when you have multiple futures that you want to run concurrently and you want to wait for the first one to complete
