/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

// 定义队列，队列使用向量实现的
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    // 入队
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    // 出队，出第一个元素
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Stack is empty")
        }
    }

    // 看队头的元素的值
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Stack is empty"),
        }
    }

    // 查看队列长度
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // 查看队列是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// 新建队列
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 栈，包含两个队列
pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			q1:Queue::<T>::new(), // 进行入队操作
			q2:Queue::<T>::new()  // 进行出队操作
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem); // 入队列q1
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 将q1里面的除最后一个元素依次出栈存到q2
        for n in 1..self.q1.size() {
            if let Ok(value) = self.q1.dequeue() {
                self.q2.enqueue(value);
            }
        }
        std::mem::swap(&mut self.q1, &mut self.q2); // 交换q1、q2所有权
        self.q2.dequeue() // q2负责出队
    }
    pub fn is_empty(&self) -> bool {
		self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}