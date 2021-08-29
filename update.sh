#bin/bash
rm -rf temp
git clone --depth=1 https://github.com/bkaradzic/bgfx temp/bgfx
git clone --depth=1 https://github.com/bkaradzic/bx temp/bx
git clone --depth=1 https://github.com/bkaradzic/bimg temp/bimg
rm -rf bgfx
rm -rf bx
rm -rf bimg
cp -R temp/bgfx/. bgfx
cp -R temp/bx/. bx
cp -R temp/bimg/. bimg
rm -rf temp
