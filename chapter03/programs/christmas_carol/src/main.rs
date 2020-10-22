fn main() {
    println!("Lyrics to the Christmas carol “The Twelve Days of Christmas”:\n");
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    let count = [
        "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    let pos = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for (i, _) in pos.iter().enumerate() {
        println!("On the {} day of Christmas,\nMy true love sent me", pos[i]);
        for j in (0..i + 1).rev() {
            print!("{} {}", count[j], gifts[j]);
            if j == 0 {
                println!(".");
            } else {
                println!(",");
            }
        }
        println!();
    }
}
