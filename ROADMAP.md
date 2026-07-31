# 🗺️ SAVA Project Roadmap

This document outlines the strategic engineering roadmap and feature milestones for **SAVA (Semantic AI Video Archive)**.

---

## 🎯 Phase 1: Core Foundation & IPC Architecture (v1.0-alpha) — *COMPLETED*
- [x] High-performance Rust CLI application with `clap` parser.
- [x] Zero-latency Stdio JSON-RPC IPC bridge connecting Rust host engine with Python AI sidecar.
- [x] Dynamic, hot-reloadable `config.json` via Rust `arc-swap` crate.
- [x] Rotated logging system (10 MB size limit / 7-day retention) with Zstd compression.
- [x] Standardized `JsonResponse` output schema.

---

## 🚀 Phase 2: 10-Track Modular Binary Container (v2.0-beta) — *COMPLETED*
- [x] Binary track layout design eliminating raw text JSON bloat.
- [x] **Global Face Gallery Deduplication**: InsightFace 512-dim face embeddings stored once in a lookup dictionary (99.4% metadata size reduction).
- [x] **4-bit Temporal Residual Depth Maps**: DepthAnything V2 depth maps with 4-bit nibble packing and inter-frame residual coding (98% size reduction).
- [x] **Affine Global Camera Motion**: 12-byte camera transformation vectors for optical flow (`motion.bin`).
- [x] OPUS audio extraction (`audio.opus`) & 72p H.264/H.265 low-res composition video stream (`lowres_video.mp4`).
- [x] Unified ZIP/Zstd `.sava` binary container packing and unpacking engine.

---

## 🌐 Phase 3: Streaming & Real-Time Protocol (v3.0) — *IN PROGRESS*
- [ ] **SAVA WebRTC Streamer**: Real-time streaming protocol pushing 72p skeleton video + binary semantic tracks over WebRTC channels.
- [ ] **Adaptive Bitrate Track Selector**: Client-side dynamic switching of active binary tracks based on network bandwidth.
- [ ] **Multi-GPU Parallel Pipeline**: Tokio async worker pool distributing neural feature extraction across multiple GPUs.

---

## ⚡ Phase 4: Hardware Acceleration & Edge Inference (v4.0) — *PLANNED*
- [ ] **NVIDIA TensorRT Integration**: Quantized INT8/FP16 sidecar models for 10x faster feature extraction.
- [ ] **DirectML & Apple Metal Backend**: Native acceleration on Windows/DirectX and macOS Apple Silicon (M1/M2/M3/M4).
- [ ] **ONNX Runtime Sidecar**: Lightweight C++ sidecar daemon eliminating Python runtime overhead.

---

## 📱 Phase 5: Client Ecosystem & WebAssembly Decoder (v5.0) — *PLANNED*
- [ ] **WASM Browser Decoder**: Native WebAssembly + WebGPU in-browser SAVA player for zero-install 4K generative playback.
- [ ] **Mobile SDK (iOS & Android)**: Native Rust core SDK bindings for mobile video archiving applications.
- [ ] **FFmpeg Demuxer Plugin**: Native `libsava` plugin allowing VLC, MPV, and FFmpeg to play `.sava` files directly.

---

## 💬 Community & Feedback

Have ideas or feature requests for SAVA?
- Join the discussion on [GitHub Discussions](https://github.com/kadiryildiz283/sava/discussions).
- Submit bug reports or feature proposals on [GitHub Issues](https://github.com/kadiryildiz283/sava/issues).
