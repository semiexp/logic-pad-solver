use serde_json;

mod puzzle;
mod solver;

static mut SHARED_ARRAY: Vec<u8> = Vec::new();

fn solve_puzzle_impl(data: &[u8], underclued: bool) -> String {
    let puzzle: puzzle::Puzzle = match serde_json::from_slice(data) {
        Ok(p) => p,
        Err(e) => {
            return format!("{{\"error\": \"{}\"}}", e);
        }
    };

    let solution = match solver::solve(&puzzle, underclued) {
        Ok(solution) => solution,
        Err(e) => {
            return format!("{{\"error\": \"{}\"}}", e);
        }
    };

    serde_json::to_string(&solution).unwrap()
}

#[no_mangle]
fn solve_puzzle(data: *const u8, len: usize, underclued: i32) -> *const u8 {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let result = solve_puzzle_impl(data, underclued != 0);

    unsafe {
        let result_len = result.len();
        SHARED_ARRAY.clear();
        SHARED_ARRAY.push((result_len & 0xff) as u8);
        SHARED_ARRAY.push(((result_len >> 8) & 0xff) as u8);
        SHARED_ARRAY.push(((result_len >> 16) & 0xff) as u8);
        SHARED_ARRAY.push(((result_len >> 24) & 0xff) as u8);
        SHARED_ARRAY.extend(result.as_bytes());
        SHARED_ARRAY.as_ptr()
    }
}
