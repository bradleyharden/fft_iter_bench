#![feature(test)]

const LUT: [i16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

pub fn fft(samples: &[i16; 16]) -> (i32, i32) {
    let sin = LUT.iter().cycle();
    let cos = LUT.iter().cycle().skip(LUT.len() / 4);
    sin.zip(cos).zip(samples).fold(
    (0, 0), |(mut real, mut imag), ((sin, cos), &sample)| {
            real += sample as i32 * (*cos as i32);
            imag += sample as i32 * (*sin as i32);
            (real, imag)
    })
}

pub fn fft_manual(samples: &[i16; 16]) -> (i32, i32) {
    const LUT_LEN: usize = LUT.len();
    let mut real = 0;
    let mut imag = 0;
    for i in 0..(samples.len()/LUT_LEN) {
        for j in 0..LUT_LEN {
            let sin = LUT[j];
            let cos = LUT[(j + (LUT_LEN / 4)) % LUT_LEN];
            let sample = samples[i * LUT_LEN + j];
            real += sample as i32 * (cos as i32);
            imag += sample as i32 * (sin as i32);
        }
    }
    (real, imag)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;
    
    #[test]
    fn test_fft() {
        let samples = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(fft(&samples), (408, 504));
    }
    
    #[test]
    fn test_fft_manual() {
        let samples = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(fft_manual(&samples), (408, 504));
    }
    
    #[bench]
    fn bench_fft(b: &mut Bencher) {
        let samples = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        b.iter(|| {
            fft(&samples)
        })
    }

    #[bench]
    fn bench_fft_manual(b: &mut Bencher) {
        let samples = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        b.iter(|| {
            fft_manual(&samples)
        })
    }
}
