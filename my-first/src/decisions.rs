pub fn some_decisions() {
    let mut temperature:i8 = 25;
    choose(temperature);

    temperature = 8;
    choose(temperature);

    temperature = 15;
    choose(temperature);
    choose_value(temperature);
}

fn choose(temp:i8) {
    if temp > 20 {
        println!("Temperature at {} is really hot", temp);
    }
    else if temp < 10 {
        println!("Temperature at {} is really cold", temp);
    }
    else {
        println!("Temperature at {} is ok", temp);
    }
}

fn choose_value(temp: i8) {
    let value = if temp > 20 { "Temperature is really hot" }
        else if temp < 10 { "Temperature is really cold" }
        else { "Is ok" };

    println!("{}", value);
}