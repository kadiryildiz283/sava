#!/bin/bash
set -e

echo "=== SAVA Rust & AI Sidecar Integration Test ==="

# 1. Generate test input 4K video if not present
python3 -c '
import cv2, numpy as np
w, h, fps = 3840, 2160, 30
out = cv2.VideoWriter("test_input.mp4", cv2.VideoWriter_fourcc(*"mp4v"), fps, (w, h))
for i in range(30):
    img = np.zeros((h, w, 3), dtype=np.uint8)
    img[:, :] = [100, 50, (i*8)%255]
    cv2.rectangle(img, (1000 + i*10, 500), (1800, 1500), (0, 255, 0), -1)
    cv2.putText(img, "SAVA RUST TEST", (1100, 700), cv2.FONT_HERSHEY_SIMPLEX, 3.0, (255, 255, 255), 6)
    out.write(img)
out.release()
print("Synthetic 4K video created: test_input.mp4")
'

# 2. Run Rust SAVA Encoder
echo -e "\n--- Step 1: Rust SAVA Encode ---"
./target/debug/sava encode -i test_input.mp4 -o test_output.sava

# Check size
ls -lh test_input.mp4 test_output.sava

# 3. Run Rust SAVA Decoder
echo -e "\n--- Step 2: Rust SAVA Decode ---"
./target/debug/sava decode -i test_output.sava -o restored_output.mp4

# Check final restored video
ls -lh restored_output.mp4

echo -e "\n=== SAVA Rust & Python AI Sidecar Pipeline Test PASSED SUCCESSFULLY! ==="
