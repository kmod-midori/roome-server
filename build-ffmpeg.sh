#!/bin/sh
cd ./ffmpeg

T_PREFIX=/home/harryc/devtools/gcc-linaro-6.5.0-2018.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-

./configure \
    --prefix=/usr \
    --enable-static \
    --enable-gpl \
    --enable-pic \
    --enable-nonfree \
    --disable-doc --disable-programs --disable-everything --disable-swscale --disable-postproc --disable-debug \
    --enable-decoder=aac,aac_fixed,aac_latm,vorbis,mp3,mp3adu,mp3adufloat,mp3float,mp3on4,mp3on4float,opus,pcm_alaw,pcm_f16le \
    --enable-decoder=pcm_f24le,pcm_f32be,pcm_f32le,pcm_f64be,pcm_f64le,pcm_mulaw,pcm_s16be,pcm_s16be_planar,pcm_s16le,pcm_s16le_planar \
    --enable-decoder=pcm_s24be,pcm_s24daud,pcm_s24le,pcm_s24le_planar,pcm_s32be,pcm_s32le,pcm_s32le_plana,pcm_s64be,pcm_s64le,pcm_s8 \
    --enable-decoder=pcm_s8_planar,pcm_u16be,pcm_u16le,pcm_u24be,pcm_u24le,pcm_u32be,pcm_u32le,pcm_u8,flac \
    --enable-parser=aac,aac_latm,flac,mpegaudio,opus,vorbis \
    --enable-demuxer=aac,ogg,mp3,wav,mpegts,hls,dash,flv \
    --enable-protocol=http,httpproxy,file,cache,async \
    --enable-bsf=aac_adtstoasc \
    --enable-cross-compile \
    --cross-prefix=/home/harryc/devtools/gcc-linaro-6.5.0-2018.12-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf- \
    --extra-cflags="-march=armv7-a -mtune=cortex-a7 -mfpu=neon-vfpv4 -mfloat-abi=hard" \
    --enable-neon --arch=arm --cpu=armv7-a --target-os=linux

make -j16

${T_PREFIX}ar -M << EOF
CREATE libffmpeg.a
ADDLIB libswresample/libswresample.a
ADDLIB libavcodec/libavcodec.a
ADDLIB libavformat/libavformat.a
ADDLIB libavutil/libavutil.a
ADDLIB libavfilter/libavfilter.a
ADDLIB libavdevice/libavdevice.a
SAVE
END
EOF
${T_PREFIX}gcc -shared -o libffmpeg.so -lc -lm -lpthread -Wl,--whole-archive -Wl,--no-undefined libffmpeg.a -Wl,--no-whole-archive
${T_PREFIX}strip libffmpeg.so
