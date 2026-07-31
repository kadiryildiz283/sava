# SAVA Public Roadmap (2026-2027)

SAVA (Semantic AI Video Archive) projesinin gelecek sürümleri için hedeflenen geliştirme yol haritası.

---

## 🎯 v0.1.0 - Alfa Sürümü (Tamamlandı)
- [x] Rust tabanlı `.sava` konteyner paketi ve demuxer/muxer (`src/container/`)
- [x] FFmpeg tabanlı 72p/144p downscaler (`src/media/`)
- [x] Python AI sidecar köprüsü (YOLO, InsightFace, EasyOCR entegrasyonu)
- [x] ControlNet + Latent Diffusion simüle edilebilir restorasyon altyapısı

---

## 🚀 v0.5.0 - Beta Sürümü (Planlanan)
- [ ] **Cross-Frame Temporal Attention**: Videodaki zamansal titremeyi (flicker) sıfırlayan AnimateDiff / SVD zaman serisi entegrasyonu.
- [ ] **ONNX Runtime Rust Bindings**: Python bağımlılığını azaltmak için YOLO ve InsightFace çıkarımını doğrudan Rust `ort` kütüphanesine taşıma.
- [ ] **Adaptive Bitrate Allocation**: Hareketli sahnelerde semantik kare sıklığını dinamik ayarlama.
- [ ] **Zstandard Chunk Compression**: `.sava` içindeki binary track'lerin paralelleştirilmiş Zstd sıkıştırması.

---

## 🌟 v1.0.0 - Üretim Sürümü (Enterprise Ready)
- [ ] **TensorRT / FP8 Hızlandırması**: GPU decoder çıkarım hızını real-time (30 FPS+) seviyesine çıkarma.
- [ ] **C / WASM C-API Bindings**: SAVA Codec'inin C++ ve WebAssembly (tarayıcı içi izleme) kütüphane bağlamları.
- [ ] **Streaming SAVA Stream Protocol**: Parçalı (.sava chunk) canlı akış (Live Streaming) desteği.
