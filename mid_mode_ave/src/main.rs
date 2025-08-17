use std::collections::HashMap;

fn sortNum(number_vec: &mut Vec<i32>) {
    //必须先对动态数组进行排序，选用最常规的冒泡排序
    let vec_len = number_vec.len(); // 快速获取动态数组长度
    for i in (1..vec_len) { // 1,2,3
        for j in (0..vec_len-i) { // 0-3; 0-2; 0-1
            if number_vec[j] > number_vec[j+1] {
                let temp: i32 = number_vec[j];
                number_vec[j] = number_vec[j+1];
                number_vec[j+1] = temp; 
            }
        }
    }
    println!{"Sorted List: {:?}",number_vec};
}

fn average(number_vec: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut sum = 0;

    // 不管是vec还是&vec, 在循环体中，rust都可以直接用，会有隐式的解引用
    for num in number_vec{
        count += 1;
        sum += num;
    }
    sum/count
}

fn midNum(number_vec: &Vec<i32>) -> i32 {
    let vec_len = number_vec.len(); // 快速获取动态数组长度

    if vec_len%2 == 0 {
        (number_vec[vec_len/2] + number_vec[(vec_len/2)-1])/2
    } else {
        number_vec[vec_len/2]
    }
}

fn modeNum(number_vec: &Vec<i32>) -> i32 {
    let mut num_dir = HashMap::new();

    // 这个地方暗含了，是否存在，不存在赋0，存在+1，无需进行手动if判断是否为空
    // rust 不会自动解引用迭代元素 因此number此时的类型是&i32， 需要自己手动解引用
    // for &number in number_vec 这样就不需要解引用了
    for number in number_vec {
        *num_dir.entry(*number).or_insert(0) += 1; //entry需要键的所有权，i32，而不是引用&i32
    }

    // 在此处可以发现，哈希映射在便利的过程中，以不定序的方式进行
    // 每次print的前后顺序不固定，也就是说要找出，出现次数最多的数字，要在一次之内完成
    // 下一次，对应的序列顺序就会改变
    println!("{:?}", num_dir);
    let mut key_store = 0;
    let mut max_count = 0;

    // 哈希表关键知识点，.entry 返回的结果是 Entry类型，不能直接当做value类型使用
    // 同样，此处也进行了模式匹配，自动解引用了哈希表里面的键值

    for (&key, &count) in &num_dir {
        if count > max_count {
            max_count = count;
            key_store = key;
        }
    }
    key_store

}


fn main(){
    let mut number_vec = vec![10, 5, 205, 15, 300, 5, 5, 15];
    //println!("{}",number_vec.len());
    sortNum(&mut number_vec);

    let average_val = average(&number_vec);
    println!("Average has been calculated {average_val}");
    let mid_num = midNum(&number_vec);
    println!("Mid number has been calculated {mid_num}");
    let mode_num = modeNum(&number_vec);
    println!("Mode number has been calculated {mode_num}");
}
