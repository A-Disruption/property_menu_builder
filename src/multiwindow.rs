use futures::{stream::BoxStream, Stream, StreamExt};
use iced::{advanced::graphics::futures::subscription, Point, Size, Subscription};
use serde::{Deserialize, Serialize};
use std::io;
pub use iced::window::{Id,Settings};

pub const MIN_SIZE: Size = Size::new(1150.0, 750.0);

pub fn default_size() -> Size {
    Size {
        width: 1150.0,
        height: 750.0,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone,)]
pub struct Window {
    pub id: Id,
    pub title: String,
//    pub position: Option<Point>,
    pub size: Size,
    pub focused: bool,
}

impl Window {
    pub fn new(id: Id, title: String) -> Self {
        Self {
            id,
            title: title,
//            position: None,
            size: Size::default(),
            focused: false,
        }
    }

    pub fn opened(&mut self, position: Option<Point>, size: Size) {
//        self.position = position;
        self.size = size;
        self.focused = true;
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Moved(Point),
    Resized(Size),
    Focused,
    Unfocused,
    Opened { position: Option<Point>, size: Size },
    CloseRequested,
}

#[cfg(target_os = "windows")]
pub fn settings() -> Settings {
    use iced::window;
    use image::EncodableLayout;

    let img = image::load_from_memory_with_format(
        include_bytes!("../icons/MenuBuilder.ico"),
        image::ImageFormat::Png,
    );
    match img {
        Ok(img) => match img.as_rgba8() {
            Some(icon) => Settings {
                icon: window::icon::from_rgba(
                    icon.as_bytes().to_vec(),
                    icon.width(),
                    icon.height(),
                )
                .ok(),
                ..Default::default()
            },
            None => Default::default(),
        },
        Err(_) => Settings {
            ..Default::default()
        },
    }
}

pub fn events() -> Subscription<(Id, Operation)> {
    subscription::from_recipe(Events)
}

enum State<T: Stream<Item = (Id, Operation)>> {
    Idle {
        stream: T,
    },
    Moving {
        stream: T,
        id: Id,
        position: Point,
    },
    Resizing {
        stream: T,
        id: Id,
        size: Size,
    },
    MovingAndResizing {
        stream: T,
        id: Id,
        position: Point,
        size: Size,
    },
}

struct Events;

impl subscription::Recipe for Events {
    type Output = (Id, Operation);

    fn hash(&self, state: &mut subscription::Hasher) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        events: subscription::EventStream,
    ) -> BoxStream<'static, Self::Output> {
        use futures::stream;
        const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(500);

        let window_events = events.filter_map(|event| {
            futures::future::ready(match event {
                subscription::Event::Interaction {
                    window: id,
                    event: iced::Event::Window(window_event),
                    status: _,
                } => match window_event {
                    iced::window::Event::Moved(point) => Some((id, Operation::Moved(point))),
                    iced::window::Event::Resized(Size { width, height }) => {
                        Some((id, Operation::Resized(Size::new(width, height))))
                    }
                    iced::window::Event::Focused => Some((id, Operation::Focused)),
                    iced::window::Event::Unfocused => Some((id, Operation::Unfocused)),
                    iced::window::Event::Opened { position, size } => {
                        Some((id, Operation::Opened { position, size }))
                    }
                    iced::window::Event::CloseRequested => Some((id, Operation::CloseRequested)),
                    _ => None,
                },
                _ => None,
            })
        });

        stream::unfold(
            State::Idle {
                stream: window_events,
            },
            move |state| async move {
                match state {
                    State::Idle { mut stream } => {
                        stream.next().await.map(|(id, event)| match event {
                            Operation::Moved(position) => (
                                vec![],
                                State::Moving {
                                    stream,
                                    id,
                                    position,
                                },
                            ),
                            Operation::Resized(size) => (vec![], State::Resizing { stream, id, size }),
                            Operation::Focused => (vec![(id, Operation::Focused)], State::Idle { stream }),
                            Operation::Unfocused => {
                                (vec![(id, Operation::Unfocused)], State::Idle { stream })
                            }
                            Operation::Opened { position, size } => (
                                vec![(id, Operation::Opened { position, size })],
                                State::Idle { stream },
                            ),
                            Operation::CloseRequested => {
                                (vec![(id, Operation::CloseRequested)], State::Idle { stream })
                            }
                        })
                    }
                    State::Moving {
                        mut stream,
                        id,
                        position,
                    } => {
                        let next_event = tokio::time::timeout(TIMEOUT, stream.next()).await;

                        match next_event {
                            Ok(Some((next_id, Operation::Moved(position)))) if next_id == id => Some((
                                vec![],
                                State::Moving {
                                    stream,
                                    id,
                                    position,
                                },
                            )),
                            Ok(Some((next_id, Operation::Resized(size)))) if next_id == id => Some((
                                vec![],
                                State::MovingAndResizing {
                                    stream,
                                    id,
                                    position,
                                    size,
                                },
                            )),
                            _ => Some((vec![(id, Operation::Moved(position))], State::Idle { stream })),
                        }
                    }
                    State::Resizing {
                        mut stream,
                        id,
                        size,
                    } => {
                        let next_event = tokio::time::timeout(TIMEOUT, stream.next()).await;

                        match next_event {
                            Ok(Some((next_id, Operation::Resized(size)))) if next_id == id => {
                                Some((vec![], State::Resizing { stream, id, size }))
                            }
                            Ok(Some((next_id, Operation::Moved(position)))) if next_id == id => Some((
                                vec![],
                                State::MovingAndResizing {
                                    stream,
                                    id,
                                    position,
                                    size,
                                },
                            )),
                            _ => Some((vec![(id, Operation::Resized(size))], State::Idle { stream })),
                        }
                    }
                    State::MovingAndResizing {
                        mut stream,
                        id,
                        position,
                        size,
                    } => {
                        let next_event = tokio::time::timeout(TIMEOUT, stream.next()).await;

                        match next_event {
                            Ok(Some((next_id, Operation::Moved(position)))) if next_id == id => Some((
                                vec![],
                                State::MovingAndResizing {
                                    stream,
                                    id,
                                    position,
                                    size,
                                },
                            )),
                            Ok(Some((next_id, Operation::Resized(size)))) if next_id == id => Some((
                                vec![],
                                State::MovingAndResizing {
                                    stream,
                                    id,
                                    position,
                                    size,
                                },
                            )),
                            _ => Some((
                                vec![(id, Operation::Moved(position)), (id, Operation::Resized(size))],
                                State::Idle { stream },
                            )),
                        }
                    }
                }
            },
        )
        .map(stream::iter)
        .flatten()
        .boxed()
    }
}

#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Position> for iced::Point {
    fn from(position: Position) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

impl From<Position> for iced::window::Position {
    fn from(position: Position) -> Self {
        Self::Specific(position.into())
    }
}