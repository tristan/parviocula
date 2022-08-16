import asyncio

from starlette.applications import Starlette
from starlette.requests import Request
from starlette.responses import JSONResponse, Response
from starlette.status import HTTP_204_NO_CONTENT
from starlette.routing import Route
from mixed_routes import create_server

async def simple(request):
    return Response(content="HELLO WORLD!")

async def empty(request):
    return Response(status_code=HTTP_204_NO_CONTENT)

async def mixed(request: Request):
    data = await request.json()
    name = data["name"]
    return Response(content=f"Hello {name}, from python!")

async def headers(request: Request):
    return Response(content="check headers!", headers={"SOME":"DATA"})

async def json(request: Request):
    return JSONResponse({"hello": "world!"})

async def startup():
    print(">>>>> STARTUP CALLED!")

async def shutdown():
    print(">>>>> SHUTDOWN CALLED!")

async def main():
    app = Starlette(debug=True, routes=[
        Route("/simple", simple),
        Route("/empty", empty),
        Route("/json", json),
        Route("/post_or_get", mixed, methods=["POST"]),
        Route("/headers", headers),
    ], on_startup=[startup], on_shutdown=[shutdown])

    context = create_server(app, port=3000)

    # because context.start() returns a future and getting a task out of that
    # doesn't work so well.
    # NOTE: it is not required to do this if you don't care about shutting down
    # gracefully, you can simply `await context.start()`.
    async def start_server():
        await context.start()

    # start the server in a task, so we don't cancel the future on ctrl+c
    task = asyncio.create_task(start_server())
    try:
        # sleep "forever"
        while True:
            await asyncio.sleep(60)
    finally:
        # ctrl+c will most of the time fall back onto here but it's not perfect
        # trigger a graceful shutdown and wait for everything to close
        await context.shutdown()
        await task

asyncio.run(main())
