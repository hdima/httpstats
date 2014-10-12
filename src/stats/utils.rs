pub fn format_duration(duration: u64) -> String {
    let (mut n, mut pos, modifier) = if duration < 1000 {
            (duration, 3, "s")
        } else if duration < 60 * 1000 {
            (duration / 100, 1, "s")
        } else if duration < 60 * 60 * 1000 {
            (duration / (60 * 100), 1, "m")
        } else if duration < 24 * 60 * 60 * 1000 {
            (duration / (60 * 60 * 100), 1, "h")
        } else {
            (duration / (24 * 60 * 60 * 100), 1, "d")
        };
    while pos > 0 && n % 10 == 0 {
        n /= 10;
        pos -= 1;
    }
    let int_part = (n / pow(10, pos)).to_string();
    let mut fract_part = String::with_capacity(pos as uint + 1);
    if pos != 0 && n != 0 {
        fract_part.push('.');
        while pos > 0 {
            let digit = (n / pow(10, pos - 1)) % 10;
            fract_part.push((digit as u8 + '0' as u8) as char);
            pos -= 1;
        }
    }
    int_part + fract_part + modifier
}

// FIXME: Can we replace it with std::num::pow?
fn pow(n: u64, p: uint) -> u64 {
    let mut num = 1;
    for _ in range(0, p) {
        num *= n;
    }
    num
}

pub fn format_bytes(mut bytes: u64) -> String {
    static MODIFIERS: [&'static str, ..5] = ["T", "G", "M", "K", ""];
    let mut i = MODIFIERS.len() - 1;
    while bytes >= 1024 && i != 0 {
        bytes = (bytes / 1024) + (bytes % 1024) / 513;
        i -= 1;
    }
    bytes.to_string() + MODIFIERS[i]
}

pub fn format_number(mut bytes: u64) -> String {
    static MODIFIERS: [&'static str, ..5] = ["T", "G", "M", "K", ""];
    let mut i = MODIFIERS.len() - 1;
    while bytes >= 1000 && i != 0 {
        bytes = (bytes / 1000) + (bytes % 1000) / 501;
        i -= 1;
    }
    bytes.to_string() + MODIFIERS[i]
}

/*
 * Tests
 */
#[cfg(test)]
mod test {
    use super::{format_duration, format_bytes, format_number};

    #[inline]
    fn assert_duration(exp: &str, input: u64) {
        assert_eq!(exp, format_duration(input).as_slice());
    }

    #[test]
    fn test_format_duration() {
        assert_duration("0s", 0);
        assert_duration("0.009s", 9);
        assert_duration("0.09s", 90);
        assert_duration("0.9s", 900);
        assert_duration("0.999s", 999);
        assert_duration("1s", 1000);
        assert_duration("1s", 1009);
        assert_duration("1.4s", 1400);
        assert_duration("1.6s", 1600);
        assert_duration("59s", 59000);
        assert_duration("1m", 60000);
        assert_duration("1.5m", 90000);
        assert_duration("1h", 60 * 60 * 1000);
        assert_duration("1.5h", 90 * 60 * 1000);
        assert_duration("10h", 10 * 60 * 60 * 1000);
        assert_duration("1d", 24 * 60 * 60 * 1000);
        assert_duration("1.5d", 36 * 60 * 60 * 1000);
    }

    #[inline]
    fn assert_bytes(exp: &str, input: u64) {
        assert_eq!(exp, format_bytes(input).as_slice());
    }

    #[test]
    fn test_format_bytes() {
        assert_bytes("0", 0);
        assert_bytes("9", 9);
        assert_bytes("90", 90);
        assert_bytes("900", 900);
        assert_bytes("1000", 1000);
        assert_bytes("1K", 1024);
        assert_bytes("2K", 2048);
        assert_bytes("4K", 4096);
        assert_bytes("4K", 4608);
        assert_bytes("5K", 4609);
        assert_bytes("1M", 1024 * 1024);
        assert_bytes("1G", 1024 * 1024 * 1024);
        assert_bytes("1T", 1024 * 1024 * 1024 * 1024);
        assert_bytes("1024T", 1024 * 1024 * 1024 * 1024 * 1024);
    }

    #[inline]
    fn assert_number(exp: &str, input: u64) {
        assert_eq!(exp, format_number(input).as_slice());
    }

    #[test]
    fn test_format_number() {
        assert_number("0", 0);
        assert_number("9", 9);
        assert_number("90", 90);
        assert_number("900", 900);
        assert_number("1K", 1000);
        assert_number("2K", 2000);
        assert_number("4K", 4000);
        assert_number("4K", 4500);
        assert_number("5K", 4501);
        assert_number("1M", 1000 * 1000);
        assert_number("1G", 1000 * 1000 * 1000);
        assert_number("1T", 1000 * 1000 * 1000 * 1000);
        assert_number("1000T", 1000 * 1000 * 1000 * 1000 * 1000);
    }
}
