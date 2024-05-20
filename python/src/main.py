from sanic import Sanic, response
import os
import sys
from usercode import handle

port = os.environ.get('GITCALL_PORT')
if port is None or port == "":
    sys.stderr.write("GITCALL_PORT env is required but not set\n")
    sys.exit(1)

app = Sanic("gitcall-py-app")

@app.route("/", methods=["POST"])
async def handler(request):
    req = request.json
    jsonrpc = req["jsonrpc"]
    id = req["id"]
    params = req["params"]
    try:
        result = handle(id, params)
        return response.json({"jsonrpc": jsonrpc, "id": id, "result": result})
    except Exception as err:
        return response.json({"jsonrpc": jsonrpc, "id": id, "error": {"code": 1, "message": str(err)}})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=int(port))