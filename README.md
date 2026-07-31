# SAVA (Semantic AI Video Archive)

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.10+-blue.svg)](https://www.python.org/)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)]()

**SAVA (Semantic AI Video Archive)**, klasik piksel tabanlı video sıkıştırma yöntemlerini (H.264, AV1) terk edip, **Koşullu Üretken Yapay Zeka (Conditioned Generative Video AI)** tabanlı semantik depolama sunan **Rust & Python AI** destekli devrimsel bir video arşivleme mimarisidir.

---

## 💡 Temel İdea

Bir videoyu baştan aşağı pikselleriyle saklamak yerine; videonun genel kompozisyonunu ve hareketini koruyan ultra düşük çözünürlüklü temel bir görsel iskelet (Low-Res 72p/144p) ile Yapay Zeka'nın halüsinasyon görmesini engelleyen matematiksel/vektörel rehber verileri (**Helper Config**) saklanır.

İzleme sırasında güçlü bir **Generative AI Decoder** (ControlNet + SVD / Wan 2.1), bu verileri birleştirerek orijinal 4K görüntüyü restore eder.

### Örnek Sıkıştırma Senaryosu:
- **Orijinal Video**: 10 GB (4K Çözünürlük, `kadir.mp4`)
- **SAVA Arşivi (`kadir.sava`)**: **~60 MB Toplam Boyut**
  - `~30 MB` -> LowRes Video (72p/144p temel kompozisyon iskeleti)
  - `~30 MB` -> Helper Config & Metadata (Yüz embedding'leri, OCR metinleri, nesne koordinatları, zaman damgalı promptlar)

---

## 🛠️ Mimari ve Dizin Yapısı

```text
sava/
├── Cargo.toml                  # Rust sistem yapılandırması
├── config.json                 # Merkezi yapılandırma (Sıfır hardcoded değişken)
├── requirements.txt            # Python AI kütüphane bağımlılıkları
├── README.md                   # Dokümantasyon
├── demo_sava.sh                # Entegrasyon test betiği
│
├── src/                        # RUST CORE HOST ENGINE
│   ├── main.rs                 # CLI Arayüzü (sava encode / sava decode)
│   ├── container/              # .sava Konteyner Paketleyici & Unpacker (Zip/Zstd)
│   ├── media/                  # FFmpeg Downscaler Modülü
│   ├── schema/                 # Serde Veri Şeması
│   ├── helper/                 # Log Rotasyonu, Config Hot-Reloading & JSON Response
│   └── bridge/                 # Rust <-> Python AI Stdio IPC Köprü Servisi
│
└── ai_engine/                  # PYTHON AI SIDECAR SERVİSİ
    ├── main_sidecar.py         # Rust Daemon Dinleyicisi (Stdio IPC)
    ├── encoder_ai.py           # YOLO11, InsightFace ArcFace 512-dim, EasyOCR Çıkarıcılar
    └── decoder_ai.py           # ControlNet + Generative AI Restoration Engine
```

---

## 🚀 Kurulum ve Çalıştırma

### 1. Sistem Bağımlılıkları
- **Rust Toolchain**: `cargo` & `rustc` (Edition 2021)
- **FFmpeg**: Video downscaling için (`ffmpeg` binary)
- **Python**: 3.10+ & PyTorch ekosistemi

### 2. Bağımlılıkları Yükleme
```bash
pip install -r requirements.txt
```

### 3. Derleme
```bash
cargo build --release
```

### 4. Kullanım

#### 📹 Videoyu `.sava` Arşivine Sıkıştırma (Encode)
```bash
./target/release/sava encode -i input_4k.mp4 -o archive.sava -w 256 -H 144
```

#### 🔄 `.sava` Arşivini 4K Kalitesine Restore Etme (Decode)
```bash
./target/release/sava decode -i archive.sava -o restored_4k.mp4
```

---

## 🧪 Entegrasyon Testini Çalıştırma

```bash
./demo_sava.sh
```

---

## 📄 Lisans

Bu proje **MIT** ve **Apache-2.0** dual lisansı ile korunmaktadır. Detaylar için [`LICENSE-MIT`](LICENSE-MIT) ve [`LICENSE-APACHE`](LICENSE-APACHE) dosyalarına göz atabilirsiniz.
