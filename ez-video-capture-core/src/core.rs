use std::collections::VecDeque;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread;

use crate::{
    capture::VideoCapture,
    decoder::{HardwareAcceleration, VideoDecoder},
    error::Error,
    packet::{Packet, clone_packet},
    writer::VideoWriter,
};

type PacketBuffer = VecDeque<Packet>;

pub struct VideoCaptureCore {
    buffer: Arc<Mutex<PacketBuffer>>,
    decoder: Mutex<VideoDecoder>,
    width: u32,
    height: u32,
    is_closed: Arc<AtomicBool>,
    daemon_threads: Vec<thread::JoinHandle<()>>,
}

impl VideoCaptureCore {
    pub fn new(
        path: &str,
        timeout: u32,
        hardware_acceleration: HardwareAcceleration,
        save_path: Option<String>,
    ) -> Result<Self, Error> {
        let (capture, decoder) = Self::connect(path, timeout, hardware_acceleration)?;
        let mut instance = VideoCaptureCore {
            buffer: Arc::new(Mutex::new(PacketBuffer::new())),
            width: decoder.width() as u32,
            height: decoder.height() as u32,
            decoder: Mutex::new(decoder),
            is_closed: Arc::new(AtomicBool::new(false)),
            daemon_threads: Vec::new(),
        };

        let writer_tx = save_path
            .map(|path| instance.setup_writer_thread(path, &capture))
            .transpose()?;
        instance.setup_capture_thread(capture, writer_tx);

        Ok(instance)
    }

    fn connect(
        path: &str,
        timeout: u32,
        hardware_acceleration: HardwareAcceleration,
    ) -> Result<(VideoCapture, VideoDecoder), Error> {
        let (capture, codec) = VideoCapture::new(path, timeout)?;
        let decoder = VideoDecoder::new(codec, capture.codecpar(), hardware_acceleration)?;
        Ok((capture, decoder))
    }

    fn setup_capture_thread(
        &mut self,
        mut capture: VideoCapture,
        writer: Option<mpsc::Sender<Packet>>,
    ) {
        let is_closed = self.is_closed.clone();
        let buffer = self.buffer.clone();
        let mut tasks: Vec<Box<dyn Fn(Packet) + Send>> = vec![];
        tasks.push(Box::new(move |packet| {
            let mut buffer = buffer.lock().unwrap();
            if packet.flags == 1 {
                buffer.clear();
            }
            buffer.push_back(packet);
        }));
        if let Some(writer) = writer {
            let is_closed = self.is_closed.clone();
            tasks.push(Box::new(move |packet| {
                if writer.send(packet).is_err() {
                    is_closed.store(true, Ordering::Relaxed);
                }
            }))
        }
        let handler = thread::spawn(move || {
            while let Ok(Some(packet)) = capture.receive() {
                if packet.flags == 1 {
                    for task in tasks.iter() {
                        task(clone_packet(&packet));
                    }
                    break;
                }
            }
            while !is_closed.load(Ordering::Relaxed) {
                let packet = match capture.receive() {
                    Ok(Some(packet)) => packet,
                    _ => break,
                };
                for task in tasks.iter() {
                    task(clone_packet(&packet));
                }
            }
            is_closed.store(true, Ordering::Relaxed);
        });
        self.daemon_threads.push(handler);
    }

    fn setup_writer_thread(
        &mut self,
        path: String,
        capture: &VideoCapture,
    ) -> Result<mpsc::Sender<Packet>, Error> {
        let mut writer = VideoWriter::new(&path, capture.codecpar().clone(), capture.time_base())?;
        let (tx, rx) = mpsc::channel();
        let handler = thread::spawn(move || {
            for packet in rx.iter() {
                if writer.push(packet).is_err() {
                    break;
                };
            }
            let _ = writer.end();
        });
        self.daemon_threads.push(handler);

        Ok(tx)
    }

    pub fn grab(&self) -> Result<Option<Vec<u8>>, Error> {
        if self.is_closed.load(Ordering::Relaxed) {
            return Err(Error::ConnectionClosed);
        }
        let mut decoder = self.decoder.lock().unwrap();
        let packets: Vec<Packet> = {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.drain(..).collect()
        };
        let frames = packets
            .into_iter()
            .flat_map(|packet| decoder.decode(&packet));
        Ok(frames.last())
    }

    pub fn close(&mut self) {
        self.is_closed.store(true, Ordering::Relaxed);
        for t in self.daemon_threads.drain(..) {
            t.join().expect("Couldn't join on the associated thread");
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
