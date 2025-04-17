fn main() {
    'outer: for i in 0.. {
        'middle: for j in 0.. {
            'inner: for k in 0.. {
                if i + j + k == 1000 {
                    break 'outer;
                }
            }
        }
    }
}
