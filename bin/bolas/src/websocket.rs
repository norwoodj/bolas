use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use foundations::telemetry::log;
use serde::Deserialize;

use crate::{
    bolas::{Bola, BolasArena},
    settings::BolasConfig,
};

pub(crate) async fn serve_websockets(
    req: HttpRequest,
    stream: web::Payload,
    config: web::Data<BolasConfig>,
) -> Result<HttpResponse, Error> {
    let actor = BolasWebsocketActor {
        bolas_state: BolasArena::new(
            config.bolas_refresh_rate_ms,
            config.velocity_scaling_factor,
            config.collision_detection_algorithm,
        ),
    };

    ws::start(actor, &req, stream)
}

struct BolasWebsocketActor {
    bolas_state: BolasArena,
}

#[derive(Deserialize)]
enum ClientMessage {
    SetCanvasDimensions { height: i32, width: i32 },
    NewBola(Bola),
}

impl BolasWebsocketActor {
    fn start_refresh_loop(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let arena_id = self.bolas_state.get_id();
        log::info!("Created new bolas arena"; "arena" => %arena_id);

        ctx.run_interval(self.bolas_state.get_refresh_rate(), move |act, ctx| {
            act.bolas_state.update_state();
            let message = match serde_json::to_string(&act.bolas_state) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Failed to serialize bolas state to send to client"; "arena" => %arena_id, "error" => %e);
                    ctx.stop();
                    return;
                }
            };

            ctx.text(message);
        });
    }
}

impl Actor for BolasWebsocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_refresh_loop(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BolasWebsocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        let client_message_text = match msg {
            ws::Message::Text(text) => text,
            ws::Message::Close(_) => {
                log::debug!("Client closed the connection, exiting actor"; "arena" => %self.bolas_state.get_id());
                ctx.stop();
                return;
            }
            _ => {
                log::error!("Websocket actor received unexpected message type"; "arena" => %self.bolas_state.get_id(), "message_type" => ?msg);
                ctx.stop();
                return;
            }
        };

        let Ok(client_message) = serde_json::from_slice(client_message_text.as_bytes()) else {
            log::error!(
                "Failed to parse message from client";
                "arena" => %self.bolas_state.get_id(),
                "message" => ?client_message_text,
            );
            ctx.stop();
            return;
        };

        match client_message {
            ClientMessage::SetCanvasDimensions { height, width } => {
                log::debug!(
                    "Updating canvas dimensions";
                    "arena" => %self.bolas_state.get_id(),
                    "height" => height,
                    "width" => width,
                );
                self.bolas_state.set_canvas_dimensions(height, width);
            }
            ClientMessage::NewBola(bola) => {
                log::debug!("Adding new bola"; "arena" => %self.bolas_state.get_id(), "bola" => ?bola);
                self.bolas_state.add_bola(bola);
            }
        }
    }
}
