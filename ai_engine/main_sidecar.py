"""
SAVA Python AI Sidecar Service
Receives JSON IPC requests from Rust core via STDIN and dispatches to Binary Track AI engines.
"""

import sys
import json
import traceback
from encoder_ai import SAVAEncoderAI
from decoder_ai import SAVADecoderAI

def main():
    raw_input = sys.stdin.read().strip()
    if not raw_input:
        sys.exit(0)

    try:
        req = json.loads(raw_input)
        cmd = req.get("command")

        if cmd == "encode":
            encoder = SAVAEncoderAI()
            encoder.extract_binary_tracks(
                input_video_path=req["input_video"],
                temp_dir=req["output_config_path"], # Pass temp directory for 10 binary tracks
                sample_rate=req.get("sample_rate", 1)
            )
            response = {
                "status": "SUCCESS",
                "message": "AI Binary track extraction completed.",
                "payload": None
            }
        elif cmd == "decode":
            decoder = SAVADecoderAI()
            decoder.restore_video_from_binary_tracks(
                temp_dir=req["helper_config_path"], # Pass temp directory containing binary tracks
                lowres_video_path=req["lowres_video"],
                output_video_path=req["output_video"],
                target_resolution=tuple(req.get("target_resolution", [3840, 2160]))
            )
            response = {
                "status": "SUCCESS",
                "message": "AI Generative Restoration completed from binary tracks.",
                "payload": None
            }
        else:
            response = {
                "status": "ERROR",
                "message": f"Unknown command: {cmd}",
                "payload": None
            }

    except Exception as e:
        response = {
            "status": "ERROR",
            "message": str(e),
            "payload": {"traceback": traceback.format_exc()}
        }

    print(json.dumps(response))

if __name__ == "__main__":
    main()
