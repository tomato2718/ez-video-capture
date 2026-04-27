import multiprocessing as mp
import os

import psutil

from . import cv2_hw, cv2_sw, ez_hw, ez_sw

CASES = {
    "cv2.VideoCapture (sw)": cv2_sw.run,
    "cv2.VideoCapture (hw)": cv2_hw.run,
    "EzVideoCapture (sw)": ez_sw.run,
    "EzVideoCapture (hw)": ez_hw.run,
}


def subprocess_target(
    case_name: str,
    source: str,
    count: int,
    duration: int,
    queue: "mp.Queue[tuple[float, int]]",
) -> None:
    proc = psutil.Process(os.getpid())
    rss_before = proc.memory_info().rss
    fps = CASES[case_name](source, count, duration)
    rss_delta = proc.memory_info().rss - rss_before
    queue.put((fps, rss_delta))
