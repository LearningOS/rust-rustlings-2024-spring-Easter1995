/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

// 定义堆结构
pub struct Heap<T>
where
    T: Default,
{
    count: usize, // 堆节点的个数
    items: Vec<T>, // 用向量来存储整个堆元素
    comparator: fn(&T, &T) -> bool, // 函数指针，堆元素比较器
}

impl<T> Heap<T>
where
    T: Default + std::cmp::PartialOrd
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 从下标为1的地方开始存堆
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1; // index从1开始
        self.items.push(value); // 先在堆的最后添加一个节点
        let mut index = self.count;
        
        while index > 1 {
            let parent_idx = self.parent_idx(index); // 获取最后一个元素的父元素
            if (self.comparator)(&self.items[index], &self.items[parent_idx]) {
                self.items.swap(index, parent_idx); // 将它和父元素交换
                index = parent_idx; // 发生交换后index移动到父元素的位置
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // 判断idx是否有孩子
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 按堆规定的顺序返回两个孩子中应该在前面的一个的索引值
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right > self.count {
            left
        } else if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap 小根堆，小于时返回true
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap 大根堆，大于时返回true
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::cmp::PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        // 相当于堆排序
        let ret = self.items.swap_remove(1); // 将堆顶元素输出，堆底元素升上来
        self.count -= 1;
        // 再次调整堆为大根堆或小根堆
        let mut index = 1;
        // 当当前元素有子元素时
        while self.children_present(index) {
            let child_index = self.smallest_child_idx(index);
            if (self.comparator)(&self.items[child_index], &self.items[index]) {
                self.items.swap(child_index, index);
                index = child_index;
            }
            else {
                break; // 如果不break会进入死循环
            }
        }

        Some(ret)
    }
}

pub struct MinHeap; // 定义小根堆

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}