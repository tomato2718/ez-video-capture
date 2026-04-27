"""
Benchmark entrance.

Usage:
    python -m benchmark rtsp://localhost:8554/stream
    python -m benchmark rtsp://localhost:8554/stream --count 5 --time 15
"""

import argparse
import multiprocessing as mp
from dataclasses import dataclass

from ._worker import CASES, subprocess_target


@dataclass
class BenchResult:
    name: str
    fps: float
    rss_delta: int

    def __str__(self) -> str:
        return (
            f"{self.name:<24} "
            f"fps={self.fps:>8.2f}  "
            f"rss_delta={self.rss_delta / (1024 * 1024):>10.2f} MiB"
        )


def benchmark(name: str, source: str, count: int, duration: int) -> BenchResult:
    ctx = mp.get_context("spawn")
    queue: "mp.Queue[tuple[float, int]]" = ctx.Queue()
    p = ctx.Process(
        target=subprocess_target,
        args=(name, source, count, duration, queue),
    )
    p.start()
    p.join()
    fps, rss_delta = queue.get()
    return BenchResult(name=name, fps=fps, rss_delta=rss_delta)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="FPS & memory benchmark for video capture backends."
    )
    parser.add_argument("source", help="Video source URL (e.g. RTSP stream).")
    parser.add_argument(
        "--count", type=int, default=5, help="Number of connections (default: 5)."
    )
    parser.add_argument(
        "--time", type=int, default=15, help="Duration in seconds (default: 15)."
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    results: list[BenchResult] = []
    for name in CASES:
        print(f"[running] {name}", flush=True)
        result = benchmark(name, args.source, args.count, args.time)
        print(f"[done]    {result}", flush=True)
        results.append(result)

    print()
    print("=" * 72)
    print(f"Benchmark: count={args.count}, time={args.time}s")
    print("=" * 72)
    print(f"{'Case':<24} {'FPS':>8} {'RSS delta (MiB)':>16}")
    print("-" * 56)
    for r in results:
        print(f"{r.name:<24} {r.fps:>8.2f} {r.rss_delta / (1024 * 1024):>16.2f}")


if __name__ == "__main__":
    main()
