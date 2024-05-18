from sanic import Sanic, response
import os
import sys
from usercode import handle

uri = os.environ.get('USERCODE_PROXY_ADDR')
if uri is None or uri == "":
    sys.stderr.write("USERCODE_PROXY_ADDR env is required but not set\n")
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
    hostPort = uri.split(":")
    app.run(host=hostPort[0], port=int(hostPort[1]))