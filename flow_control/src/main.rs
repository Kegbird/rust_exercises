
fn main() {
    let test = true;

    if test
    {
        println!("Test is true");
    }
    else
    {
        println!("Test is false");
    }
    
    let number = if test { 5 } else { 6 };
    println!("Number is {number}.");

    let mut i = 0;
    loop
    {
        println!("Iteration {i}");
        if i==number
        {
            break;
        }
        i=i+1;
    }

    println!("Loop with return value");
    i = 0;
    let j = loop
    {
        println!("Iteration {i}");
        if i==number
        {
            break i;
        }
        i=i+1;
    };
    println!("j is now {j}");

    let array = [10, 20, 30, 40, 50];

    println!("The array contains:");
    for element in array 
    {
        println!("- {element}");
    }

    println!("Counting to 4:");
    for k in 1..=4
    {
        println!("{k}");
    }
}
