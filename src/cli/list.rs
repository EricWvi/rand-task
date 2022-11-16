use crate::util;
use rtdb::projects::{ProjectStatus, ProjectType};
use rtdb::{project_dao, project_view};
use sea_orm::DatabaseConnection;

pub async fn list_projects(db: &DatabaseConnection, mut with_pending: bool) {
    println!(
        "Find projects by\n\
        📥 a.Inbox\n\
        💻 b.Today\n\
        💬 c.En\n\
        💪 d.Focus another thing\n\
        🎧 e.Take a break\n\
        🪫 f.Tired\n\
        ✅ g.Completed\n\
        📦 h.All"
    );
    let (projects, distri) = match util::eval_choice(8, false) {
        'g' => {
            let projects = project_dao::find_projects_by_status(db, ProjectStatus::Completed)
                .await
                .expect("failed to list completed projects from db");
            (projects, false)
        }
        'h' => {
            let projects = project_dao::find_all_projects(db)
                .await
                .expect("failed to list all projects from db");
            (projects, false)
        }
        c @ 'a'..='f' => {
            let project_type = match c {
                'a' => {
                    // list Inbox with pending projects by default
                    with_pending = true;
                    ProjectType::Inbox
                }
                'b' => ProjectType::Today,
                'c' => ProjectType::En,
                'd' => ProjectType::FocusAnotherThing,
                'e' => ProjectType::TakeABreak,
                'f' => ProjectType::Tired,
                _ => unreachable!(),
            };
            let projects =
                project_dao::find_projects_by_type(db, project_type, with_pending, false)
                    .await
                    .expect("failed to find projects by type from db");
            (projects, true)
        }
        _ => unreachable!(),
    };

    let mut views = projects
        .into_iter()
        .map(|t| project_view::ListView::from(t))
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
