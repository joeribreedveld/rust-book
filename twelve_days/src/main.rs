const DAYS: usize = 12;

const GIFTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    for day in 0..DAYS {
        println!("On the {} day of Christmas,", ORDINALS[day]);
        println!("my true love gave to me");

        for gift_index in (0..=day).rev() {
            println!(
                "{}{}",
                GIFTS[gift_index],
                match (gift_index, day == DAYS - 1) {
                    (0, true) => "!",
                    (0, false) => ".",
                    _ => ",",
                }
            );
        }

        if day != DAYS - 1 {
            println!();
        }
    }
}
