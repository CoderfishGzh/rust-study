use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 创建有4个线程的线程池子
    let pool = ThreadPool::new(4);

    // 读取信息流接受端
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 每读取到一个请求，就从线程池里选择一个空闲的线程执行 handle_connection 这个闭包函数
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
