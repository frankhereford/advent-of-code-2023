from flask import Flask, send_from_directory, render_template
import os
import time
import redis
import random
import logging
import sys

app = Flask(__name__)
static_folder_path = "/application/workdir"
images_folder_path = "/application/serve/images"

# Set up logging to stdout
logging.basicConfig(stream=sys.stdout, level=logging.DEBUG)


def append_to_redis_list(redis_host='redis', redis_port=6379, key_to_append='start_queue', value_to_append=''):
    # Initialize the Redis client
    r = redis.Redis(host=redis_host, port=redis_port)

    # Append the value to the list
    r.rpush(key_to_append, value_to_append)
    print(f"Appended '{value_to_append}' to the list stored at key {key_to_append}")


def get_random_video_id():
    all_files = os.listdir(static_folder_path)
    directories = [f for f in all_files if os.path.isdir(os.path.join(static_folder_path, f))]
    pick = random.choice(directories)
    message = "Directories: " + str(directories) + " Picked: " + pick
    app.logger.info(message)
    return pick

@app.route('/video_processor/')
def list_files():
    all_files = os.listdir(static_folder_path)
    directories = [f for f in all_files if os.path.isdir(os.path.join(static_folder_path, f))]
    return render_template("list_files.html", files=directories)

@app.route('/video_processor/<path:filename>')
def serve_file(filename):
    full_path = os.path.join(static_folder_path, filename)

    if not os.path.exists(full_path):
        # Your Redis operations here
        append_to_redis_list(value_to_append=filename)
        # log or return something to indicate Redis operation happened
        app.logger.info("Static folder does not exist. Performed Redis operation.")
        return "Static folder does not exist. Performed Redis operation.", 404

    #while not os.path.isdir(full_path):
        #time.sleep(1) 
    
    if os.path.isdir(full_path):

        playlist_path = os.path.join(full_path, 'playlist.m3u8')
        while not os.path.exists(playlist_path):
            time.sleep(1)

        directories = os.listdir(full_path)

        another_video_id = get_random_video_id()
        return render_template("video_player.html", top_television=filename, bottom_television=another_video_id)
    else:
        return send_from_directory(static_folder_path, filename)

@app.route('/video_processor/images/<img_name>')
def serve_image(img_name):
    return send_from_directory(images_folder_path, img_name)


if __name__ == '__main__':
    app.run(host="0.0.0.0", port=80, debug=True)
