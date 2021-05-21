/* 型列挙 */
pub enum Foods {
    Dogfood {size: i32}, /* sizeというメンバを持つDogfood型構造体 */
    Catfood {size: i32} /* sizeというメンバを持つCatfood型構造体 */
}

/* 構造体 */
pub struct Dog {
    name: String
}

/* トレイト */
pub trait Action {
    fn speak(&self);
    fn eat(&self, food: Foods);
}

/* 構造体 + トレイト = クラスのイメージ */

impl Action for Dog {

    fn speak(&self) {
        println!("ぼくの名前は{}だワン", self.name);
    }

    fn eat(&self, food: Foods) {
        match food {
            Foods::Dogfood { size } => {
                if size <= 140 {
                    println!("おいしい");
                } else {
                    println!("おおすぎ");
                }
            },
            /* 量に関係なくキャッフード(型)ならまずい 変数:_でその変数を使わないことを示す */
            Foods::Catfood { size:_ } => {
                println!("まずい");
            }
        }
    }

}

fn main() {
    let dog = Dog {name: "ポチ".to_string()};
    dog.speak();
    dog.eat(Foods::Dogfood {size: 140});
    dog.eat(Foods::Dogfood {size: 180});
    dog.eat(Foods::Catfood {size: 140});
}