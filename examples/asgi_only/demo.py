import asyncio
from starlette.applications import Starlette
from starlette.requests import Request
from starlette.responses import JSONResponse, Response
from starlette.routing import Route
from asgi_only import create_server

async def simple(_request: Request):
    return Response(content="HELLO WORLD!")

async def json(_request: Request):
    return JSONResponse({"hello": "world!"})

async def echo(request: Request):
    agent = request.headers.get("User-Agent", "UNKNOWN")
    body = (await request.body()).decode("utf-8")
    return JSONResponse({"hello": agent, "body": body})

async def startup():
    print(">>>>> STARTUP CALLED!")

async def shutdown():
    print(">>>>> SHUTDOWN CALLED!")

app = Starlette(debug=True, routes=[
    Route("/simple", simple),
    Route("/json", json),
    Route("/echo", json),
], on_startup=[startup], on_shutdown=[shutdown])

async def main():
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

if __name__ == "__main__":
    asyncio.run(main())
