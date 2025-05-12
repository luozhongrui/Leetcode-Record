#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        if values.len() == 0 {
            return None;
        }
        let mut head = ListNode::new(values[0]);
        head.next = ListNode::from_vec(&values[1..]);
        Some(Box::new(head))
    }

    #[inline]
    pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut head = head;
        while let Some(node) = head {
            res.push(node.val);
            head = &node.next;
        }
        res
    }
}

pub mod solution {
    use super::*;
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn remove_nth_from_end_recr(
            head: Option<Box<ListNode>>,
            n: i32,
        ) -> (Option<Box<ListNode>>, usize) {
            match head {
                None => (None, 1),
                Some(mut current) => {
                    // 递归下去，走到最后的None再往上回来，因此num在第一个执行的地方是(None, 1) -> 递归上一层 (Some(Node), 2) -> ...
                    let (next, num) = remove_nth_from_end_recr(current.next.take(), n);
                    if n == num as i32 {
                        (next, num + 1)
                    } else {
                        current.next = next;
                        (Some(current), num + 1)
                    }
                }
            }
        }
        remove_nth_from_end_recr(head, n).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use solution::remove_nth_from_end;
        assert_eq!(
            ListNode::to_vec(&remove_nth_from_end(
                ListNode::from_vec(&[1, 2, 3, 4, 5]),
                2
            )),
            vec![1, 2, 3, 5]
        );
    }
}
