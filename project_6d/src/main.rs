#[derive(Debug)]
enum StatesUS {
    Utah, Oregon, Washington
}

impl StatesUS {
    // tests if a state existed in the US yet
    fn existed_in(&self, year:u16) -> bool {
        match self {    
            // >= <= are operators, => is a match return
            StatesUS::Utah => year >= 1896,
            StatesUS::Oregon => year >= 1859,
            StatesUS::Washington => year >= 1889,
            _ => false,
        }
    }
}



enum Coin {
    Nickel, Dime, Quarter(StatesUS), Half, Dollar
}

impl Coin {
    fn description(&self) -> Option<String> {
        let Coin::Quarter(state) = self else {
            return Some(format!("Not a quarter"));
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is old!"))
        } else {
            Some(format!("{state:?} is newer than 1900!"))
        }
    }
}



fn main() {
    let my_coin = Coin::Quarter(StatesUS::Oregon);
    println!("{:?}", my_coin.description());


    // this code ...
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("configured to {max}"),
        _ => ()
    }

    // ... is equivalent to this code
    if let Some(max) = config_max {
        println!("configured to {max}");
    } 
    
    // can add an else statement too, to expand
    else {
        println!("no config max set");
    }
}
