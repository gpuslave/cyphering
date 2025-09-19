use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

// Represents the permissions for a user on an object.
#[derive(Clone, Copy, Debug)]
struct Permissions {
    read: bool,
    write: bool,
    execute: bool,
    owner: bool,
}

impl Permissions {
    // Create a new Permissions struct
    fn new(read: bool, write: bool, execute: bool, owner: bool) -> Self {
        Self { read, write, execute, owner }
    }
}

// Implement Display to print permissions in "rwxo" format for debugging or other formats.
impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}",
            if self.read { 'r' } else { '-' },
            if self.write { 'w' } else { '-' },
            if self.execute { 'x' } else { '-' },
            if self.owner { 'o' } else { '-' }
        )
    }
}

/// Helper function to get trimmed user input.
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

/// Formats the permissions into a human-readable string.
fn format_rights(perms: &Permissions) -> String {
    if perms.owner {
        return "Полные права".to_string();
    }
    let mut rights = Vec::new();
    if perms.read {
        rights.push("Чтение");
    }
    if perms.write {
        rights.push("Запись");
    }
    if rights.is_empty() {
        "Запрет".to_string()
    } else {
        rights.join(", ")
    }
}

/// Displays the permissions for the current user.
fn display_permissions(
    user_index: usize,
    users: &[&str],
    files: &[String],
    access_matrix: &HashMap<(usize, usize), Permissions>,
) {
    println!("User: {}", users[user_index]);
    println!("Идентификация прошла успешно, добро пожаловать в систему");
    println!("Перечень Ваших прав:");
    for (i, file) in files.iter().enumerate() {
        let perms = access_matrix
            .get(&(user_index, i))
            .cloned()
            .unwrap_or(Permissions::new(false, false, false, false));
        // The object name is derived from the file name for simplicity.
        println!("Объект{}:       {}", i + 1, format_rights(&perms));
    }
}

/// Handles read and write operations.
fn handle_read_write(
    operation: &str,
    user_index: usize,
    files: &[String],
    access_matrix: &HashMap<(usize, usize), Permissions>,
) {
    let obj_input = get_user_input("Над каким объектом производится операция? ");
    match obj_input.parse::<usize>() {
        Ok(obj_num) if obj_num > 0 && obj_num <= files.len() => {
            let file_index = obj_num - 1;
            let perms = access_matrix
                .get(&(user_index, file_index))
                .cloned()
                .unwrap_or(Permissions::new(false, false, false, false));

            let has_permission = match operation {
                "read" => perms.read,
                "write" => perms.write,
                _ => false,
            };

            if has_permission {
                println!("Операция прошла успешно");
            } else {
                println!("Отказ в выполнении операции. У Вас нет прав для ее осуществления");
            }
        }
        _ => println!("Неверный номер объекта."),
    }
}

/// Handles the grant operation.
fn handle_grant(
    current_user_index: usize,
    users: &[&str],
    files: &[String],
    access_matrix: &mut HashMap<(usize, usize), Permissions>,
) {
    let obj_input = get_user_input("Право на какой объект передается? ");
    let file_index = match obj_input.parse::<usize>() {
        Ok(num) if num > 0 && num <= files.len() => num - 1,
        _ => {
            println!("Неверный номер объекта.");
            return;
        }
    };

    let owner_perms = access_matrix
        .get(&(current_user_index, file_index))
        .cloned()
        .unwrap_or(Permissions::new(false, false, false, false));

    if !owner_perms.owner {
        println!("Отказ в выполнении операции. У Вас нет прав для ее осуществления");
        return;
    }

    let right_to_grant = get_user_input("Какое право передается? ");
    let target_user_name = get_user_input("Какому пользователю передается право? ");

    if let Some(target_user_index) = users.iter().position(|&u| u.eq_ignore_ascii_case(&target_user_name)) {
        let target_perms = access_matrix
            .entry((target_user_index, file_index))
            .or_insert(Permissions::new(false, false, false, false));

        match right_to_grant.as_str() {
            "read" => target_perms.read = true,
            "write" => target_perms.write = true,
            "execute" => target_perms.execute = true,
            _ => {
                println!("Неизвестное право.");
                return;
            }
        }
        println!("Операция прошла успешно");
    } else {
        println!("Пользователь '{}' не найден.", target_user_name);
    }
}


fn main() {
    let users: Vec<&str> = vec!["Admin", "Boris", "Ivan"];
    let files: Vec<String> = (1..=4).map(|i| format!("Объект{}", i)).collect();

    let mut access_matrix: HashMap<(usize, usize), Permissions> = HashMap::new();

    // --- Populate the access matrix with some initial permissions ---
    // Admin (user 0) has full control over all files.
    for i in 0..files.len() {
        access_matrix.insert((0, i), Permissions::new(true, true, true, true));
    }
    // Boris (user 1)
    access_matrix.insert((1, 0), Permissions::new(true, false, false, false)); // Объект1: Чтение
    // Объект2: Запрет (no entry)
    access_matrix.insert((1, 2), Permissions::new(true, true, false, false));  // Объект3: Чтение, Запись
    access_matrix.insert((1, 3), Permissions::new(true, true, true, true));   // Объект4: Полные права

    // Ivan (user 2) has no permissions initially.

    // Main application loop
    loop {
        let user_input = get_user_input("Введите идентификатор пользователя: ");
        
        if let Some(user_index) = users.iter().position(|&u| u.eq_ignore_ascii_case(&user_input)) {
            display_permissions(user_index, &users, &files, &access_matrix);

            // Command loop for the logged-in user
            loop {
                let command = get_user_input("Жду ваших указаний > ");
                match command.as_str() {
                    "read" | "write" => {
                        handle_read_write(&command, user_index, &files, &access_matrix);
                    }
                    "grant" => {
                        handle_grant(user_index, &users, &files, &mut access_matrix);
                    }
                    "quit" => {
                        println!("Работа пользователя {} завершена. До свидания.", users[user_index]);
                        break;
                    }
                    "table" => {
                        // println!("Access Control Matrix:");
                        println!("{:<10} | {:<20} | Permissions", "User", "File");
                        println!("{:-<11}|{:-<22}|{:-<12}", "", "", "");

                        for (user_index, user) in users.iter().enumerate() {
                            for (file_index, file) in files.iter().enumerate() {
                                let perms = access_matrix.get(&(user_index, file_index))
                                    .cloned()
                                    .unwrap_or(Permissions::new(false, false, false, false)); // Default to no permissions
                                println!("{:<10} | {:<20} | {}", user, file, perms);
                            }
                            if user_index < users.len() - 1 {
                                println!("{:-<11}|{:-<22}|{:-<12}", "", "", "");
                            }
                        }
                        // break;
                    }
                    _ => {
                        println!("Неизвестная команда.");
                    }
                }
            }
        } else {
            println!("Неуспешная идентификация. Пользователь не найден.");
        }
    }
}