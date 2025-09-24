/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}

impl<T: PartialOrd + Display> LinkedList<T> {
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        // 处理其中一个或两个链表为空的边界情况
        if list_a.start.is_none() {
            return list_b;
        }
        if list_b.start.is_none() {
            return list_a;
        }

        let mut merged_list = LinkedList::new();
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        // 在 unsafe 块中进行指针操作
        unsafe {
            // 1. 确定合并后链表的头结点
            let head;
            if (*current_a.unwrap().as_ptr()).val <= (*current_b.unwrap().as_ptr()).val {
                head = current_a;
                current_a = (*head.unwrap().as_ptr()).next;
            } else {
                head = current_b;
                current_b = (*head.unwrap().as_ptr()).next;
            }
            merged_list.start = head;
            let mut tail = head; // tail 指向新链表的尾部

            // 2. 循环比较并链接节点
            while let (Some(ptr_a), Some(ptr_b)) = (current_a, current_b) {
                if (*ptr_a.as_ptr()).val <= (*ptr_b.as_ptr()).val {
                    (*tail.unwrap().as_ptr()).next = Some(ptr_a);
                    tail = Some(ptr_a);
                    current_a = (*ptr_a.as_ptr()).next;
                } else {
                    (*tail.unwrap().as_ptr()).next = Some(ptr_b);
                    tail = Some(ptr_b);
                    current_b = (*ptr_b.as_ptr()).next;
                }
            }

            // 3. 链接剩余的节点
            if let Some(ptr) = current_a {
                (*tail.unwrap().as_ptr()).next = Some(ptr);
                merged_list.end = list_a.end;
            } else if let Some(ptr) = current_b {
                (*tail.unwrap().as_ptr()).next = Some(ptr);
                merged_list.end = list_b.end;
            }

            // 4. 清空旧链表的指针，防止内存被重复释放
            list_a.start = None;
            list_a.end = None;
            list_b.start = None;
            list_b.end = None;
        }

        // 5. 更新新链表的长度
        merged_list.length = list_a.length + list_b.length;
        merged_list
    }
}