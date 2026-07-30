# Maintainer: Maxime Gauduin <alucryd@archlinux.org>
# Maintainer: Peter Jung <ptr1337@archlinux.org>
# Contributor: Mark Wagie <mark.wagie@proton.me>

pkgname=cosmic-osd
pkgver=1.5.0
pkgrel=1
epoch=1
pkgdesc='COSMIC On-Screen Display'
arch=(x86_64)
url=https://github.com/AdityaHebballe/cosmic-osd
license=(GPL-3.0-only)
groups=(cosmic)
depends=(
  libgcc
  glibc
  libinput
  libpipewire
  libpulse
  libxkbcommon
  sound-theme-freedesktop
  systemd-libs
  wayland
)
makedepends=(
  cargo
  clang
  git
  just
  lld
)
source=(
  git+https://github.com/AdityaHebballe/cosmic-osd.git#branch=master
)
b2sums=('SKIP')

prepare() {
  cd cosmic-osd
  cargo fetch --locked
}

build() (
  export RUSTFLAGS+=" -C link-arg=-fuse-ld=lld"
  cd cosmic-osd
  just build-release
)

package() {
  cd cosmic-osd
  just rootdir="${pkgdir}" install
}

# vim: ts=2 sw=2 et:
