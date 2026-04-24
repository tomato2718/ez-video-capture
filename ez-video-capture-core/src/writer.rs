use crate::error::Error;
use crate::packet::Packet;
use rsmpeg::{
    avcodec::AVCodecParameters,
    avformat::AVFormatContextOutput,
    avutil::{AVDictionary, AVRational},
};
use std::ffi::CString;

pub struct VideoWriter {
    writer: AVFormatContextOutput,
    input_time_base: AVRational,
    output_time_base: AVRational,
    last_dts: i64,
}

impl VideoWriter {
    pub fn new(
        path: &str,
        codecpar: AVCodecParameters,
        input_time_base: AVRational,
    ) -> Result<Self, Error> {
        let cpath = CString::new(path).map_err(|_| Error::FailedToOpenWriter)?;
        let writer = Self::create_output(cpath)?;
        let writer = Self::setup_stream(writer, codecpar)?;
        let output_time_base = writer.streams()[0].time_base;
        Ok(Self {
            writer,
            input_time_base,
            output_time_base,
            last_dts: 0,
        })
    }

    fn create_output(path: CString) -> Result<AVFormatContextOutput, Error> {
        AVFormatContextOutput::builder()
            .format_name(c"mp4")
            .filename(&path)
            .build()
            .map_err(|_| Error::FailedToOpenWriter)
    }

    fn setup_stream(
        mut writer: AVFormatContextOutput,
        codecpar: AVCodecParameters,
    ) -> Result<AVFormatContextOutput, Error> {
        {
            let mut stream = writer.new_stream();
            stream.set_codecpar(codecpar.clone());
        }
        match writer.write_header(&mut Some(AVDictionary::new(
            c"movflags",
            c"frag_keyframe+empty_moov+default_base_moof",
            0,
        ))) {
            Ok(_) => Ok(writer),
            Err(_) => Err(Error::FailedToOpenWriter),
        }
    }

    pub fn push(&mut self, mut packet: Packet) -> Result<(), Error> {
        packet.set_stream_index(0);
        packet.rescale_ts(self.input_time_base, self.output_time_base);

        if packet.dts <= self.last_dts {
            packet.set_dts(self.last_dts + 1);
        }
        self.last_dts = packet.dts;

        if packet.pts < packet.dts {
            packet.set_pts(packet.dts);
        }

        self.writer
            .write_frame(&mut packet)
            .map_err(|_| Error::FailedToWriteFrame)
    }

    pub fn end(&mut self) -> Result<(), Error> {
        self.writer
            .write_trailer()
            .map_err(|_| Error::FailedToWriteTrailer)
    }
}
