use calc::{eval, parse};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    let mut writer: TcpStream = stream.try_clone()?;
    let buf: BufReader<TcpStream> = BufReader::new(stream);
    let _ = writer.set_nonblocking(true);
    for line in buf.lines() {
        let line = line?;
        if let Ok(result) = parse(&line) {
            if let Ok(evalresult) = eval(&result) {
                writer.write_all(format!("{} \n", evalresult).as_bytes())?;
            } else {
                writer.write_all(format!("Couldn't eval result {:?}", result).as_bytes())?;
            }
        } else {
            writer.write_all(format!("Couldn't parse line {}", line).as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let output = handle_client(stream?);
        println!("{:?}", output);
    }
    Ok(())
}
