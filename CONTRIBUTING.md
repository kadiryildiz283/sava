# Katkı Rehberi (Contributing Guidelines)

SAVA (Semantic AI Video Archive) projesine katkıda bulunmak istediğiniz için teşekkürler!

## 🚀 Katkı Sağlama Adımları

1. **Repoyu Fork'layın**: Kendi GitHub hesabınıza repoyu klonlayın.
2. **Branch Oluşturun**: `git checkout -b feature/harika-ozellik` veya `fix/hata-duzeltme`.
3. **Geliştirme Yapın**:
   - Rust kodları için `cargo fmt` ve `cargo clippy` komutlarını çalıştırın.
   - Python sidecar kodları için PEP8 standartlarına uyun.
4. **Testleri Çalıştırın**:
   - `cargo test`
   - `./demo_sava.sh`
5. **Pull Request (PR) Açın**: PR başlığı ve içeriğinde yapılan değişiklikleri net açıklayın.

## 🎨 Kod Standardı

- **Rust**: `rustfmt` formatlama ve `clippy` uyarısız derleme şarttır.
- **Python**: Python 3.10+ uyumluluğu ve tip ipuçları (`type hints`) kullanımı teşvik edilir.

## 📬 İletişim & Sorular

Hatalar ve özellik istekleri için lütfen [GitHub Issues](https://github.com/your-org/sava/issues) kullanın. Genel tartışmalar ve sorular için [GitHub Discussions](https://github.com/your-org/sava/discussions) kanalını tercih edin.
