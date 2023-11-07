const COLUMNS: [&str; 7] = [
    "Name",
    "Size",
    "Type",
    "Created",
    "Modified",
    "Accessed",
    "Permissions",
];
const COLUMN_SEPARATOR: &str = " | ";
const ROW_SEPARATOR: &str = "-";
const COLUMN_FIXED_WIDTH: usize = 10;

fn print_horizontal_centered(text: &str, max_width: usize) -> () {
    //Align center
    let col_width = match text.len() {
        len if len > max_width => max_width,
        len => len,
    };
    let padding_left = (max_width - col_width) / 2;
    let padding_right = max_width - col_width - padding_left;
    for _ in 0..padding_left {
        print!(" ");
    }
    print!("{}", text);
    for _ in 0..padding_right {
        print!(" ");
    }
    print!("{}", COLUMN_SEPARATOR);
}
fn main() {
    // Args
    let args = std::env::args().collect::<Vec<String>>();
    println!("Args: {:?}", args);
    if args.len() < 2 {
        println!("Please provide a path");
        return;
    }

    // Path argument
    let path = std::path::Path::new(&args[1]);
    println!("Path: {:?}", path);

    let files_intent = std::fs::read_dir(path);
    // Error handling
    let error = files_intent.as_ref().err();
    if error.is_some() {
        println!("Error: {:?}", error);
        return;
    }

    // Printing
    let files = files_intent.unwrap();

    let columns_max_width = [COLUMN_FIXED_WIDTH; COLUMNS.len()];
    println!("{:?}", columns_max_width);
    for col in COLUMNS {
        // Align left
        // print!("{:width$}", col, width = columns_max_width[1]);
        // print!("{}", COLUMN_SEPARATOR);
        print_horizontal_centered(col, columns_max_width[1]);
    }
    println!("");
    for col in columns_max_width {
        for _ in 0..col {
            print!("{}", ROW_SEPARATOR);
        }
        print!("{}", COLUMN_SEPARATOR);
    }
    println!("");

    for wrapped_file in files {
        let file = wrapped_file.unwrap();
        let name = file.file_name().into_string().unwrap();
        let meta = file.metadata().unwrap();
        print_horizontal_centered(name.as_str(), COLUMN_FIXED_WIDTH)
    }
}
