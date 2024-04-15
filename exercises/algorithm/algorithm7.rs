/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
// 定义了一个栈，向量的尾部就是栈顶
struct Stack<T> {
	size: usize, // 栈大小
	data: Vec<T>, // 用向量来存储数据
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	// 判断栈是否为空
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	// 返回栈长度
	fn len(&self) -> usize {
		self.size
	}
	// 清空栈
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	// 往栈里加东西
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		if self.size > 0 {
			self.size -= 1;
		}
		self.data.pop()
	}
	// 看栈顶的数据
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	// 返回可修改的栈顶数据
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	// 获取栈的迭代器（获取所有权）
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	// 获取栈的迭代器（获取引用），相当于创建一个新的vec，把stack的里面的数据push进去
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		// self.data.iter()返回data的迭代器
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	// 获取栈的可变引用
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		// self.data.iter_mut()返回data的可变迭代器
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	let mut stack = Stack::new();
	// 遍历字符串里面的字符
	for chr in bracket.chars() {
		// 扫描到左括号就入栈
		if chr == '(' || chr == '{' || chr == '[' {
			stack.push(chr);
		} 
		if chr == ')' || chr == '}' || chr == ']' {
			if stack.is_empty() {
				return false;
			}
			if let Some(stack_top) = stack.pop() {
				if chr == ')' {
					if stack_top != '(' { return false; }
				} else if chr == '}' {
					if stack_top != '{' { return false; }
				} else if chr == ']' {
					if stack_top != '[' { return false; }
				}
			}
		}
	}
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}