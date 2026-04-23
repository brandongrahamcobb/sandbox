import subprocess, time
from obsws_python import ReqClient
import os
from dotenv import load_dotenv


def get_frontmost():
    script = 'tell application "System Events" to get {name of first application process whose frontmost is true, name of front window of (first application process whose frontmost is true)}'
    try:
        out = (
            subprocess.check_output(["osascript", "-e", script])
            .decode()
            .strip()
            .split(",")
        )
        return out[0].strip(), out[1].strip()
    except:
        return None, None


def main():
    load_dotenv()
    host = "127.0.0.1"
    password = os.environ["OBS_PASSWORD"]
    port = 4455
    client = ReqClient(host=host, port=port, password=password, sleep=0)
    last = None
    while True:
        app, title = get_frontmost()
        scene = "Main"
        if app == "ChatGPT Atlas":
            scene = "Atlas"
        elif app == "Terminal":
            scene = "Code"
        if scene != last:
            client.set_current_program_scene(scene)
            last = scene
        time.sleep(5)


main()
