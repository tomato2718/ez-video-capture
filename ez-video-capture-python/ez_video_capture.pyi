__all__ = ["EzVideoCapture", "HardwareType"]

from enum import Enum
from typing import Optional

class EzVideoCapture:
    """A video capture client that reads frames from video streams.

    :param path: Video source path (e.g. RTSP URL or file path).
    :param timeout: Connection timeout in milliseconds.
    :param hardware_acceleration: Hardware decoder to use, or None for software decoding.
    :param save_path: If provided, saves the stream to this file path.
    :raises ConnectionError: Failed to open the video source, or no video stream found.
    :raises IOError: Failure while reading from the source.
    :raises ValueError: Failed to open the decoder, no matching hardware config for the codec,
        or selected HardwareType not supported on this platform.
    """

    def __init__(
        self,
        path: str,
        /,
        *,
        timeout: int = 10000,
        hardware_acceleration: Optional["HardwareType"] = None,
        save_path: Optional[str] = None,
    ) -> None: ...
    def grab(self) -> Optional[bytes]:
        """Grab the latest frame as raw RGB24 bytes (width * height * 3).

        :returns: Raw RGB24 pixel data, or None if no frame is available yet.
        :raises ConnectionError: The connection has been closed.
        """
        ...
    def close(self) -> None:
        """Close the connection and stop all background threads."""
        ...
    def width(self) -> int:
        """Return the video width in pixels."""
        ...
    def height(self) -> int:
        """Return the video height in pixels."""
        ...

class HardwareType(Enum):
    """Hardware acceleration type for video decoding.

    :cvar VAAPI: Video Acceleration API. Linux (Intel/AMD).
    :cvar VideoToolbox: Apple VideoToolbox. macOS.
    :cvar D3D11VA: Direct3D 11 Video Acceleration. Windows.
    :cvar D3D12VA: Direct3D 12 Video Acceleration. Windows.
    :cvar CUDA: NVIDIA CUDA. Linux/Windows.
    :cvar Vulkan: Vulkan Video. Linux/Windows.
    """

    VAAPI = 0
    VideoToolbox = 1
    D3D11VA = 2
    D3D12VA = 3
    CUDA = 4
    Vulkan = 5
