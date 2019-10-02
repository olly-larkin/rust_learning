pub fn verse(n: i32) -> String {
    if n > 2 {
        format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1)
    } else if n > 1 {
        "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else if n > 0 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for i in (end..start+1).rev() {
        song.push_str(verse(i).as_str());
        if i != end {
            song.push('\n');
        }
    }
    song
}
