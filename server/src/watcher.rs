use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{error, info};

pub fn start_watcher(
    watch_dir: &str,
    sender: broadcast::Sender<String>,
) -> notify::Result<RecommendedWatcher> {
    let watch_path = std::fs::canonicalize(watch_dir)
        .map(|p| {
            #[cfg(windows)]
            {
                let s = p.to_string_lossy();
                if let Some(stripped) = s.strip_prefix(r"\\?\") {
                    return std::path::PathBuf::from(stripped);
                }
            }
            p
        })
        .unwrap_or_else(|_| std::path::PathBuf::from(watch_dir));

    info!("Starting file watcher on: {:?}", watch_path);

    let (raw_tx, mut raw_rx) = tokio::sync::mpsc::channel::<()>(100);

    // Spawn a debouncer task in Tokio:
    // When files change, wait 300ms of quiet before sending a single "reload" event
    let debounced_sender = sender.clone();
    tokio::spawn(async move {
        let debounce_duration = Duration::from_millis(300);
        loop {
            // Wait for first event
            if raw_rx.recv().await.is_none() {
                break;
            }

            // Drain any subsequent events that arrive within the debounce window
            loop {
                tokio::select! {
                    msg = raw_rx.recv() => {
                        if msg.is_none() {
                            break;
                        }
                    }
                    _ = tokio::time::sleep(debounce_duration) => {
                        break;
                    }
                }
            }

            info!("Debounce window passed, broadcasting 'reload'");
            let _ = debounced_sender.send("reload".to_string());
        }
    });

    let raw_tx_arc = Arc::new(raw_tx);
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| match res {
            Ok(event) => match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    let tx = raw_tx_arc.clone();
                    let _ = tx.try_send(());
                }
                _ => {}
            },
            Err(e) => {
                error!("Watcher error: {:?}", e);
            }
        },
        Config::default(),
    )?;

    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

    Ok(watcher)
}
