# roome-server
Add new functionality to the Roome XiaoYi smart speaker.

https://reimu.moe/2021/09/04/Roome-XY-2/

## Features
### DLNA Casting
Can receive audio stream from mobile devices and play them with a new FFmpeg.

Supported audio formats: AAC, MP3, Opus, PCM (WAV), FLAC.

Supported container formats: MPEG, Vorbis, AAC, MP3, DASH, HLS, FLV.

Supported protocols: HTTP.

### Remote Control API
Can send any control command to the device via `/command/:id`.
