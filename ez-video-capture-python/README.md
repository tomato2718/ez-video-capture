# ez-video-capture

A high-level Python API for capturing frames from video streams, written in Rust on top of FFmpeg via [`rsmpeg`](https://github.com/larksuite/rsmpeg) and exposed to Python through [PyO3](https://pyo3.rs/).

## Requirements

- Python ≥ 3.8
- FFmpeg development libraries and `pkg-config`

## Example

Capture one frame per second from an RTSP stream and save each as a PNG:

```python
from time import sleep

from PIL import Image
from ez_video_capture import EzVideoCapture, HardwareType

PATH = "rtsp://192.168.123.123:1234/"

capture = EzVideoCapture(PATH, hardware_acceleration=HardwareType.VAAPI)
width, height = capture.width(), capture.height()

for i in range(20):
    frame = capture.grab()
    if frame is not None:
        Image.frombytes(mode="RGB", size=(width, height), data=frame).save(
            f"frame_{i:03d}.png", format="PNG"
        )
    sleep(1)

capture.close()
```

Record an RTSP stream to an MP4 file while grabbing frames.

> Note that recording only remuxes the raw packets, so `hardware_acceleration` has no effect on the saved file:



```python
from time import sleep

from ez_video_capture import EzVideoCapture, HardwareType

PATH = "rtsp://192.168.123.123:1234/"

capture = EzVideoCapture(
    PATH,
    hardware_acceleration=HardwareType.VAAPI,
    save_path="output.mp4",
)

for _ in range(100):
    capture.grab()
    sleep(0.1)

capture.close()
```
