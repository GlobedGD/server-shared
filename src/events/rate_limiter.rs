use qunet::transport::RateLimiter;

#[derive(Clone, Debug)]
pub struct EventRateLimiterOptions {
    pub events_per_sec: u32,
    pub max_burst: u32,
}

pub struct EventRateLimiter {
    limiter: RateLimiter,
}

impl EventRateLimiter {
    pub fn new(opts: EventRateLimiterOptions) -> Self {
        Self {
            limiter: RateLimiter::new(opts.events_per_sec, opts.max_burst),
        }
    }

    pub fn tick(&mut self, targets: usize, data_size: usize, reliable: bool) -> bool {
        // compute amount of tokens to consume
        let mut quota = match targets {
            0..1 => 1,
            _ => targets as u32 / 2,
        };

        if reliable {
            quota *= 2;
        }

        quota *= data_size.max(1).div_ceil(256) as u32;

        self.limiter.consume_many(quota)
    }
}
