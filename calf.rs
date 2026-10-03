//! Studio-Hall und Chorus, nachgebaut nach Calf Studio Gear (Reverb von Krzysztof Foltman,
//! Multi Chorus), Lizenz LGPL 2.1 oder neuer – https://github.com/calf-studio-gear/calf
//! Gleiche Rechenwege (inkl. Festkomma-LFOs), damit der Klang dem bisherigen Calf-Hall entspricht.

use std::f64::consts::PI;

#[inline]
fn san(x: f32) -> f32 {
    if x.abs() < 1e-20 {
        0.0
    } else {
        x
    }
}

// ---------------------------------------------------------------- Einpol-Filter (Calf onepole)
#[derive(Clone, Copy, Default)]
struct OnePole {
    a0: f32,
    a1: f32,
    b1: f32,
    x1: f32,
    y1: f32,
}

impl OnePole {
    fn set_lp(&mut self, fc: f32, sr: f32) {
        let x = (PI * fc as f64 / (2.0 * sr as f64)).tan();
        let q = 1.0 / (1.0 + x);
        self.a0 = (x * q) as f32;
        self.a1 = self.a0;
        self.b1 = ((x - 1.0) * q) as f32;
    }
    fn set_hp(&mut self, fc: f32, sr: f32) {
        let x = (PI * fc as f64 / (2.0 * sr as f64)).tan();
        let q = 1.0 / (1.0 + x);
        self.a0 = q as f32;
        self.a1 = -self.a0;
        self.b1 = ((x - 1.0) * q) as f32;
    }
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let y = x * self.a0 + self.x1 * self.a1 - self.y1 * self.b1;
        self.x1 = x;
        self.y1 = san(y);
        self.y1
    }
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

// ---------------------------------------------------------------- Biquad (Calf biquad_d2, Bandpass nach RBJ)
#[derive(Clone, Copy, Default)]
struct Biquad {
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
    w1: f64,
    w2: f64,
}

impl Biquad {
    fn set_bp_rbj(&mut self, fc: f64, q: f64, sr: f64) {
        let omega = 2.0 * PI * fc / sr;
        let (sn, cs) = (omega.sin(), omega.cos());
        let alpha = sn / (2.0 * q);
        let inv = 1.0 / (1.0 + alpha);
        self.a0 = inv * alpha;
        self.a1 = 0.0;
        self.a2 = -inv * alpha;
        self.b1 = -2.0 * cs * inv;
        self.b2 = (1.0 - alpha) * inv;
    }
    #[inline]
    fn process(&mut self, x: f64) -> f64 {
        let n = x - self.w1 * self.b1 - self.w2 * self.b2;
        let out = n * self.a0 + self.w1 * self.a1 + self.w2 * self.a2;
        self.w2 = self.w1;
        self.w1 = if n.abs() < 1e-20 { 0.0 } else { n };
        out
    }
}

// ---------------------------------------------------------------- Verzoegerungsleitung (Calf simple_delay)
struct Delay {
    d: Vec<f32>,
    pos: usize,
    mask: usize,
}

impl Delay {
    fn new(n: usize) -> Self {
        Delay { d: vec![0.0; n], pos: 0, mask: n - 1 }
    }
    fn reset(&mut self) {
        self.d.iter_mut().for_each(|v| *v = 0.0);
        self.pos = 0;
    }
    #[inline]
    fn put(&mut self, x: f32) {
        self.d[self.pos] = x;
        self.pos = (self.pos + 1) & self.mask;
    }
    #[inline]
    fn get_interp(&self, delay: usize, frac: f32) -> f32 {
        let n = self.mask + 1;
        let ppos = (self.pos + n - (delay & self.mask)) & self.mask;
        let pppos = (ppos + n - 1) & self.mask;
        let a = self.d[ppos];
        a + (self.d[pppos] - a) * frac
    }
    #[inline]
    fn allpass_comb_lerp16(&mut self, x: f32, delay: u32, fb: f32) -> f32 {
        let old = self.get_interp((delay >> 16) as usize, (delay & 0xFFFF) as f32 * (1.0 / 65536.0));
        let cur = san(x + fb * old);
        self.put(cur);
        old - fb * cur
    }
}

fn sine_table(n: usize, mult: f64) -> Vec<i32> {
    (0..=n).map(|i| (mult * (i as f64 * 2.0 * PI / n as f64).sin()) as i32).collect()
}

// ---------------------------------------------------------------- Calf Reverb
/// Einstellungen eines Hallraums (Calf-Parameter)
#[derive(Clone, Copy)]
pub struct Room {
    pub decay: f32,
    pub hf_damp: f32,
    pub room_size: f32,
    pub diffusion: f32,
    pub amount: f32,
    pub dry: f32,
    pub predelay: f32,
    pub bass_cut: f32,
    pub treble_cut: f32,
}

const fn room(decay: f32, hf_damp: f32, room_size: f32, diffusion: f32, amount: f32, dry: f32,
              predelay: f32, bass_cut: f32, treble_cut: f32) -> Room {
    Room { decay, hf_damp, room_size, diffusion, amount, dry, predelay, bass_cut, treble_cut }
}

/// 0 = Calf-Grundeinstellung (bisheriger Studio-Hall), 1..6 = die Calf-Voreinstellungen
pub const ROOMS: [Room; 7] = [
    room(1.5, 5000.0, 2.0, 0.5, 0.25, 1.0, 0.0, 300.0, 5000.0),
    room(0.445945, 5508.46, 4.0, 0.54, 0.469761, 1.0, 25.0, 257.65, 20000.0), // Room
    room(1.10354, 2182.58, 4.0, 0.69, 0.291183, 1.0, 6.5, 514.079, 4064.15),  // Ambience
    room(0.505687, 3971.64, 4.0, 0.17, 0.198884, 1.0, 13.0, 240.453, 3303.47), // Empty walls
    room(1.0, 3396.49, 2.0, 0.5, 0.269807, 1.0, 0.0, 300.0, 5000.0),         // DiscoVerb
    room(2.00689, 20000.0, 2.0, 0.5, 0.366022, 1.0, 0.0, 300.0, 5000.0),     // Large Empty Hall
    room(1.45397, 9795.58, 2.0, 0.5, 0.184284, 1.0, 0.0, 300.0, 5000.0),     // Large Occupied Hall
];

pub struct CalfReverb {
    sr: f32,
    ap_l: Vec<Delay>,
    ap_r: Vec<Delay>,
    phase: u32,
    dphase: u32,
    sine: Vec<i32>,
    lp_l: OnePole,
    lp_r: OnePole,
    old_l: f32,
    old_r: f32,
    fb: f32,
    tl: [i32; 6],
    tr: [i32; 6],
    ldec: [f32; 6],
    rdec: [f32; 6],
    pre: [Delay; 2],
    pre_amt: usize,
    lo: [OnePole; 2],
    hi: [OnePole; 2],
    wet: f32,
    dry: f32,
}

const ROOM_TIMES: [[(i32, i32); 6]; 6] = [
    [(397, 383), (457, 429), (549, 631), (649, 756), (773, 803), (877, 901)],
    [(697, 783), (957, 929), (649, 531), (1049, 1177), (473, 501), (587, 681)],
    [(697, 783), (957, 929), (649, 531), (1249, 1377), (1573, 1671), (1877, 1781)],
    [(1097, 1087), (1057, 1031), (1049, 1039), (1083, 1055), (1075, 1099), (1003, 1073)],
    [(197, 133), (357, 229), (549, 431), (949, 1277), (1173, 1671), (1477, 1881)],
    [(197, 133), (257, 179), (549, 431), (619, 497), (1173, 1371), (1577, 1881)],
];

impl CalfReverb {
    pub fn new(sr: f32) -> Self {
        let mut r = CalfReverb {
            sr,
            ap_l: (0..6).map(|_| Delay::new(2048)).collect(),
            ap_r: (0..6).map(|_| Delay::new(2048)).collect(),
            phase: 0,
            dphase: ((0.5 * 128.0 / sr as f64) * (1u64 << 25) as f64) as u32,
            sine: sine_table(128, 10000.0),
            lp_l: OnePole::default(),
            lp_r: OnePole::default(),
            old_l: 0.0,
            old_r: 0.0,
            fb: 0.0,
            tl: [0; 6],
            tr: [0; 6],
            ldec: [0.0; 6],
            rdec: [0.0; 6],
            pre: [Delay::new(131072), Delay::new(131072)],
            pre_amt: 1,
            lo: [OnePole::default(); 2],
            hi: [OnePole::default(); 2],
            wet: 0.25,
            dry: 1.0,
        };
        r.set_room(0);
        r
    }

    pub fn reset(&mut self) {
        for d in self.ap_l.iter_mut().chain(self.ap_r.iter_mut()) {
            d.reset();
        }
        for d in self.pre.iter_mut() {
            d.reset();
        }
        self.lp_l.reset();
        self.lp_r.reset();
        for f in self.lo.iter_mut().chain(self.hi.iter_mut()) {
            f.reset();
        }
        self.old_l = 0.0;
        self.old_r = 0.0;
        self.phase = 0;
    }

    pub fn set_room(&mut self, idx: usize) {
        let p = ROOMS[idx.min(ROOMS.len() - 1)];
        let sr = self.sr;
        let typ = (p.room_size.round() as i32).clamp(0, 5) as usize;
        let f_dec = 1000.0 + 2400.0 * p.diffusion;
        for i in 0..6 {
            let (l, r) = ROOM_TIMES[typ][i];
            self.tl[i] = l << 16;
            self.tr[i] = r << 16;
            self.ldec[i] = (-(l as f32) / f_dec).exp();
            self.rdec[i] = (-(r as f32) / f_dec).exp();
        }
        self.fb = 1.0 - 0.3 / (p.decay * sr / 44100.0);
        self.lp_l.set_lp(p.hf_damp, sr);
        self.lp_r.set_lp(p.hf_damp, sr);
        let tc = p.treble_cut.clamp(20.0, sr * 0.49);
        let bc = p.bass_cut.clamp(20.0, sr * 0.49);
        for f in self.lo.iter_mut() {
            f.set_lp(tc, sr);
        }
        for f in self.hi.iter_mut() {
            f.set_hp(bc, sr);
        }
        self.pre_amt = ((sr * p.predelay / 1000.0 + 1.0) as usize).min(131071);
        self.wet = p.amount;
        self.dry = p.dry;
        self.reset();
    }

    #[inline]
    fn core(&mut self, left: &mut f32, right: &mut f32) {
        let ip = (self.phase >> 25) as usize;
        let fp = ((self.phase >> 11) & 0x3FFF) as i32;
        let (a, b) = (self.sine[ip], self.sine[ip + 1]);
        let lfo = (a + (((b - a) * fp) >> 14)) >> 2;
        self.phase = self.phase.wrapping_add(self.dphase);
        let m = [-45, 47, 54, -69, 69, -46];

        let mut l = *left + self.old_r;
        l = self.ap_l[0].allpass_comb_lerp16(l, (self.tl[0] + m[0] * lfo) as u32, self.ldec[0]);
        l = self.ap_l[1].allpass_comb_lerp16(l, (self.tl[1] + m[1] * lfo) as u32, self.ldec[1]);
        let out_l = l;
        for i in 2..6 {
            l = self.ap_l[i].allpass_comb_lerp16(l, (self.tl[i] + m[i] * lfo) as u32, self.ldec[i]);
        }
        self.old_l = san(self.lp_l.process(l * self.fb));

        let mut r = *right + self.old_l;
        r = self.ap_r[0].allpass_comb_lerp16(r, (self.tr[0] + m[0] * lfo) as u32, self.rdec[0]);
        r = self.ap_r[1].allpass_comb_lerp16(r, (self.tr[1] + m[1] * lfo) as u32, self.rdec[1]);
        let out_r = r;
        for i in 2..6 {
            r = self.ap_r[i].allpass_comb_lerp16(r, (self.tr[i] + m[i] * lfo) as u32, self.rdec[i]);
        }
        self.old_r = san(self.lp_r.process(r * self.fb));
        *left = out_l;
        *right = out_r;
    }

    #[inline]
    pub fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        // Vorverzoegerung (mindestens 1 Abtastwert, wie im Original)
        let n = 131072usize;
        let ppl = (self.pre[0].pos + n - self.pre_amt) & (n - 1);
        let dl = self.pre[0].d[ppl];
        let dr = self.pre[1].d[ppl];
        self.pre[0].put(l);
        self.pre[1].put(r);
        let mut rl = self.lo[0].process(self.hi[0].process(dl));
        let mut rr = self.lo[1].process(self.hi[1].process(dr));
        self.core(&mut rl, &mut rr);
        (self.dry * l + self.wet * rl, self.dry * r + self.wet * rr)
    }
}

// ---------------------------------------------------------------- Calf Multi Chorus (Grundeinstellung)
struct ChorusCh {
    delay: Delay,
    phase: u32,
    f1: Biquad,
    f2: Biquad,
}

pub struct CalfChorus {
    ch: [ChorusCh; 2],
    dphase: u32,
    vphase: u32,
    sine: Vec<i32>,
    mds: i32,
    mdepth: i32,
    voices: u32,
    voice_offset: i32,
    voice_depth: u32,
    scale: f32,
    wet: f32,
    dry: f32,
}

impl CalfChorus {
    pub fn new(sr: f32) -> Self {
        let srd = sr as f64;
        // Grundeinstellung des Calf Multi Chorus
        let (min_delay, mod_depth, rate, stereo, voices, vphase_deg) = (0.005, 0.006, 0.1, 180.0, 4u32, 64.0);
        let (amount, dry, freq, freq2, q, overlap) = (0.5f32, 0.5f32, 100.0, 5000.0, 0.125, 0.75f32);
        let fix20 = |x: f64| (x * (1u64 << 20) as f64) as u32;
        let min_delay_samples = (min_delay * 65536.0 * srd) as i32;
        let mod_depth_samples = (mod_depth * 32.0 * srd) as i32;
        let range = 1.0 + (1.0 - overlap) * (voices as f32 - 1.0);
        let mk = |phase: u32| {
            let mut f1 = Biquad::default();
            let mut f2 = Biquad::default();
            f1.set_bp_rbj(freq, q, srd);
            f2.set_bp_rbj(freq2, q, srd);
            ChorusCh { delay: Delay::new(4096), phase, f1, f2 }
        };
        CalfChorus {
            ch: [mk(0), mk(fix20(stereo / 360.0 * 4096.0))],
            dphase: fix20(rate / srd * 4096.0),
            vphase: fix20(vphase_deg / 360.0 * (4096.0 / (voices.max(2) - 1) as f64)),
            sine: sine_table(4096, 65535.0),
            mds: min_delay_samples + mod_depth_samples * 1024 + 2 * 65536,
            mdepth: mod_depth_samples >> 2,
            voices,
            voice_offset: (131072.0 * (1.0 - overlap) / range) as i32,
            voice_depth: ((1u64 << 30) as f64 * (1.0 / range as f64)) as u32,
            scale: (1.0 / voices as f32).sqrt(),
            wet: amount,
            dry,
        }
    }

    pub fn reset(&mut self) {
        for c in self.ch.iter_mut() {
            c.delay.reset();
            c.f1.w1 = 0.0;
            c.f1.w2 = 0.0;
            c.f2.w1 = 0.0;
            c.f2.w2 = 0.0;
        }
    }

    #[inline]
    fn lfo_value(&self, phase: u32, v: u32) -> i32 {
        let vp = phase.wrapping_add(self.vphase.wrapping_mul(v));
        let ip = (vp >> 20) as usize;
        let fp = ((vp >> 6) & 0x3FFF) as i32;
        let (a, b) = (self.sine[ip], self.sine[ip + 1]);
        let intval = a + (((b - a) * fp) >> 14);
        -65535 + v as i32 * self.voice_offset + ((((self.voice_depth >> 17) as i64) * (65536 + intval) as i64) >> 13) as i32
    }

    #[inline]
    pub fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        let mut out = [0.0f32; 2];
        for (k, x) in [l, r].into_iter().enumerate() {
            let phase = self.ch[k].phase;
            self.ch[k].delay.put(x);
            let mut acc = 0.0f32;
            for v in 0..self.voices {
                let lfo = self.lfo_value(phase, v);
                let dv = self.mds + (((self.mdepth as i64 * lfo as i64) >> 4) as i32);
                let ifv = (dv >> 16) as usize;
                acc += self.ch[k].delay.get_interp(ifv, (dv & 0xFFFF) as f32 * (1.0 / 65536.0));
            }
            let c = &mut self.ch[k];
            let post = (c.f2.process(acc as f64) + c.f1.process(acc as f64)) as f32;
            out[k] = x * self.dry + post * self.wet * self.scale;
            c.phase = c.phase.wrapping_add(self.dphase);
        }
        (out[0], out[1])
    }
}
