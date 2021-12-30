use crate::wasm4;

pub struct Tone {
    frequency: Frequency,
    duration: Duration,
    volume: u32,
    channel: Channel,
    mode: Mode,
}

impl Tone {
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
            self.frequency,
            self.duration,
            self.volume,
            self.channel,
            self.mode,
        )
    }
}

pub fn play_tone(
    frequency: Frequency,
    duration: Duration,
    volume: u32,
    channel: Channel,
    mode: Mode,
) {
    wasm4::tone(
        frequency.value(),
        duration.value(),
        volume as u32,
        (channel as u32) | (mode as u32),
    );
}

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
    One = wasm4::TONE_MODE1,
    Two = wasm4::TONE_MODE2,
    Three = wasm4::TONE_MODE3,
    Four = wasm4::TONE_MODE4,
}

#[derive(Clone, Copy)]
pub struct Envelope {
    pub attack: u32,
    pub decay: u32,
    pub sustain: u32,
    pub release: u32,
}

impl Envelope {
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

impl From<[u8; 4]> for Envelope {
    fn from(adsr: [u8; 4]) -> Self {
        Envelope {
            attack: adsr[0] as u32,
            decay: adsr[1] as u32,
            sustain: adsr[2] as u32,
            release: adsr[3] as u32,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Duration {
    Adsr(Envelope),
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

impl From<u8> for Duration {
    fn from(d: u8) -> Self {
        Duration::Duration(d)
    }
}

impl From<[u8; 4]> for Duration {
    fn from(adsr: [u8; 4]) -> Self {
        Duration::Adsr(adsr.into())
    }
}

#[derive(Clone, Copy)]
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

impl From<[u32; 2]> for Slide {
    fn from(slide: [u32; 2]) -> Self {
        Slide {
            start: slide[0],
            end: slide[1],
        }
    }
}

#[derive(Clone, Copy)]
pub enum Frequency {
    Frequency(u32),
    Slide(Slide),
}

impl From<u32> for Frequency {
    fn from(f: u32) -> Self {
        Frequency::Frequency(f)
    }
}

impl From<[u32; 2]> for Frequency {
    fn from(slide: [u32; 2]) -> Self {
        Frequency::Slide(slide.into())
    }
}

impl Frequency {
    pub fn value(&self) -> u32 {
        match self {
            Frequency::Frequency(f) => *f,
            Frequency::Slide(s) => s.value(),
        }
    }
}
