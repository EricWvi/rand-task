use rtdb::tasks::TaskStatus;
use rtdb::{task_dao, util, Task};
use sea_orm::DatabaseConnection;

pub async fn update_task(db: &DatabaseConnection, id: i32) {
    let task = match task_dao::find_task_by_id(db, id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };

    let name = util::get_dialog_answer("Name", &*task.name);
    println!("Name: {name}\n");
    let file_link = match util::get_dialog_answer(
        "File Link",
        &*task.file_link.as_ref().unwrap_or(&"null".to_string()),
    )
    .as_str()
    {
        "null" => None,
        s => Some(s.to_string()),
    };
    println!(
        "File Link: {}\n",
        file_link.as_ref().unwrap_or(&"null".to_string())
    );

    println!(
        "TaskStatus:  a.Unfinished  b.Completed  c.Discarded  [{:?}]",
        task.status
    );
    let status = match util::eval_choice(3, true) {
        'a' => TaskStatus::Unfinished,
        'b' => TaskStatus::Completed,
        'c' => TaskStatus::Discarded,
        '\n' => task.status,
        _ => unreachable!(),
    };
    let task_new = Task {
        id,
        name,
        file_link,
        project_id: task.project_id,
        status,
        seq: task.seq,
    };
    task_dao::update_task(db, &task, &task_new)
        .await
        .expect(&*format!("failed to update task[id={}]", task.id));
}
