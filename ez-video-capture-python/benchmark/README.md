# Benchmark

FPS & memory benchmark for video capture backends.

## Usage

```bash
python -m benchmark <source> [--count N] [--time N]
```

## Environment

| Item | Value |
|---|---|
| OS | macOS (Apple Silicon) |
| CPU | Apple M2 Pro |
| RAM | 16 GB |
| GPU | Apple M2 Pro (integrated) |
| Python | 3.12.12 |
| OpenCV | opencv-python-headless 4.13.0.92 |
| ez-video-capture | 0.2.0 |
| FFmpeg | 8.0.1 |

## Test Source

Original video (480p) re-streamed to localhost via RTSP, rescaled to 4K 30fps.

| Item | Value |
|---|---|
| Original resolution | 480x360 |
| Original codec | H.264 (Main), yuv420p |
| Original bitrate | 153 kb/s |
| Stream resolution | 3840x2160 (4K) |
| Stream FPS | 30 |

## Results

> Note: numbers may vary between runs depending on system load and network conditions.

### 1 connection

| Case | FPS | RSS delta (MiB) | CPU (%) |
|---|---|---|---|
| cv2.VideoCapture (sw) | 29.80 | 400.70 | 161.0 |
| cv2.VideoCapture (hw) | 32.10 | 337.36 | 163.4 |
| EzVideoCapture (sw) | 30.35 | 220.45 | 136.9 |
| EzVideoCapture (hw) | 29.40 | 181.27 | 143.6 |

### 5 connections

| Case | FPS | RSS delta (MiB) | CPU (%) |
|---|---|---|---|
| cv2.VideoCapture (sw) | 26.90 | 1505.83 | 353.9 |
| cv2.VideoCapture (hw) | 28.02 | 1501.06 | 384.1 |
| EzVideoCapture (sw) | 29.94 | 811.67 | 292.5 |
| EzVideoCapture (hw) | 29.61 | 503.39 | 319.8 |

### 10 connections

| Case | FPS | RSS delta (MiB) | CPU (%) |
|---|---|---|---|
| cv2.VideoCapture (sw) | 14.39 | 1962.48 | 442.9 |
| cv2.VideoCapture (hw) | 15.14 | 2034.39 | 466.2 |
| EzVideoCapture (sw) | 17.52 | 1350.22 | 388.1 |
| EzVideoCapture (hw) | 17.21 | 904.27 | 306.0 |
