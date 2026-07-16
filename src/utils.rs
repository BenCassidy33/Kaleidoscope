
pub fn find_closing_delim<const N: usize>(
    input: &str,
    opening: [char; N],
    closing: char,
) -> Result<std::ops::Range<usize>, isize> {
    let mut count = 0isize;
    let mut first = -1isize;

    for (i, c) in input.chars().enumerate() {
        if opening.contains(&c) {
            if first == -1 {
                first = i as isize;
            }
            count += 1;
            continue;
        }

        if c == closing {
            count -= 1;
        }

        if count == 0 {
            if first != -1 {
                return Ok(first as usize..i);
            }

            return Err(count);
        }
    }

    Err(count)
}
