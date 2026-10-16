class Tuicli < Formula
  desc "Lightweight terminal desktop environment for Linux TTY"
  homepage "https://github.com/qalmurri/tuicli"
  url "https://github.com/qalmurri/tuicli/archive/refs/tags/v0.1.3.tar.gz"
  sha256 "20fadb77f6f05cd7542f3ee54d685a950781f5208d1368334e841e8810ac4428"
  license "MIT"

  depends_on :linux
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--path", ".", "--root", prefix
  end

  test do
    assert_predicate bin/"tuicli", :executable?
  end
end
