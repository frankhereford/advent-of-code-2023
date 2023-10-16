#!/usr/bin/env python3

# https://oioiiooixiii.blogspot.com

import subprocess
import sys
import os
import re

def validate_string(s):
    if len(s) != 11:
        return False
    pattern = re.compile("^[a-zA-Z0-9-_]+$")
    return bool(pattern.match(s))

def run_ffmpeg(video_id):
    if not validate_string(video_id):
        print("Invalid video ID")
        return
    try:
        subtitles = None
        if os.path.exists(f'{video_id}.en.srt'):
            subtitles=f"subtitles={video_id}.en.srt:force_style='FontName=Arial,PrimaryColour=&H00ffffff,OutlineColour=&H00000000,BackColour=&H00000000,BorderStyle=3,Outline=1,Shadow=0,MarginV=35'"

        # Reduce input to 25% PAL resolution
        shrink144="scale=-2:144"

        # Crop to 4:3 aspect ratio at 25% PAL resolution
        crop43="crop=180:144"

        # Create RGB chromatic aberration
        rgbFX="""split=3[red][green][blue];
            [red] lutrgb=g=0:b=0,
                    scale=188x144,
                    crop=180:144 [red];
            [green] lutrgb=r=0:b=0,
                    scale=184x144,
                    crop=180:144 [green];
            [blue] lutrgb=r=0:g=0,
                    scale=180x144,
                    crop=180:144 [blue];
            [red][blue] blend=all_mode='addition' [rb];
            [rb][green] blend=all_mode='addition',
                        format=gbrp"""

        # Create YUV chromatic aberration
        yuvFX="""split=3[y][u][v];
            [y] lutyuv=u=0:v=0,
                scale=192x144,
                crop=180:144 [y];
            [u] lutyuv=v=0:y=0,
                scale=188x144,
                crop=180:144 [u];
            [v] lutyuv=u=0:y=0,
                scale=180x144,
                crop=180:144 [v];
            [y][v] blend=all_mode='lighten' [yv];
            [yv][u] blend=all_mode='lighten'"""

        # Create edge contour effect
        edgeFX="edgedetect=mode=colormix:high=0"

        # Add noise to each frame of input
        noiseFX="noise=c0s=7:allf=t"

        # Add interlaced fields effect to input
        interlaceFX="""split[a][b];
                    [a] curves=darker [a];
                    [a][b] blend=all_expr='if(eq(0,mod(Y,2)),A,B)':shortest=1"""

        # 0 is good, 3 is good, 4 is strong, 7 is sharp 
        interlaceFX="""tinterlace=mode=7"""

        # Re-scale input to full PAL resolution
        scale2PAL="scale=720:576"

        # Re-scale input to full PAL resolution with linear pixel
        scale2PALpix="scale=720:576:flags=neighbor"

        # Add magnetic damage effect to input [crt screen]
        screenGauss="""[base];
                    nullsrc=size=720x576,
                        drawtext=
                        fontfile=/usr/share/fonts/truetype/freefont/FreeSerif.ttf:
                        text='@':
                        x=600:
                        y=30:
                        fontsize=170:
                        fontcolor=red@1.0,
                    boxblur=80 [gauss];
                    [gauss][base] blend=all_mode=screen:shortest=1"""

        # Add reflections to input [crt screen]
        reflections="""[base];
                    nullsrc=size=720x576,
                    format=gbrp,
                    drawtext=
                    fontfile=/usr/share/fonts/truetype/freefont/FreeSerif.ttf:
                    text='€':
                    x=50:
                    y=50:
                    fontsize=150:
                    fontcolor=white,
                    drawtext=
                    fontfile=/usr/share/fonts/truetype/freefont/FreeSerif.ttf:
                    text='J':
                    x=600:
                    y=460:
                    fontsize=120:
                    fontcolor=white,
                    boxblur=25 [lights];
                    [lights][base] blend=all_mode=screen:shortest=1"""

        # Add more detailed highlight to input [crt screen]
        highlight="""[base];
                    nullsrc=size=720x576,
                    format=gbrp,
                    drawtext=
                    fontfile=/usr/share/fonts/truetype/freefont/FreeSerif.ttf:
                    text='¡':
                    x=80:
                    y=60:
                    fontsize=90:
                    fontcolor=white,
                    boxblur=7 [lights];
                    [lights][base] blend=all_mode=screen:shortest=1"""

        # Curve input to mimic curve of crt screen
        curveImage="""vignette,
                    format=gbrp,
                    lenscorrection=k1=0.1:k2=0.2"""

        # Add bloom effect to input [crt screen]
        bloomEffect="""split [a][b];
                    [b] boxblur=26,
                            format=gbrp [b];
                    [b][a] blend=all_mode=screen:shortest=1"""

        skipFrames="select=mod(n\,2)"


        subprocess.run(f'rm -frv {video_id}', shell=True)
        subprocess.run(f'mkdir {video_id}', shell=True)

        filters = ''
        if subtitles:
            filters = f'''{subtitles}, {shrink144}, {crop43}, {rgbFX}, {yuvFX}, {noiseFX}, {interlaceFX}, {scale2PAL}
                {screenGauss} {reflections}
                {highlight}, {curveImage}, {bloomEffect}'''
        else:
            filters = f'''{shrink144}, {crop43}, {rgbFX}, {yuvFX}, {noiseFX}, {interlaceFX}, {scale2PAL}
                {screenGauss} {reflections}
                {highlight}, {curveImage}, {bloomEffect}'''

        command = [
            '/usr/local/bin/ffmpeg', '-y',
            #'-hwaccel', 'qsv',
            '-i', f'{video_id}.mp4',
            '-vframes', '5000',
            '-an', # mute
            '-c:v', 'libx264', # encode in h264, required by HLS
            #'-c:v', 'h264_qsv', # hardware accelerated encoding, still h264
            '-vf', filters, # apply filtergraphs
            '-bsf:v', 'h264_mp4toannexb', '-map', '0', '-f', 'segment', '-segment_time', '5',
            '-segment_list', f'{video_id}/playlist.m3u8', '-segment_format', 'mpegts', f'{video_id}/stream%03d.ts'
        ]
        subprocess.run(command)

    except Exception as e:
        print(f"Something went wrong: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        run_ffmpeg(sys.argv[1])
    else:
        print("Please provide a YouTube video ID as an argument.")
