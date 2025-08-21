#[derive(Clone)]
struct TitleChannel {
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl TitleChannel {
    fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}

#[derive(Clone)]
struct StopSignal {
    tx: watch::Sender<bool>,
    rx: watch::Receiver<bool>,
}

impl StopSignal {
    fn new() -> Self {
        let (tx, rx) = watch::channel(false);
        Self { tx, rx }
    }
}

#[derive(Clone)]
struct WorkerArgs {
    id: usize,
    title_channel: TitleChannel,
    stop_signal: StopSignal,
    parents: Arc<Mutex<HashMap<String, String>>>,
}

impl WorkerArgs {
    fn new(id: usize, parents: Arc<Mutex<HashMap<String, String>>>) -> Self {
        let title_channel = TitleChannel::new();
        let stop_signal = StopSignal::new();
        Self {
            id,
            title_channel,
            stop_signal,
            parents,
        }
    }
}
