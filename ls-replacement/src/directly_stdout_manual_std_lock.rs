use std::io::Write;
use std::{
    os::unix::prelude::{MetadataExt, PermissionsExt},
    path::Path,
};

const COLUMNS: [&str; 6] = ["Permissions", "Owner", "Group", "Size", "Modified", "Name"];
const COLUMN_SEPARATOR: &str = " | ";
const ROW_SEPARATOR: &str = "-";
const PADDING_X: usize = 0;
const COLUMN_FIXED_WIDTH: usize = 12;

fn print_horizontal_centered(text: &str, _max_width: usize) -> String {
    let max_width = _max_width + PADDING_X * 2;
    let mut output = String::from("");
    //Align center
    let col_width = match text.len() {
        len if len > max_width => max_width,
        len => len,
    };
    let padding_left = (max_width - col_width) / 2;
    let padding_right = max_width - col_width - padding_left;
    for _ in 0..padding_left {
        output.push_str(" ");
    }
    output.push_str(text);
    for _ in 0..padding_right {
        output.push_str(" ");
    }
    output.push_str(COLUMN_SEPARATOR);
    output
}
fn print_left_aligned(text: &str, _max_width: usize, lock: &mut std::io::StdoutLock<'_>) {
    // Align to left
    let max_width = _max_width + PADDING_X * 2;
    let col_width = match text.len() {
        len if len > max_width => max_width,
        len => len,
    };
    let padding_right = max_width - col_width;
    write!(lock, "{}", text);
    for _ in 0..padding_right {
        write!(lock, " ");
    }
    write!(lock, "{}", COLUMN_SEPARATOR);
}
pub fn directly_stdout_manual_stdout_lock() {
    let stdout = std::io::stdout();
    let mut lock: std::io::StdoutLock<'_> = stdout.lock();

    // Args
    let args = std::env::args().collect::<Vec<String>>();
    // #[cfg(debug_assertions)]
    // println!("Args: {:?}", args);

    // Path argument
    let mut path = match args.get(1) {
        Some(path) => std::path::PathBuf::from(path),
        _ => std::env::current_dir().unwrap(),
    };
    if !path.exists() {
        path = Path::new("./testings").to_path_buf();
    }

    // #[cfg(debug_assertions)]
    // println!("Path: {:?}", path);

    let files_intent = std::fs::read_dir(&path);
    if files_intent.is_err() {
        println!("Path: {:?}", path);
        println!("Error: {:?}", files_intent.err());
        return;
    }
    // Error handling
    let error = files_intent.as_ref().err();
    if error.is_some() {
        println!("Error: {:?}", error);
        return;
    }

    // Printing
    let files = files_intent.unwrap();

    let columns_max_width = [
        COLUMNS[0].len(),
        COLUMNS[1].len(),
        COLUMNS[2].len(),
        6,
        20,
        20,
    ];
    for (i, col) in COLUMNS.iter().enumerate() {
        // Align left
        let padding_right = match (columns_max_width[i]) as isize - (col.len()) as isize {
            len if len > 0 => len as usize,
            _ => 0,
        };
        write!(&mut lock, "{}", col);
        for _ in 0..padding_right + PADDING_X * 2 {
            write!(&mut lock, " ");
        }
        write!(&mut lock, "{}", COLUMN_SEPARATOR);
    }

    writeln!(&mut lock);
    for col in columns_max_width {
        for _ in 0..col + PADDING_X * 2 {
            write!(&mut lock, "{}", ROW_SEPARATOR);
        }
        write!(&mut lock, "{}", COLUMN_SEPARATOR);
    }
    writeln!(&mut lock);

    for wrapped_file in files {
        let file = wrapped_file.unwrap();
        let name = file.file_name().into_string().unwrap();
        let meta = file.metadata().unwrap();
        // Octal number
        let permissions_bytes = u16::from_str_radix(
            format!("{:o}", meta.permissions().mode() & 0o777).as_str(),
            10,
        )
        .unwrap();
        // #[cfg(debug_assertions)]
        // println!(
        //     "{} -> {} = {}",
        //     format!("{:o}", meta.permissions().mode() & 0o777).as_str(),
        //     format!("{:o}", meta.permissions().mode() & 0o777).as_str(),
        //     permissions_bytes
        // );
        let calculate_user_permissions = match permissions_bytes / 100 {
            0 => "---",
            1 => "--x",
            2 => "-w-",
            3 => "-wx",
            4 => "r--",
            5 => "r-x",
            6 => "rw-",
            7 => "rwx",
            _ => "???",
        };
        let calculate_group_permissions = match permissions_bytes / 10 % 10 {
            0 => "---",
            1 => "--x",
            2 => "-w-",
            3 => "-wx",
            4 => "r--",
            5 => "r-x",
            6 => "rw-",
            7 => "rwx",
            _ => "???",
        };
        let calculate_other_permissions = match permissions_bytes % 10 {
            0 => "---",
            1 => "--x",
            2 => "-w-",
            3 => "-wx",
            4 => "r--",
            5 => "r-x",
            6 => "rw-",
            7 => "rwx",
            _ => "???",
        };

        let permissions = format!(
            "{}{}{}",
            calculate_user_permissions, calculate_group_permissions, calculate_other_permissions
        );
        let user_id = meta.uid();
        let group_id = meta.gid();
        let size = meta.size();
        let humanized_size = {
            let mut size = size;
            let mut unit = 0;
            while size > 1024 {
                size /= 1024;
                unit += 1;
            }
            format!("{} {}", size, ["B", "KB", "MB", "GB", "TB"][unit])
        };
        let modified = meta.modified().unwrap();
        let modified_datetime =
            chrono::DateTime::<chrono::Local>::from(modified).format("%Y-%m-%d %H:%M:%S");

        print_left_aligned(&permissions, columns_max_width[0], &mut lock);
        print_left_aligned(&user_id.to_string(), columns_max_width[1], &mut lock);
        print_left_aligned(&group_id.to_string(), columns_max_width[2], &mut lock);
        print_left_aligned(&humanized_size, columns_max_width[3], &mut lock);

        print_left_aligned(
            &modified_datetime.to_string(),
            columns_max_width[4],
            &mut lock,
        );
        print_left_aligned(&name, columns_max_width[5], &mut lock);
        writeln!(&mut lock, "");
    }
    // print lock
}
