from flask import Flask, send_from_directory, render_template
import os
import time

app = Flask(__name__)
static_folder_path = "/application/workdir"

@app.route('/files/')
def list_files():
    all_files = os.listdir(static_folder_path)
    directories = [f for f in all_files if os.path.isdir(os.path.join(static_folder_path, f))]
    return render_template("list_files.html", files=directories)

@app.route('/files/<path:filename>')
def serve_file(filename):
    full_path = os.path.join(static_folder_path, filename)
    
    if os.path.isdir(full_path):

        playlist_path = os.path.join(full_path, 'playlist.m3u8')
        while not os.path.exists(playlist_path):
            time.sleep(1)

        directories = os.listdir(full_path)
        return render_template("video_player.html", key=filename)
    else:
        return send_from_directory(static_folder_path, filename)

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=80)
