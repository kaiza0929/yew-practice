fn main() {

    /* 値を直接表示するので後述する{}は不要 */
    println!("hello");
    
    /* 不変変数(再代入不可)の宣言let i32は整数型 */
    let number: i32 = 1;
    /* 変数を表示するには変数の数だけ{}(文字列 複数の場合はカンマで区切る)が必要 今回は変数numberだけなので{}は1つ */
    println!("{}", number);
    
    /* mutを付けることで可変変数(再代入が可能)を宣言 */
    let mut number2: i32 = 2;
    number2 *= 10;
    /* 表示する変数はnumberとnumber2の2つなので{}は2つ必要 */
    println!("{}, {}", number, number2);
    
    /* constで定数を宣言 変数と違い大文字にする &strは文字列型 */
    const MESSAGE: &str = "goodbye";
    println!("{}", MESSAGE);

    /* 固定長配列 長さ5で文字列を要素に持つ */
    let array: [&str; 3] = ["イヌ", "サル", "キジ"];
    println!("{}", array[2]);

    /* 関数の呼び出し */
    fizz_buzz(50);

}

fn fizz_buzz(max_num: i32) {

    for i in 1..max_num + 1 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

}