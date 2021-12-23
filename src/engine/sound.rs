use crate::wasm4;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Channel {
    Pulse1 = wasm4::TONE_PULSE1,
    Pulse2 = wasm4::TONE_PULSE2,
    Triangle = wasm4::TONE_TRIANGLE,
    Noise = wasm4::TONE_NOISE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Mode {
    Mode1 = wasm4::TONE_MODE1,
    Mode2 = wasm4::TONE_MODE2,
    Mode3 = wasm4::TONE_MODE3,
    Mode4 = wasm4::TONE_MODE4,
}

pub struct Adsr {
    pub attack: u32,
    pub decay: u32,
    pub sustain: u32,
    pub release: u32,
}

impl Adsr {
    pub fn new(attack: u32, decay: u32, sustain: u32, release: u32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn value(&self) -> u32 {
        self.attack << 24 | self.decay << 16 | self.sustain | self.release << 8
    }
}

pub enum Frequency {
    Frequency(u32),
    Slide(Slide),
}

impl Frequency {
    pub fn value(&self) -> u32 {
        match self {
            Frequency::Frequency(f) => *f,
            Frequency::Slide(s) => s.value(),
        }
    }
}

pub enum Duration {
    Adsr(Adsr),
    Duration(u8),
}

impl Duration {
    pub fn value(&self) -> u32 {
        match self {
            Duration::Duration(d) => *d as u32,
            Duration::Adsr(envelope) => envelope.value(),
        }
    }
}

pub struct Slide {
    start: u32,
    end: u32,
}

impl Slide {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn value(&self) -> u32 {
        self.start | self.end << 16
    }
}

pub struct Sound {
    pub frequency: Frequency,
    pub duration: Duration,
    pub volume: u32,
    pub channel: Channel,
    pub mode: Mode,
}

impl Sound {
    pub fn new(
        frequency: Frequency,
        duration: Duration,
        volume: u32,
        channel: Channel,
        mode: Mode,
    ) -> Self {
        Self {
            frequency,
            duration,
            volume,
            channel,
            mode,
        }
    }

    pub fn play(&self) {
        play_tone(
            &self.frequency,
            &self.duration,
            self.volume,
            self.channel,
            self.mode,
        );
    }
}

pub fn play_tone(
    frequency: &Frequency,
    duration: &Duration,
    volume: u32,
    channel: Channel,
    mode: Mode,
) {
    let channel: u32 = channel as u32;
    let mode: u32 = mode as u32;
    let flags = channel | mode;
    wasm4::tone(frequency.value(), duration.value(), volume as u32, flags);
}
