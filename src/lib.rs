use std::fmt::Write;

fn generate_class(str_literal: &str, mut buf: impl Write) {
    write!(buf, "asd").unwrap()
}

fn parse_class(s: &str) {
    // identify if we're using a pseduoclass (i.e. :hover)
    let mut psuedo = None;

    let inner = if s.contains(":") {
        let mut iter = s.split(":");
        let primary = iter.next().unwrap();
        psuedo = Some(primary);
        iter.next().unwrap()
    } else {
        s
    };

    match s {
        _ => {
            // unidentified
        }
    }
}

#[test]
fn test_generate() {
    let mut buf = String::new();
    generate_class("columns-3 w-full aspect-video", &mut buf);
    assert_eq!(buf, "asd");
}

enum Columns {
    // "auto": "auto",
// "1": "1",
// "2": "2",
// "3": "3",
// "4": "4",
// "5": "5",
// "6": "6",
// "7": "7",
// "8": "8",
// "9": "9",
// "10": "10",
// "11": "11",
// "12": "12",
// "3xs": "16rem",
// "2xs": "18rem",
// "xs": "20rem",
// "sm": "24rem",
// "md": "28rem",
// "lg": "32rem",
// "xl": "36rem",
// "2xl": "42rem",
// "3xl": "48rem",
// "4xl": "56rem",
// "5xl": "64rem",
// "6xl": "72rem",
// "7xl": "80rem"
}

enum Spacing {
    // "px": "1px",
// "0": "0px",
// "0.5": "0.125rem",
// "1": "0.25rem",
// "1.5": "0.375rem",
// "2": "0.5rem",
// "2.5": "0.625rem",
// "3": "0.75rem",
// "3.5": "0.875rem",
// "4": "1rem",
// "5": "1.25rem",
// "6": "1.5rem",
// "7": "1.75rem",
// "8": "2rem",
// "9": "2.25rem",
// "10": "2.5rem",
// "11": "2.75rem",
// "12": "3rem",
// "14": "3.5rem",
// "16": "4rem",
// "20": "5rem",
// "24": "6rem",
// "28": "7rem",
// "32": "8rem",
// "36": "9rem",
// "40": "10rem",
// "44": "11rem",
// "48": "12rem",
// "52": "13rem",
// "56": "14rem",
// "60": "15rem",
// "64": "16rem",
// "72": "18rem",
// "80": "20rem",
// "96": "24rem"
}

enum Animation {
    // "none": "none",
// "spin": "spin 1s linear infinite",
// "ping": "ping 1s cubic-bezier(0, 0, 0.2,1) infinite",
// "pulse": "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
// "bounce": "bounce 1s infinite"
}
