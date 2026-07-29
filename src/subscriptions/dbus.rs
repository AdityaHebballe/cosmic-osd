// TODO: Handle loss of connection, name?

use cosmic::iced;
use cosmic::iced::futures::SinkExt;
use cosmic::iced::stream;

static NAME: &str = "com.system76.CosmicOsd";
const OBJECT_PATH: &str = "/com/system76/CosmicOsd";

#[derive(Clone, Debug)]
pub enum Event {
    Connection(zbus::Connection),
    SystemConnection(zbus::Connection),
    ExternalBrightness(f64),
    Error(&'static str, zbus::Error),
}

#[derive(Clone)]
struct ExternalBrightnessService {
    output: iced::futures::channel::mpsc::Sender<Event>,
}

#[zbus::interface(name = "com.system76.CosmicOsd.ExternalBrightness")]
impl ExternalBrightnessService {
    async fn show(&self, brightness: f64) {
        let mut output = self.output.clone();
        let _ = output
            .send(Event::ExternalBrightness(brightness.clamp(0.0, 1.0)))
            .await;
    }
}

pub fn subscription() -> iced::Subscription<Event> {
    iced::Subscription::run_with("dbus-service", |_| {
        stream::channel(
            2,
            |mut output: iced::futures::channel::mpsc::Sender<Event>| async move {
                match connection(output.clone()).await {
                    Ok(connection) => {
                        if output.send(Event::Connection(connection)).await.is_err() {
                            return;
                        }
                    }
                    Err(err) => {
                        let _ = output
                            .send(Event::Error("create session connection", err))
                            .await;
                        return;
                    }
                }

                match system_connection().await {
                    Ok(connection) => {
                        if output
                            .send(Event::SystemConnection(connection))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Err(err) => {
                        let _ = output
                            .send(Event::Error("create system connection", err))
                            .await;
                    }
                }

                iced::futures::future::pending::<()>().await;
            },
        )
    })
}

async fn connection(
    output: iced::futures::channel::mpsc::Sender<Event>,
) -> zbus::Result<zbus::Connection> {
    zbus::connection::Builder::session()?
        .name(NAME)?
        .serve_at(OBJECT_PATH, ExternalBrightnessService { output })?
        .build()
        .await
}

async fn system_connection() -> zbus::Result<zbus::Connection> {
    zbus::connection::Builder::system()?.build().await
}
