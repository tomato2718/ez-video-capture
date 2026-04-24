use crate::error::Error;
use crate::packet::Packet;
use rsmpeg::{
    avcodec::{AVCodecParametersRef, AVCodecRef},
    avformat::AVFormatContextInput,
    avutil::{AVDictionary, AVRational},
    ffi::AVMEDIA_TYPE_VIDEO,
};
use std::ffi::CString;

pub struct VideoCapture {
    input: AVFormatContextInput,
    video_index: i32,
}

impl VideoCapture {
    pub fn new(path: &str, timeout: u32) -> Result<(Self, AVCodecRef<'_>), Error> {
        let open_err = || Error::FailedToOpenSource(path.to_string());
        let cpath = CString::new(path).map_err(|_| open_err())?;
        let input = Self::create_input(cpath, timeout).map_err(|_| open_err())?;
        let (video_index, codec) = match input.find_best_stream(AVMEDIA_TYPE_VIDEO) {
            Ok(Some((index, codec))) => Ok((index as i32, codec)),
            _ => Err(Error::NoVideoStream),
        }?;
        let capture = VideoCapture { input, video_index };
        if capture.codecpar().width <= 0 {
            Err(open_err())
        } else {
            Ok((capture, codec))
        }
    }

    fn create_input(cpath: CString, timeout: u32) -> Result<AVFormatContextInput, ()> {
        let timeout_us = CString::new((timeout as u64 * 1000).to_string()).map_err(|_| ())?;
        AVFormatContextInput::builder()
            .url(cpath.as_ref())
            .options(&mut Some(
                AVDictionary::new(c"rtsp_transport", c"tcp", 0).set(c"timeout", &timeout_us, 0),
            ))
            .open()
            .map_err(|_| ())
    }

    pub fn receive(&mut self) -> Result<Option<Packet>, Error> {
        loop {
            let packet = match self.input.read_packet() {
                Ok(Some(p)) => p,
                Ok(None) => return Ok(None),
                Err(_) => return Err(Error::ReadError),
            };
            if packet.stream_index != self.video_index {
                continue;
            }
            return Ok(Some(packet));
        }
    }

    pub fn codecpar(&self) -> AVCodecParametersRef<'_> {
        self.input.streams()[self.video_index as usize].codecpar()
    }

    pub fn time_base(&self) -> AVRational {
        self.input.streams()[self.video_index as usize].time_base
    }
}
