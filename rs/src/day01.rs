use anyhow::Result;

mod part1 {
    use core::fmt;
    use std::str;

    use anyhow::Result;

    #[derive(Debug)]
    struct Digit(u32);

    impl TryFrom<&[u8]> for Digit {
        type Error = anyhow::Error;

        fn try_from(value: &[u8]) -> Result<Self> {
            if value[0].is_ascii_digit() {
                Ok(Digit((value[0] - b'0') as u32))
            } else {
                anyhow::bail!(format!("no digit at: '{}'", String::from_utf8_lossy(value)))
            }
        }
    }

    impl fmt::Display for Digit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct Calibration<'a> {
        // calibration line
        line: &'a str,
        // digits parsed from the calibration line
        digits: Vec<Digit>,
        // value of the calibration line
        value: u32,
    }

    impl<'a> TryFrom<&'a str> for Calibration<'a> {
        type Error = anyhow::Error;

        fn try_from(line: &'a str) -> Result<Self> {
            let bytes = line.as_bytes();
            let digits = (0..bytes.len())
                .flat_map(|i| bytes[i..].try_into().ok())
                .collect::<Vec<_>>();
            let value = match digits.as_slice() {
                [Digit(d)] => d * 10 + d,
                [Digit(d1), .., Digit(d2)] => d1 * 10 + d2,
                _ => anyhow::bail!(format!("invalid calibration line: '{}'", line)),
            };

            Ok(Calibration {
                line,
                digits,
                value,
            })
        }
    }

    impl fmt::Display for Calibration<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:80}", self.line)?;
            write!(f, " => value = {:2}: digits = ", self.value)?;
            for d in &self.digits {
                write!(f, "{}, ", d)?;
            }
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct Calibrations<'a>(Vec<Calibration<'a>>);

    impl<'a> TryFrom<&'a str> for Calibrations<'a> {
        type Error = anyhow::Error;

        fn try_from(s: &'a str) -> Result<Self> {
            s.lines()
                .map(Calibration::try_from)
                .collect::<Result<Vec<_>>>()
                .map(Calibrations)
        }
    }

    impl fmt::Display for Calibrations<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for c in &self.0 {
                writeln!(f, "{}", c)?;
            }
            Ok(())
        }
    }

    impl Calibrations<'_> {
        pub fn sum(&self) -> u32 {
            self.0.iter().map(|c| c.value).sum()
        }
    }
}

mod part2 {
    use core::fmt;
    use std::str;

    use anyhow::Result;

    #[derive(Debug)]
    struct Digit(u32);

    impl TryFrom<&[u8]> for Digit {
        type Error = anyhow::Error;

        fn try_from(value: &[u8]) -> Result<Self> {
            #[rustfmt::skip]
        static ZERO_TO_NINE: [(u32, &[u8]); 10] = [
            (0, b"zero"),
            (1, b"one"),
            (2, b"two"),
            (3, b"three"),
            (4, b"four"),
            (5, b"five"),
            (6, b"six"),
            (7, b"seven"),
            (8, b"eight"),
            (9, b"nine"),
        ];

            if value[0].is_ascii_digit() {
                Ok(Digit((value[0] - b'0') as u32))
            } else {
                ZERO_TO_NINE
                    .iter()
                    .find_map(|&(v, d)| value.starts_with(d).then_some(Digit(v)))
                    .ok_or(anyhow::anyhow!(format!(
                        "no digit at: '{}'",
                        String::from_utf8_lossy(value)
                    )))
            }
        }
    }

    impl fmt::Display for Digit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct Calibration<'a> {
        // calibration line
        line: &'a str,
        // digits parsed from the calibration line
        digits: Vec<Digit>,
        // value of the calibration line
        value: u32,
    }

    impl<'a> TryFrom<&'a str> for Calibration<'a> {
        type Error = anyhow::Error;

        fn try_from(line: &'a str) -> Result<Self> {
            let bytes = line.as_bytes();
            let digits = (0..bytes.len())
                .flat_map(|i| bytes[i..].try_into().ok())
                .collect::<Vec<_>>();
            let value = match digits.as_slice() {
                [Digit(d)] => d * 10 + d,
                [Digit(d1), .., Digit(d2)] => d1 * 10 + d2,
                _ => anyhow::bail!(format!("invalid calibration line: '{}'", line)),
            };

            Ok(Calibration {
                line,
                digits,
                value,
            })
        }
    }

    impl fmt::Display for Calibration<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:80}", self.line)?;
            write!(f, " => value = {:2}: digits = ", self.value)?;
            for d in &self.digits {
                write!(f, "{}, ", d)?;
            }
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct Calibrations<'a>(Vec<Calibration<'a>>);

    impl<'a> TryFrom<&'a str> for Calibrations<'a> {
        type Error = anyhow::Error;

        fn try_from(s: &'a str) -> Result<Self> {
            s.lines()
                .map(Calibration::try_from)
                .collect::<Result<Vec<_>>>()
                .map(Calibrations)
        }
    }

    impl fmt::Display for Calibrations<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for c in &self.0 {
                writeln!(f, "{}", c)?;
            }
            Ok(())
        }
    }

    impl Calibrations<'_> {
        pub fn sum(&self) -> u32 {
            self.0.iter().map(|c| c.value).sum()
        }
    }
}

pub fn part1() -> Result<()> {
    let input = include_str!("../../input/day01.txt");
    let calibrations = part1::Calibrations::try_from(input)?;
    tracing::debug!("[part 1] parsed calibrations: \n{}", calibrations);
    let ans = calibrations.sum();
    tracing::info!("[part 1] sum of calibration values: {}", ans);
    assert_eq!(ans, 54927);
    Ok(())
}

pub fn part2() -> Result<()> {
    let input = include_str!("../../input/day01.txt");
    let calibrations = part2::Calibrations::try_from(input)?;
    tracing::debug!("[part 2] parsed calibrations: \n{}", calibrations);
    let ans = calibrations.sum();
    tracing::info!("[part 2] sum of calibration values: {}", ans);
    assert_eq!(ans, 54581);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day01a.txt");
        let calibrations = part1::Calibrations::try_from(input)?;
        assert_eq!(calibrations.sum(), 142);

        let input = include_str!("../../sample/day01b.txt");
        let calibrations = part2::Calibrations::try_from(input)?;
        assert_eq!(calibrations.sum(), 281);

        Ok(())
    }
}
