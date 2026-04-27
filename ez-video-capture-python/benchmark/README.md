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

### 1 connection (20s)

| Case | FPS | RSS delta (MiB) |
|---|---|---|
| cv2.VideoCapture (sw) | 27.20 | 354.38 |
| cv2.VideoCapture (hw) | 28.15 | 354.78 |
| EzVideoCapture (sw) | 31.50 | 181.75 |
| EzVideoCapture (hw) | 31.25 | 155.12 |

### 5 connections (20s)

| Case | FPS | RSS delta (MiB) |
|---|---|---|
| cv2.VideoCapture (sw) | 25.52 | 1469.55 |
| cv2.VideoCapture (hw) | 25.22 | 1455.55 |
| EzVideoCapture (sw) | 31.00 | 783.17 |
| EzVideoCapture (hw) | 29.27 | 475.58 |

### 10 connections (20s)

| Case | FPS | RSS delta (MiB) |
|---|---|---|
| cv2.VideoCapture (sw) | 12.51 | 1922.05 |
| cv2.VideoCapture (hw) | 14.66 | 2108.80 |
| EzVideoCapture (sw) | 22.57 | 1350.41 |
| EzVideoCapture (hw) | 18.70 | 899.61 |
