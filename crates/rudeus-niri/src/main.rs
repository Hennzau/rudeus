use std::sync::{Arc, Mutex};

use eyre::{Context, OptionExt, bail};
use niri_ipc::{Event, Request, Response, socket::Socket};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkspaceData {
    pub id: u64,
    pub idx: u8,
    pub focused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NiriResponsiveData {
    pub reveal_workspaces: bool,
    pub workspaces: Vec<WorkspaceData>,
}

fn to_json_string(data: &NiriResponsiveData) -> eyre::Result<String> {
    serde_json::to_string(&data).wrap_err("error serializing data to JSON")
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tokio::task::spawn_blocking(|| -> eyre::Result<()> {
        let socket = Socket::connect().wrap_err("error connecting to the niri socket")?;

        let (reply, mut read_event) = socket
            .send(Request::EventStream)
            .wrap_err("error communicating with niri")?;

        let reply = reply.map_err(eyre::Report::msg)?;

        let Response::Handled = reply else {
            bail!("unexpected response: expected Handled, got {reply:?}");
        };

        let data = Arc::new(Mutex::new(NiriResponsiveData {
            reveal_workspaces: false,
            workspaces: Vec::new(),
        }));

        let last_reveal = Arc::new(Mutex::new(Instant::now()));

        loop {
            let event = read_event().wrap_err("error reading event from niri")?;

            match event {
                Event::WorkspacesChanged { workspaces } => {
                    let mut data = data.lock().ok().ok_or_eyre("failed to lock data")?;

                    data.workspaces = workspaces
                        .iter()
                        .map(|ws| WorkspaceData {
                            id: ws.id,
                            idx: ws.idx,
                            focused: ws.is_focused,
                        })
                        .collect();

                    data.workspaces.sort_by_key(|ws| ws.idx);

                    println!("{}", to_json_string(&data)?);
                }
                Event::WorkspaceActivated { id, focused: _ } => {
                    let mut data_mut = data.lock().ok().ok_or_eyre("failed to lock data")?;

                    if let Some(workspace) = data_mut.workspaces.iter_mut().find(|ws| ws.id == id) {
                        workspace.focused = true;
                    }

                    let previous_workspaces = data_mut
                        .workspaces
                        .iter_mut()
                        .filter(|ws| ws.id != id && ws.focused)
                        .collect::<Vec<_>>();

                    for workspace in previous_workspaces {
                        workspace.focused = false;
                    }

                    let mut last_reveal_mut = last_reveal.lock().unwrap();
                    *last_reveal_mut = Instant::now();

                    data_mut.reveal_workspaces = true;

                    println!("{}", to_json_string(&data_mut)?);

                    let data = data.clone();
                    let last_reveal = last_reveal.clone();
                    tokio::task::spawn_blocking(move || -> Result<(), eyre::Error> {
                        std::thread::sleep(std::time::Duration::from_millis(1000));

                        let last_reveal_read = last_reveal.lock().unwrap();

                        if last_reveal_read.elapsed().as_millis() < 800 {
                            return Ok(());
                        }

                        let mut data_mut = data.lock().ok().ok_or_eyre("failed to lock data")?;
                        data_mut.reveal_workspaces = false;

                        println!("{}", to_json_string(&data_mut)?);
                        Ok(())
                    });
                }
                _ => {}
            }
        }
    })
    .await??;

    Ok(())
}
