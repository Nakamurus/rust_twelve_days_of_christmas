fn main() {
    let num = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let lines = [
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for index in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            num[index]
        );
        for element in lines[0..index].iter().rev() {
            println!("{}", element);
        }
        println!("A partridge in a pear tree");
        println!("\n");
    }
}
