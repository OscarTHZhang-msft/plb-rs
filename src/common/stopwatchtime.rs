pub struct StopwatchTime {
    ticks: i64,
}

impl StopwatchTime {
    pub fn default() -> StopwatchTime {
        StopwatchTime { ticks: 0 }
    }

    pub fn new(ticks: i64) -> StopwatchTime {
        StopwatchTime { ticks: ticks }
    }
}