// AsyncRead and AsyncWrite

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

async fn read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0, 30];

    let n = f.read(&mut buffer[..]).await?;
    println!("Read {} bytes from file.", n);
    println!("  The bytes: {:?}", &buffer[..n]);

    Ok(())
}

async fn read2() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0, 30];
    let mut len = 0;

    // read ends not till the end
    loop {
        match f.read(&mut buffer[len..]).await.unwrap() {
            0 => {
                println!("0: len={}", len);
                break;
            }
            n => {
                len = len + n;
                println!("{}: len={}", n, len);
            }
        }
    }

    println!("Read {} bytes from file.", len);
    println!("  The bytes: {:?}", buffer);

    Ok(())
}

async fn read_to_end() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    let n = f.read_to_end(&mut buffer).await?;
    println!("Read {} bytes from file.", n);
    println!("  The bytes: {:?}", &buffer);

    Ok(())
}

async fn read_to_string(file: &str) -> io::Result<()> {
    let mut f = File::open(file).await?;
    let mut buffer = String::new();

    let n = f.read_to_string(&mut buffer).await?;
    println!("Read {} bytes from {}", n, file);
    println!("  The bytes: {:?}", &buffer);

    Ok(())
}

async fn write() -> io::Result<()> {
    let mut f = File::create("foo.txt").await?;

    // println!("Wrote the first {} bytes of foo.txt", n);
    // let n = f.write(b"some bytes").await?;

    f.write_all(b"some bytes").await?;

    Ok(())
}

async fn buf_write() -> io::Result<()> {
    use tokio::fs::OpenOptions;

    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .open("foo.txt")
        .await?;

    let mut writer = BufWriter::new(f);

    writer.write("buffered data1\n".as_bytes()).await?;
    writer.write("buffered data2\n".as_bytes()).await?;
    writer.write("buffered data3\n".as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

async fn buf_read() -> io::Result<()> {
    let f = File::open("foo.txt").await?;

    let mut reader = BufReader::new(f);
    let mut buff = String::new();

    let result = reader.read_line(&mut buff).await?;
    println!("Buf read_line:1 {:?}, [{:?}]", buff, result);

    let result = reader.read_line(&mut buff).await?;
    println!("Buf read_line:2 {:?}, [{:?}]", buff, result);

    let result = reader.read_line(&mut buff).await?;
    println!("Buf read_line:3 {:?}, [{:?}]", buff, result);

    let result = reader.read_line(&mut buff).await?;
    println!("Buf read_line:4 {:?}, [{:?}]", buff, result);

    Ok(())
}


async fn copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut f = File::create("bar.txt").await?;

    io::copy(&mut reader, &mut f).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    use std::path::Path;

    if Path::new("foo.txt").exists() {
        std::fs::remove_file("foo.txt")?;
    }
    if Path::new("bar.txt").exists() {
        std::fs::remove_file("bar.txt")?;
    }

    write().await?;
    read_to_string("foo.txt").await?;
    buf_write().await?;
    read_to_string("foo.txt").await?;
    buf_read().await?;
    copy().await?;
    read_to_string("bar.txt").await?;

    read().await?;
    read2().await?;
    read_to_end().await?;

    Ok(())
}
