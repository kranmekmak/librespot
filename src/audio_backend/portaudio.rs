use super::{Open, Sink};
use std::io;
use portaudio;

pub struct PortAudioSink<'a>(Option<portaudio::stream::Stream<'a, i16, i16>>);

impl <'a> Open for PortAudioSink<'a> {
    fn open() -> PortAudioSink<'a> {
        portaudio::initialize().unwrap();
        PortAudioSink(None)
    }
}

impl <'a> Sink for PortAudioSink<'a> {
    fn start(&mut self) -> io::Result<()> {
        if self.0.is_some() {
        } else {
            self.0 = Some(portaudio::stream::Stream::open_default(
                0, 2, 44100.0,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                None
            ).unwrap());
        }

        self.0.as_mut().unwrap().start().unwrap();
        Ok(())
    }
    fn stop(&mut self) -> io::Result<()> {
        self.0.as_mut().unwrap().stop().unwrap();
        self.0 = None;
        Ok(())
    }
    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        match self.0.as_mut().unwrap().write(&data) {
            Ok(_) => (),
            Err(portaudio::PaError::OutputUnderflowed) => error!("PortAudio write underflow"),
            Err(e) => panic!("PA Error {}", e),
        };

        Ok(())
    }
}

impl <'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio::terminate().unwrap();
    }
}
