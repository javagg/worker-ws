use worker::*;
use futures::stream::StreamExt;

#[event(fetch)]
async fn main(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/",  root)
        .get_async("/ws", ws)
        .run(request, env)
        .await
}

pub async fn root(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::ok("Hello Worker")
}

pub async fn ws(request: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    match request.headers().get("Upgrade")? {
        Some(value) => {
            if value.contains("websocket") {
                let pair = WebSocketPair::new()?;
                let server = pair.server;
                let client = pair.client;
                server.accept()?;
                let mut event_stream = server.events()?;
                // while let Some(event) = event_stream.next().await {
                //     let event = event?;
                //     if let WebsocketEvent::Message(msg) = event {
                //         if let Some(mut buf) = msg.bytes() {
                //             unimplemented!();
                //             // return Response::ok(buf);
                //         }
                //     }
                // }
                Response::from_websocket(client)
            } else {
                Response::error("Expected Upgrade: websocket", 426)
            }
        }
        None => Response::error("Expected Upgrade: websocket", 426),
    }
}