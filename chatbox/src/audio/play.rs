use bytes::Bytes;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::io::{Cursor, Read, Seek};

pub fn audio_local(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    play_audio(BufReader::new(file))
}

pub fn audio_memory(source: &Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(source.to_vec());
    play_audio(cursor)
}

fn play_audio<R>(source: R) -> Result<(), Box<dyn std::error::Error>>
where
    R: Read + Seek + Send + Sync + 'static,
{
    let source = Decoder::new(source)?;

    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    sink.append(source);
    sink.play();
    sink.sleep_until_end();
    Ok(())
}
