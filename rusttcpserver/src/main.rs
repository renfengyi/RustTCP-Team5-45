use std::{thread,time};
use std::io::{Error,Read,Write};
use std::net::TcpStream;
use std::net::TcpListener;

fn handle_client(mut stream: TcpStream)->Result<(),Error>{
    let mut buf=[0;512];
    for _ in 0..1000{
        //Read Stream
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read==0{
            return Ok(());
        }
        //Write Stream
        stream.write(&buf[..bytes_read])?;

        thread::sleep(time::Duration::from_secs(1 as u64));
    }
    Ok(())
}
fn main()->std::io::Result<()> {
    //Server Listener IP"127.0.0.1:8080"
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>>=Vec::new();
    //loop liatener
    for stream in listener.incoming(){
        let stream = stream.expect("failed!");
        let handle=thread::spawn(move || {
            handle_client(stream)
            .unwrap_or_else(|error| eprintln!("{:?}",error));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec{
        handle.join().unwrap();
    }
    Ok(())
}
