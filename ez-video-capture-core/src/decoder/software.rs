use rsmpeg::{
    avcodec::{AVCodecContext, AVCodecParametersRef, AVCodecRef},
    avutil::AVFrame,
    ffi::{AV_PIX_FMT_RGB24, SWS_BILINEAR},
    swscale::SwsContext,
};

use crate::error::Error;
use crate::packet::Packet;

pub struct SoftwareDecoder {
    decoder: AVCodecContext,
    scaler: SwsContext,
    frame_buffer: AVFrame,
    output_buffer: Vec<u8>,
}

impl SoftwareDecoder {
    pub fn new(codec: AVCodecRef, codecpar: AVCodecParametersRef) -> Result<Self, Error> {
        let decoder = Self::create_decoder(codec, codecpar)?;
        let scaler = Self::create_scaler(&decoder)?;
        let frame_buffer = Self::create_frame_buffer(&decoder);
        let output_buffer = vec![0u8; (3 * decoder.width * decoder.height) as usize];

        Ok(SoftwareDecoder {
            decoder,
            scaler,
            frame_buffer,
            output_buffer,
        })
    }
    fn create_decoder(
        codec: AVCodecRef,
        codecpar: AVCodecParametersRef,
    ) -> Result<AVCodecContext, Error> {
        let mut decoder = AVCodecContext::new(&codec);
        decoder.apply_codecpar(&codecpar).unwrap();
        match decoder.open(None) {
            Ok(_) => Ok(decoder),
            Err(_) => Err(Error::FailedToOpenDecoder),
        }
    }

    fn create_scaler(decoder: &AVCodecContext) -> Result<SwsContext, Error> {
        match SwsContext::get_context(
            decoder.width,
            decoder.height,
            decoder.pix_fmt,
            decoder.width,
            decoder.height,
            AV_PIX_FMT_RGB24,
            SWS_BILINEAR,
            None,
            None,
            None,
        ) {
            Some(sws) => Ok(sws),
            None => Err(Error::FailedToOpenDecoder),
        }
    }

    fn create_frame_buffer(decoder: &AVCodecContext) -> AVFrame {
        let mut rgb_frame = AVFrame::new();
        rgb_frame.set_width(decoder.width);
        rgb_frame.set_height(decoder.height);
        rgb_frame.set_format(AV_PIX_FMT_RGB24);
        rgb_frame.get_buffer(1).unwrap();
        rgb_frame
    }

    pub fn width(&self) -> usize {
        self.decoder.width as usize
    }

    pub fn height(&self) -> usize {
        self.decoder.height as usize
    }

    pub fn decode(&mut self, packet: &Packet, mut receive_frame: impl FnMut(&[u8])) {
        if self.decoder.send_packet(Some(packet)).is_err() {
            return;
        }
        while let Ok(frame) = self.decoder.receive_frame() {
            self.scaler
                .scale_frame(&frame, 0, frame.height, &mut self.frame_buffer)
                .unwrap();

            self.frame_buffer
                .image_copy_to_buffer(&mut self.output_buffer, 1)
                .expect("Should be ok");

            receive_frame(&self.output_buffer);
        }
    }
}
