fn main () {
    //布尔型
    // let x: bool = true;
    // let y: bool = false;
    //     println!("{}", x);
    //     println!("{}", y);

    //char  Unicode 字符
    // let x = 'x';
    // let two_hearts = '💕';
    //     println!("{}", x);
    //     println!("{}", two_hearts);

    //数字类型 , 很复杂 , 有空慢慢嚼
    // let x = 42; // `x` has type `i32`.
    // let y = 1.0; // `y` has type `f64`.
    //     println!("{}", x);
    //     println!("{}", y);

    //数组
    // let a = [1, 2, 3]; // a: [i32; 3]
    // let mut m = [1, 2, 3]; // m: [i32; 3]
        //a的每一个值都初始化为0
        // let a = [0; 20]; // a: [i32; 20]
        // println!("a has {} elements", a.len());
        // let a = [1, 2, 3];
        // //跟js一毛一样
        // let names = ["Graydon", "Brian", "Niko"];
        // println!("The second name is: {}", names[1]);// names: [&str; 3]

    //切片
    // let a = [0, 1, 2, 3, 4];
    // let complete = &a[..]; // A slice containing all of the elements in `a`.
    // let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.

    //元组（Tuples）
    let x = (1, "hello");
    let tuple : (i32, &str) = (1, "hello");
    let x = tuple.0;
    let y = tuple.1;

        println!("x is {}", x);

}