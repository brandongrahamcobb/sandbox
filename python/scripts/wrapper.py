from flask import Flask, request, Response, jsonify
import requests
import json
import datetime
app = Flask(__name__)
LLAMA_SERVER_COMPLETIONS = "http://127.0.0.1:8080/v1/completions"
LLAMA_SERVER_CHAT = "http://127.0.0.1:8080/v1/chat/completions"
LLAMA_SERVER_MODELS = "http://127.0.0.1:8080/v1/models"

def inject_stream_false(data):
    data["stream"] = False
    return data

def clean_response(resp_json):
    # Remove 'reasoning_content' from all choices
    for choice in resp_json.get("choices", []):
        if "message" in choice and "reasoning_content" in choice["message"]:
            choice["message"].pop("reasoning_content")
    return resp_json

def forward_request(url):
    data = request.get_json()
    if data is None:
        return "Invalid JSON", 400
    data = inject_stream_false(data)
    resp = requests.post(url, json=data)
    try:
        resp_json = resp.json()
        resp_json = clean_response(resp_json)
        return Response(json.dumps(resp_json), status=resp.status_code, content_type="application/json")
    except ValueError:
        # fallback if llama-server didn't return JSON
        return Response(resp.content, status=resp.status_code, content_type="application/json")

@app.route("/generate", methods=["POST"])
def proxy_generate():
    return forward_request(LLAMA_SERVER_COMPLETIONS)

@app.route("/api/chat", methods=["POST"])
def proxy_chat():
    return forward_request(LLAMA_SERVER_CHAT)

@app.route("/reset", methods=["POST"])
def proxy_reset():
    return {"status": "reset"}, 200

@app.route("/api/tags", methods=["GET"])
def proxy_tags():
    try:
        resp = requests.get(LLAMA_SERVER_MODELS)
        resp.raise_for_status()
        server_models = resp.json().get("models", [])
    except Exception as e:
        print("Error fetching models:", e)
        server_models = []

    tags = []
    for m in server_models:
        # Only include a tag if it's capable of chat/completion
        if "completion" in m.get("capabilities", []):
            tags.append({
                "id": m.get("model", m.get("name", "unknown")),
                "name": m.get("name", m.get("model", "unknown")),
                "model": m.get("model", m.get("name", "unknown")),
                "type": "chat",
                "modified_at": m.get("modified_at") or datetime.datetime.utcnow().isoformat() + "Z",
                "size": m.get("size", 0),
                "digest": m.get("digest", ""),
                "details": {
                    "format": m.get("details", {}).get("format", "gguf"),
                    "family": m.get("details", {}).get("family", ""),
                    "parameter_size": m.get("details", {}).get("parameter_size", ""),
                    "quantization_level": m.get("details", {}).get("quantization_level", "")
                }
            })

    return jsonify({"models": tags}), 200

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8090, debug=True, threaded=True)
