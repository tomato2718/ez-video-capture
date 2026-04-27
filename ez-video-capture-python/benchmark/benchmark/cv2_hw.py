import threading
from time import time
from typing import Optional

import cv2


class Cv2HwCapture:
    def __init__(self, source: str) -> None:
        self._capture = cv2.VideoCapture(
            source + "?rtsp_transport=tcp",
            cv2.CAP_FFMPEG,
            [
                cv2.CAP_PROP_HW_ACCELERATION,
                cv2.VIDEO_ACCELERATION_ANY,
                cv2.CAP_PROP_OPEN_TIMEOUT_MSEC,
                10000,
                cv2.CAP_PROP_READ_TIMEOUT_MSEC,
                10000,
            ],
        )
        self._lock = threading.Lock()
        self._stopped = False
        self._frame = None
        self._thread = threading.Thread(target=self._loop, daemon=True)
        self._thread.start()

    def _loop(self) -> None:
        while not self._stopped:
            ret, frame = self._capture.read()
            if not ret:
                break
            with self._lock:
                self._frame = frame

    def grab(self) -> Optional[object]:
        with self._lock:
            frame = self._frame
            self._frame = None
        return frame

    def release(self) -> None:
        self._stopped = True
        self._thread.join(timeout=1.0)
        self._capture.release()


def run(source: str, count: int, duration: int) -> float:
    caps = [Cv2HwCapture(source) for _ in range(count)]
    frames = 0
    start = time()

    while time() - start < duration:
        for cap in caps:
            if cap.grab() is not None:
                frames += 1

    for cap in caps:
        cap.release()

    return frames / count / duration
