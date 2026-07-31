"""
SAVA Python AI Sidecar Service
Receives JSON IPC requests from Rust core via STDIN and dispatches to Encoder/Decoder AI engines.
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
            encoder.extract_features(
                input_video_path=req["input_video"],
                output_config_path=req["output_config_path"],
                sample_rate=req.get("sample_rate", 1)
            )
            response = {
                "status": "SUCCESS",
                "message": "AI Encoding feature extraction completed.",
                "payload": None
            }
        elif cmd == "decode":
            decoder = SAVADecoderAI()
            decoder.restore_video(
                lowres_video_path=req["lowres_video"],
                helper_config_path=req["helper_config_path"],
                output_video_path=req["output_video"],
                target_resolution=tuple(req.get("target_resolution", [3840, 2160]))
            )
            response = {
                "status": "SUCCESS",
                "message": "AI Generative Restoration completed.",
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
