use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

type TodoDB = BTreeMap<u16, String>;
thread_local! {
    static TODOMAP: RefCell<TodoDB> = RefCell::default();
    static GEN_ID: RefCell<u16> = RefCell::new(0);
}

/// Creates a new todo and returns its unique ID.
///
/// This update function adds a new todo with the provided content to the internal storage.
/// It assigns a unique identifier to the new todo and returns it.
///
/// # Parameters
///
/// * `todo_str` (String): The content of the new todo.
///
/// # Returns
///
/// A `Result<u16, String>`.
/// On success:  It returns the ID for the newly created todo.
/// On error: It return an error string. --> none as of now
///
#[update(name = "add")]
fn add_todo(todo_str: String) -> Result<u16, String> {
    let new_tid = GEN_ID.with(|tid| {
        let mut borrowed = tid.borrow_mut();
        let current_id = borrowed.clone();
        *borrowed = current_id + 1;
        borrowed.clone()
    });
    TODOMAP.with(|todomap| todomap.borrow_mut().insert(new_tid, todo_str));
    Ok(new_tid)
}

/// Reads the content of a specific todo by its ID.
///
/// This query function retrieves the content of a todo identified by the provided ID.
///
/// # Parameters
///
/// * `id` (u16): The unique identifier of the todo to be read.
///
/// # Returns
///
/// A `Result<String, String>`.
/// On success: It returns the content of the todo as a string.
/// On error: It returns an error string.
///
/// # Errors
///
/// This function can return an error string (`No todo with this ID`)
/// If the provided ID is invalid or the todo doesn't exist.
#[query(name = "read")]
fn read_todo(id: u16) -> Result<String, String> {
    TODOMAP.with(|todomap| match todomap.borrow().get(&id) {
        Some(todo_str) => Ok(todo_str.clone()),
        None => Err(format!("No todo with this ID {:?}", id)),
    })
}

/// Retrieves a paginated list of todos.
///
/// This query function retrieves a maximum of 10 todos per page.
///
/// # Parameters
///
/// * `page` (u16): The requested page number (starting from 1).
///
/// # Returns
///
/// A tuple containing:
///
/// * `Vec<String>`: An array of strings representing the todo content for the requested page.
/// * `Option<u8>`: An optional value indicating the next page number (if applicable).
///                  If there are no more todos beyond the current page, this will be `None`.
///
/// # Errors
///
/// This function can return an error string (`Invalid Page <page>`) in the following cases:
/// * Invalid `page` number.
/// * No todos found on the requested page.
#[query(name = "read_all")]
fn read_all_todos(mut page: u16) -> Result<(Vec<String>, Option<u16>), String> {
    TODOMAP.with(|todomap| {
        let todomap = todomap.borrow();
        let limit = 10;
        page = std::cmp::max(page, 1);

        let start_index = (page - 1) * limit;

        let todo_slice: Vec<_> = todomap
            .values()
            .skip(start_index as usize)
            .take(limit as usize)
            .cloned()
            .collect();

        if todo_slice.is_empty() {
            return Err(format!("Invalid Page {}", page).to_string());
        }

        let next_page = if todomap.len() as u16 > start_index + limit {
            Some(page + 1)
        } else {
            None
        };

        Ok((todo_slice, next_page))
    })
}

/// Updates the content of an existing todo.
///
/// This update function modifies the content of a todo identified by the provided ID with the new content.
///
/// # Parameters
///
/// * `id` (u16): The unique identifier of the todo to be updated.
/// * `new_todo_str` (String): The new content for the todo.
///
/// # Returns
///
/// A `Result<(), String>`.
/// On success: It returns an empty `Ok(())`.
/// On error: It returns an error  string.
///
/// # Errors
///
/// This function can return an error string (`No todo with this ID: <todo_id> found. Invalid operation`)
/// If the provided ID is invalid or the todo doesn't exist.
#[update(name = "update")]
fn update_todo(id: u16, new_todo_str: String) -> Result<(), String> {
    TODOMAP.with(|todomap| {
        let mut todomap = todomap.borrow_mut();
        match todomap.get(&id) {
            Some(_) => {
                todomap.insert(id, new_todo_str);
                Ok(())
            }
            None => {
                Err(format!("No todo with this ID: {:?} found. Invalid operation", id).to_string())
            }
        }
    })
}

/// Deletes a todo by its ID.
///
/// This update function removes a todo identified by the provided ID from the internal storage.
///
/// # Parameters
///
/// * `id` (u16): The unique identifier of the todo to be deleted.
///
/// # Returns
///
/// A `Result<(), String>`.
/// On success: it returns an empty `Ok(())`.
/// On error: it contains an error message string.
///
/// # Errors
///
/// This function can return an error string (`No todo with this ID: <todo_id> found.`)
/// If the provided ID is invalid or the todo doesn't exist.
#[update(name = "delete")]
fn delete_todo(id: u16) -> Result<(), String> {
    TODOMAP.with(|todomap| match todomap.borrow_mut().remove_entry(&id) {
        Some(_) => Ok(()),
        None => Err(format!("No todo with this ID: {:?} found.", id)),
    })
}

ic_cdk::export_candid!();
