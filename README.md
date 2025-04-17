# Todo CLI app

A CLI to-do app written in Rust that uses json file for storage.

## List of Commands
| Implemented | Command | Description |
|-------------|---------|-------------|
| [ ] | `todo list` | lists all the incomplete tasks (should be numbered) |
| [ ] | `todo list --done` | lists all the completed tasks |
| [ ] | `todo list --all` | lists all the completed tasks |
| [ ] | `todo list --category {_category_}` | lists all the tasks of the category \_category\_ |
| [ ] | `todo add {_task_}` | adds \_task\_ to the tasks |
| [ ] | `todo add --category {_category_} {_task_}` | adds \_task\_ to the tasks with category \_category\_ |
| [ ] | `todo did {_task_id_}` | mark a task with number \_number\_ as completed |
| [ ] | `todo undid {_task_id_}` | mark a task with number \_number\_ as not completed |
| [ ] | `todo update --category {new_category} {_task_id_}` | updates the category of existing task |
| [ ] | `todo delete {_task_id_}` | deletes the task with task id \_task\_id\_ |
 
__Example of todo lists__ <br>
| __ID__ | __Name__ | __category__ |
| -------| ---------| --------------|
| 5 | Finish Todo App | side-project |
