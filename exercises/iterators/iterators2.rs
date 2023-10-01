// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars(); // 创建字符迭代器
    match c.next() {  // 获取迭代器的第一个字符
        None => String::new(),   // 如果没有字符，则返回空字符串
        Some(first) => {
            let capitalized = first.to_uppercase(); // 将第一个字符大写
            capitalized.to_string() + c.as_str() // 添加剩余字符并转换为字符串
        }
    }
}


// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: Vec<&str>) -> Vec<String> {
    words.iter() // 创建字符串切片的迭代器
        .map(|&word| capitalize_first(word)) //处理每个 word
        .collect::<Vec<String>>() // 收集处理后的结果并返回为向量
        // .collect() 不写类型也可以
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter() // 创建字符串切片的迭代器
        .map(|&word| capitalize_first(word)) // 使用 capitalize_first 函数处理每个单词
        .collect::<Vec<_>>() // 收集处理后的结果并返回为向量
        .join("")
        // .collect::<String>()
        // 后面的可以直接替换成 .map().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
