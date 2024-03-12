use std::collections::HashMap;

pub struct Input {
    index: u32,                         // 索引
    chars: Vec<char>,                   // 键盘输入的字符会存储进来, 直到回车后销毁
    pub contents: HashMap<u32, String>, // 输入的所有的命令存储
}

impl Input {
    pub fn new() -> Self {
        Self {
            index: 0,
            chars: vec![],
            contents: HashMap::new(),
        }
    }

    /// 将每次输入的字符添加contents保存
    pub fn insert(&mut self) -> Option<()> {
        self.contents
            .insert(self.index, String::from_iter(&self.chars));
        self.index += 1;
        self.chars.clear();
        Some(())
    }

    /// 获取输入中的的字符串(即将所有的字符切片转换为字符串)
    pub fn get_chars_to_string(&self) -> String {
        String::from_iter(&self.chars)
    }
    /// 获取输入完的的字符串(即将所有的字符切片转换为字符串)
    pub fn get_to_string(&self) -> Option<&String> {
        self.contents.get(&(self.index - 1))
    }
    /// 获取输入的命令的条数
    pub fn get_index(&self) -> u32 {
        self.index
    }
    ///添加输入的字符
    pub fn push_char(&mut self, c: char) {
        self.chars.push(c)
    }
    ///删除输入的最后一个字符, 不管是否删除成功
    pub fn pop_char(&mut self) {
        match self.chars.pop() {
            Some(_) => (),
            None => (),
        }
    }
    ///删除输入的指定索引的字符
    pub fn remove_char(&mut self, index: usize) -> char {
        self.chars.remove(index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_pop() {
        let mut v = Vec::<String>::new();
        // v.pop().expect("this error");
        v.pop();
    }

    #[test]
    fn test_vec_to_string() {
        let v = Vec::<String>::new();
        let _s = String::from_iter(v);
    }
}
