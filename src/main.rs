#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Run {
    val: bool,
    len: usize,
}

struct RleOrIterator<T> {
    iter_a: T,
    iter_b: T,
    current_a: Option<Run>,
    current_b: Option<Run>,
}

impl<T> RleOrIterator<T>
where
    T: Iterator<Item = Run>,
{
    fn new(iter_a: T, iter_b: T) -> Self {
        Self {
            iter_a,
            iter_b,
            current_a: None,
            current_b: None,
        }
    }
}

impl<T> Iterator for RleOrIterator<T>
where
    T: Iterator<Item = Run>,
{
    type Item = Run;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_a.map_or(true, |r| r.len == 0) {
            self.current_a = self.iter_a.next();
        }

        if self.current_b.map_or(true, |r| r.len == 0) {
            self.current_b = self.iter_b.next();
        }

        match (self.current_a.as_mut(), self.current_b.as_mut()) {
            (Some(run_a), Some(run_b)) => {
                let overlap_len = run_a.len.min(run_b.len);

                let result_val = run_a.val | run_b.val;

                run_a.len -= overlap_len;
                run_b.len -= overlap_len;

                Some(Run {
                    val: result_val,
                    len: overlap_len,
                })
            }
            _ => None,
        }
    }
}

fn main() {
    let a = vec![Run { val: false, len: 3 }, Run { val: true, len: 2 }];
    let b = vec![Run { val: false, len: 1 }, Run { val: true, len: 4 }];

    let merger = RleOrIterator::new(a.into_iter(), b.into_iter());

    for run in merger {
        println!("Val: {}, Len: {}", run.val, run.len);
    }
}
