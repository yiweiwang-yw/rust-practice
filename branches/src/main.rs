fn main() {
    // let number = 7;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // let condition = true;
    // let number = if condition {5} else {6};
    //
    // println!("The number is: {number}");

    // simple loop example
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     };
    // };
    //
    // println!("The result is {result}");

    // loop with names
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    // println!("End count = {count}");

    // while simple example
    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{number}!");
    //
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF!!!");

    // while within a collection
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index += 1;
    // }

    /* above could panic if index is not updated. Below is a better implementation*/
    // for element in a {
    //     println!("the value is: {element}");
    // }

    // countdown in a for loop using rev
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");

    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            println!("{x}");
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }

}
