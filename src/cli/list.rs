use crate::util;
use rtdb::tasks::{TaskStatus, TaskType};
use rtdb::{task_dao, task_view};
use sea_orm::DatabaseConnection;

pub async fn list_tasks(db: &DatabaseConnection, mut with_pending: bool) {
    println!(
        "Find tasks by\n\
        📥 a.Inbox\n\
        💻 b.Today\n\
        💬 c.En\n\
        💪 d.Focus another thing\n\
        🎧 e.Take a break\n\
        🪫 f.Tired\n\
        ✅ g.Completed\n\
        📦 h.All"
    );
    let (tasks, distri) = match util::eval_choice(8, false) {
        'g' => {
            let tasks = task_dao::find_tasks_by_status(db, TaskStatus::Completed)
                .await
                .expect("failed to list completed tasks from db");
            (tasks, false)
        }
        'h' => {
            let tasks = task_dao::find_all_tasks(db)
                .await
                .expect("failed to list all tasks from db");
            (tasks, false)
        }
        c @ 'a'..='f' => {
            let task_type = match c {
                'a' => {
                    // list Inbox with pending tasks by default
                    with_pending = true;
                    TaskType::Inbox
                }
                'b' => TaskType::Today,
                'c' => TaskType::En,
                'd' => TaskType::FocusAnotherThing,
                'e' => TaskType::TakeABreak,
                'f' => TaskType::Tired,
                _ => unreachable!(),
            };
            let tasks = task_dao::find_tasks_by_type(db, task_type, with_pending, false)
                .await
                .expect("failed to find tasks by type from db");
            (tasks, true)
        }
        _ => unreachable!(),
    };

    let mut views = tasks
        .into_iter()
        .map(|t| task_view::ListView::from(t))
        .collect::<Vec<_>>();
    if distri {
        views.sort_by(|a, b| {
            if a.status != b.status {
                b.status.cmp(&a.status)
            } else {
                b.weight.cmp(&a.weight)
            }
        });
    }
    let all_weight = views.iter().map(|v| v.weight).sum::<i32>();
    if distri && !with_pending {
        for view in views {
            println!(
                "{:6.2}%  {view}",
                (view.weight * 100) as f64 / all_weight as f64
            );
        }
    } else {
        for view in views {
            println!(" {view}",);
        }
    }
}
