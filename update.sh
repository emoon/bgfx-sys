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
rm -rf bgfx/.git
rm -rf bx/.git
rm -rf bimg/.git
rm -rf temp
# The SIZE_C(x) macros casuse issues for bindgen so we remove them
sed -i 's/UINT8_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT16_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT32_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT64_C//' bgfx/include/bgfx/defines.h
