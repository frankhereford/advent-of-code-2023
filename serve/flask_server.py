from flask import Flask, send_from_directory, render_template
import os
import time
import random
import logging
import sys

app = Flask(__name__)
static_folder_path = "/application/workdir"
images_folder_path = "/application/serve/images"

# Set up logging to stdout
logging.basicConfig(stream=sys.stdout, level=logging.DEBUG)


def get_random_video_id():
    all_files = os.listdir(static_folder_path)
    directories = [f for f in all_files if os.path.isdir(os.path.join(static_folder_path, f))]
    pick = random.choice(directories)
    message = "Directories: " + str(directories) + " Picked: " + pick
    app.logger.info(message)
    return pick

@app.route('/')
def list_files():
    all_files = os.listdir(static_folder_path)
    directories = [f for f in all_files if os.path.isdir(os.path.join(static_folder_path, f))]
    return render_template("list_files.html", files=directories)

@app.route('/<path:filename>')
def serve_file(filename):
    full_path = os.path.join(static_folder_path, filename)
    
    if os.path.isdir(full_path):

        playlist_path = os.path.join(full_path, 'playlist.m3u8')
        while not os.path.exists(playlist_path):
            time.sleep(1)

        directories = os.listdir(full_path)

        another_video_id = get_random_video_id()
        return render_template("video_player.html", top_television=filename, bottom_television=another_video_id)
    else:
        return send_from_directory(static_folder_path, filename)

@app.route('/images/<img_name>')
def serve_image(img_name):
    return send_from_directory(images_folder_path, img_name)


if __name__ == '__main__':
    app.run(host="0.0.0.0", port=80, debug=True)
