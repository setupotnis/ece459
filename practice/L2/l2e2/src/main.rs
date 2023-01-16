struct User {
    // TODO put fields here
    name: String,
    lottomax: [u32; 7],
}

impl User {
    fn buy(&mut self) -> [u32; 7] {
        self.lottomax = [2, 0, 3, 4, 5, 6, 8];
        return self.lottomax;
    }
}

fn main() {
    let mut user = User {
        // put initial values of fields here
        name: String::from("John"),
        lottomax: [0; 7],
    };
    let ticket = user.buy();
    let result = check(ticket);
    println!("{}'s ticket is {}", user.name, result);
}

fn check(ticket: [u32; 7]) -> bool {
    let mut sum = 0;
    for i in 0..7 {
        sum += ticket[i];
    }

    if sum % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
