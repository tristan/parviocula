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

    try:
        task = context.start()
        # shield the task so we can avoid cancelling it on ctrl+c
        await asyncio.shield(task)
    finally:
        # trigger a graceful shutdown and wait for everything to close
        # (ctrl+c will most of the time be captured in this finally but it's not perfect)
        await context.shutdown()
        await task

asyncio.run(main())
