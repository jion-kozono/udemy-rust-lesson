pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 10_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    // システムのサイズ
    println!("{}", usize::BITS);

    // メモリのアドレスを表示
    println!("Memory address of const is {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is {:p}", &i2);
    println!("Stack address of i3 is {:p}", &i3);

    // シャドウイング
    let y = 5;
    println!("Stack address of y is {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is {:p}", &y);
    println!("The value of y is {}", y);
    {
        let y = 0;
        println!("The value of y is {}", y);
    }
    println!("The value of y is {}", y);

    // タプル
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);
    // こっちではダメなのか？
    // t2.0 .0 = 5;
    // t2.0 .1 = -5;

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライスとstr型
    let s1 = "helloこんにちは挨拶"; //26bite 英語一文字1bite, 日本語一文字3bite
    let s2 = "hello";
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Heap memory address of s1 is {:?}", s1.as_ptr());
    println!("Heap memory address of s2 is {:?}", s2.as_ptr());
    println!("len of s1 is {}", s1.len());
    println!("len of s2 is {}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is {:p}", &s1);
    println!("Stack address of s2 is {:p}", &s2);
    println!("Heap memory address of s1 is {:?}", s1.as_ptr());
    println!("Heap memory address of s2 is {:?}", s2.as_ptr());
    println!("len of s1 is {}", s1.len());
    println!("len of s2 is {}", s2.len());
    println!("Capacity of s1 is {}", s1.capacity());
    println!("Capacity of s2 is {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);
}
