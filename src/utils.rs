fn concat_str<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let ptr1 = s1.as_ptr();
    let ptr2 = s2.as_ptr();

    if ptr2 == unsafe { ptr1.add(s1.len()) } {
        let combined_len = s1.len() + s2.len();
        Some(unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr1, combined_len))
        })
    } else {
        None
    }
}

pub fn concat_contiguous_strs<'a, 'b>(slices: &'b [&'a str]) -> Option<&'a str> {
    match slices.first() {
        None => Some(""),
        Some(&first) => slices[1..]
            .iter()
            .try_fold(first, |acc, &slice| concat_str(acc, slice)),
    }
}
