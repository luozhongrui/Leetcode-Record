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

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn add_two(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            carry: i32,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (None, None) => {
                    if let 0 = carry {
                        None
                    } else {
                        Some(Box::new(ListNode::new(carry)))
                    }
                }
                (None, Some(node2)) => add_two(Some(node2), None, carry),
                (Some(mut node1), None) => {
                    let sum = node1.val + carry;
                    node1.val = sum % 10;
                    node1.next = add_two(node1.next, None, sum / 10);
                    Some(node1)
                }
                (Some(mut node1), Some(node2)) => {
                    let sum = node1.val + node2.val + carry;
                    node1.val = sum % 10;
                    node1.next = add_two(node1.next, node2.next, sum / 10);
                    Some(node1)
                }
            }
        }
        add_two(l1, l2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::add_two_numbers;
        let l1 = ListNode::from_vec(&[2, 4, 3]);
        let l2 = ListNode::from_vec(&[5, 6, 4]);
        let l3 = add_two_numbers(l1, l2);
        assert_eq!(vec![7, 0, 8], ListNode::to_vec(&l3));
    }
}
