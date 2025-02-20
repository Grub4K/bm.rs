pkgname="bm"
pkgver="0.1.1"
pkgrel="0"
pkgdesc="Bookmark paths and quickly change into bookmarked directories"
arch=("x86_64")
license=("MIT")
makedepends=(
    "cargo-nightly"
    "git"
)
source=("${pkgname}-${pkgver}::git+https://github.com/Grub4K/bm.rs#commit=06a945d5d23a7252717f2c8609cbfe97e15122e0")
sha256sums=('6a4fcc68f01be5537a4482af4da5ea9351e52a83acb471d268653ffccbcfa412')

prepare() {
    export RUSTUP_TOOLCHAIN="nightly"
    cd "${srcdir}/${pkgname}-${pkgver}/"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export RUSTUP_TOOLCHAIN="nightly"
    export CARGO_TARGET_DIR="target"
    cd "${srcdir}/${pkgname}-${pkgver}/"
    cargo build --release --locked
}

package() {
    install -Dm0755 -t "${pkgdir}/usr/bin/" "${srcdir}/${pkgname}-${pkgver}/target/release/${pkgname}"
}

