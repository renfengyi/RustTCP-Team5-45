use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()>{
    //TCP连接IP
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        //按行读取输入的信息
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
        //将信息写入流，发送到服务器
        stream
        .write(input.as_bytes())
        .expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer:Vec<u8> = Vec::new();
        //从流里读取
        reader
        .read_until(b'\n',&mut buffer)
        .expect("Could not read into buffer");
        //匹配读取的值，有值/无值
        match str::from_utf8(&buffer){
            Ok(buf) => {
                println!("Server Return To Me -> {}",buf);
            },
            Err(err) => {
                println!("Could not write buffer as string");
            }
        }

        // println!("Server Return Me -> {}",
        //     str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}
