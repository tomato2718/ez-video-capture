use std::sync::Mutex;

use ez_video_capture_core::{Error, HardwareAcceleration, VideoCaptureCore};
use pyo3::exceptions::{PyConnectionError, PyIOError, PyValueError};
use pyo3::prelude::*;

#[pyclass]
struct EzVideoCapture {
    core: Mutex<VideoCaptureCore>,
    width: u32,
    height: u32,
}

impl EzVideoCapture {
    fn map_error(err: Error) -> PyErr {
        match err {
            Error::FailedToOpenSource(path) => {
                PyConnectionError::new_err(format!("Failed to open video source: {}", path))
            }
            Error::NoVideoStream => PyConnectionError::new_err("No video stream found in source"),
            Error::ReadError => PyIOError::new_err("Error reading from video source"),
            Error::UnsupportedPlatform => PyValueError::new_err(
                "Selected hardware acceleration is not supported on this platform",
            ),
            Error::FailedToOpenDecoder => PyValueError::new_err("Failed to open decoder"),
            Error::NoHwConfig => {
                PyValueError::new_err("No hardware decoder config found for codec")
            }
            Error::FailedToOpenWriter => PyIOError::new_err("Failed to open writer"),
            Error::FailedToWriteFrame => PyIOError::new_err("Failed to write frame"),
            Error::FailedToWriteTrailer => PyIOError::new_err("Failed to write trailer"),
            Error::ConnectionClosed => PyConnectionError::new_err("Connection is closed"),
        }
    }
}

#[pymethods]
impl EzVideoCapture {
    #[new]
    #[pyo3(signature = (path, /, *, timeout=10000, hardware_acceleration=None, save_path=None))]
    pub fn new(
        path: String,
        timeout: u32,
        hardware_acceleration: Option<HardwareType>,
        save_path: Option<String>,
    ) -> PyResult<Self> {
        let core = VideoCaptureCore::new(
            &path,
            timeout,
            hardware_acceleration
                .map(|hw| hw.into())
                .unwrap_or(HardwareAcceleration::None),
            save_path,
        )
        .map_err(Self::map_error)?;

        let width = core.width();
        let height = core.height();

        Ok(Self {
            core: Mutex::new(core),
            width,
            height,
        })
    }

    pub fn grab(&self) -> PyResult<Option<Vec<u8>>> {
        self.core.lock().unwrap().grab().map_err(Self::map_error)
    }

    pub fn close(&self) {
        self.core.lock().unwrap().close();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum HardwareType {
    VAAPI,
    VideoToolbox,
    D3D11VA,
    D3D12VA,
    CUDA,
    Vulkan,
}

impl From<HardwareType> for HardwareAcceleration {
    fn from(value: HardwareType) -> Self {
        match value {
            HardwareType::VAAPI => HardwareAcceleration::VAAPI,
            HardwareType::VideoToolbox => HardwareAcceleration::VideoToolbox,
            HardwareType::D3D11VA => HardwareAcceleration::D3D11VA,
            HardwareType::D3D12VA => HardwareAcceleration::D3D12VA,
            HardwareType::CUDA => HardwareAcceleration::CUDA,
            HardwareType::Vulkan => HardwareAcceleration::Vulkan,
        }
    }
}

#[pymodule]
fn ez_video_capture(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EzVideoCapture>()?;
    m.add_class::<HardwareType>()?;
    Ok(())
}
