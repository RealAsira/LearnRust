#[derive(Debug)]
#[allow(dead_code)]
enum StatesUS {
    Utah, Oregon, Washington
}

#[derive(Debug)]
#[allow(dead_code)] // removes dead code warning (e.g, dime, quarter, half, and dollar are never used but the warning is now gone)
enum Coin {
    Nickel, Dime, Quarter(StatesUS), Half, Dollar
}

impl Coin {
    fn value_cents(&self)->u8 {
        match self {
            // multiple lines to executing if match
            Coin::Nickel => {
            println!("Lucky nickel");
            5
            },
            // simpler return value or code execute
            Coin::Dime => 10,
            Coin::Half => 50,
            Coin::Dollar => 100,

            // 
            Coin::Quarter(state) => {
                println!("Quarter from {:?}", {state});
                25
            },
        }
    }
}


fn main() {
    let my_coin = Coin::Nickel;
    println!("Value: {:?}", my_coin.value_cents());

    let my_quarter = Coin::Quarter(StatesUS::Oregon);
    println!("Value: {:?}", my_quarter.value_cents());
}
