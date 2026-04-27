from time import time

from ez_video_capture import EzVideoCapture, HardwareType


def run(source: str, count: int, duration: int) -> float:
    caps = [
        EzVideoCapture(
            source,
            timeout=10000,
            hardware_acceleration=HardwareType.VideoToolbox,
        )
        for _ in range(count)
    ]
    frames = 0
    start = time()

    while time() - start < duration:
        for cap in caps:
            if cap.grab() is not None:
                frames += 1

    for cap in caps:
        cap.close()

    return frames / count / duration
