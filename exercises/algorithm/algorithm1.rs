/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug, Clone, Copy)]
// 定义链表里面的节点，每个Node可以指向下一个Node
struct Node<T> {
    val: T,
    // NonNull<T> 类型是一个智能指针，它保证了指针指向的对象是非空的
    next: Option<NonNull<Node<T>>>,
}

// 新建节点，next指向None
impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug, Clone, Copy)]
// 定义结构体LinkedList，表示链表
struct LinkedList<T> {
    length: u32, // 长度
    start: Option<NonNull<Node<T>>>, // 开始节点
    end: Option<NonNull<Node<T>>>, // 终止节点
}

// 默认方法，声明一个链表
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    // 默认创建方法，创建一个空链表
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
}

impl<T: Clone + std::cmp::PartialOrd> LinkedList<T> {
    // 往链表里面添加元素
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj)); // 新建节点
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) }); // 指向node的裸指针
        match self.end {
            None => self.start = node_ptr, // 新建节点是链表中的第一个节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // 链表结尾的next指向新建节点
        }
        self.end = node_ptr; // 更新链表结尾
        self.length += 1; // 更新链表长度
    }

    // 获取索引为index的节点
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 返回节点引用
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None, // 说明链表到头了
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }), // 创建一个对裸指针指向的val字段的不安全引用
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1), // 递归直到index变为0
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
		//TODO
        let mut merged_list = LinkedList::new();
        // 获取链表的开始节点
        let mut node_a = list_a.start; 
        let mut node_b = list_b.start;
        
        while node_a.is_some() || node_b.is_some() {
            // 指针解引用，其中一个指针有可能为None
            // 但是不获得val的所有权
            let a_val = node_a.map(|ptr| unsafe{ &(*ptr.as_ptr()).val });
            let b_val = node_b.map(|ptr| unsafe{ &(*ptr.as_ptr()).val });

            // 比较大小
            match (a_val, b_val) {
                // 两个都非空
                (Some(a), Some(b)) => {
                    if a < b {
                        merged_list.add(a.clone());
                        // 指针解引用且获得指针内容的所有权
                        node_a = unsafe{ (*node_a.unwrap().as_ptr()).next };
                    } else {
                        merged_list.add(b.clone());
                        node_b = unsafe{ (*node_b.unwrap().as_ptr()).next };
                    }
                },
                // a已经空了，直接把b剩下的元素全部加进链表
                (None, Some(b)) => {
                    merged_list.add(b.clone());
                    node_b = unsafe{ (*node_b.unwrap().as_ptr()).next };
                },
                (Some(a), None) => {
                    merged_list.add(a.clone());
                    node_a = unsafe{ (*node_a.unwrap().as_ptr()).next };
                },
                (None, None) => {
                    break
                }
            }
        }

        merged_list
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