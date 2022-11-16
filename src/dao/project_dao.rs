use crate::entity::*;
use crate::projects::{ProjectStatus, ProjectType};
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, query::*, DbConn, DbErr};

pub async fn find_projects_by_type(
    db: &DbConn,
    project_type: ProjectType,
    with_pending: bool,
    with_completed: bool,
) -> Result<Vec<Project>, DbErr> {
    let select = Projects::find()
        .filter(projects::Column::Type.eq(project_type))
        .filter(projects::Column::Status.ne(ProjectStatus::Discarded));
    let select = if with_pending {
        select
    } else {
        select.filter(projects::Column::Status.ne(ProjectStatus::Pending))
    };
    let select = if with_completed {
        select
    } else {
        select.filter(projects::Column::Status.ne(ProjectStatus::Completed))
    };
    let projects: Vec<Project> = select.all(db).await?;
    Ok(projects)
}

pub async fn find_projects_by_status(
    db: &DbConn,
    project_status: ProjectStatus,
) -> Result<Vec<Project>, DbErr> {
    let projects: Vec<Project> = Projects::find()
        .filter(projects::Column::Status.eq(project_status))
        .all(db)
        .await?;
    Ok(projects)
}

pub async fn find_projects_by_name(db: &DbConn, name: &str) -> Result<Vec<Project>, DbErr> {
    let projects: Vec<Project> = Projects::find()
        .filter(projects::Column::Name.contains(name))
        .all(db)
        .await?;
    Ok(projects)
}

pub async fn find_projects_by_id(db: &DbConn, id: i32) -> Result<Project, DbErr> {
    let project: Option<Project> = Projects::find_by_id(id).one(db).await?;
    project.ok_or(DbErr::RecordNotFound(
        "project_id does not exist".to_string(),
    ))
}

pub async fn find_all_projects(db: &DbConn) -> Result<Vec<Project>, DbErr> {
    let projects: Vec<Project> = Projects::find().all(db).await?;
    Ok(projects)
}

pub async fn add_project(
    db: &DbConn,
    name: String,
    md_link: Option<String>,
    project_type: ProjectType,
    weight: i32,
) -> Result<Project, DbErr> {
    projects::ActiveModel {
        name: Set(name),
        md_link: Set(md_link),
        r#type: Set(project_type),
        weight: Set(weight),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn update_status(
    db: &DbConn,
    project: Project,
    status: ProjectStatus,
) -> Result<Project, DbErr> {
    let mut project: projects::ActiveModel = project.into();
    project.status = Set(status);
    project.update(db).await
}

pub async fn update_type(
    db: &DbConn,
    project: Project,
    project_type: ProjectType,
) -> Result<Project, DbErr> {
    let mut project: projects::ActiveModel = project.into();
    project.r#type = Set(project_type);
    project.update(db).await
}

pub async fn update_weight(db: &DbConn, project: &Project, weight: i32) -> Result<Project, DbErr> {
    let mut project: projects::ActiveModel = project.clone().into();
    project.weight = Set(weight);
    project.update(db).await
}

pub async fn update_project(db: &DbConn, old: &Project, new: &Project) -> Result<Project, DbErr> {
    if old == new {
        return Ok(new.clone());
    }

    let mut project: projects::ActiveModel = old.clone().into();
    if old.name != new.name {
        project.name = Set(new.name.clone());
    }
    if old.md_link != new.md_link {
        project.md_link = Set(new.md_link.clone());
    }
    if old.r#type != new.r#type {
        project.r#type = Set(new.r#type);
    }
    if old.weight != new.weight {
        project.weight = Set(new.weight);
    }
    if old.status != new.status {
        project.status = Set(new.status);
    }
    project.update(db).await
}

#[cfg(test)]
mod test {
    use crate::projects::{ProjectStatus, ProjectType};
    use sea_orm::DatabaseConnection;

    async fn db() -> &'static DatabaseConnection {
        crate::DB.get_or_init(crate::init).await
    }

    #[tokio::test]
    async fn test_find_projects_by_name() {
        let projects = super::find_projects_by_name(db().await, "人")
            .await
            .unwrap();
        dbg!(projects);
    }

    #[tokio::test]
    async fn test_add_project() {
        let project = super::add_project(
            db().await,
            "test".to_string(),
            None,
            ProjectType::FocusAnotherThing,
            1,
        )
        .await
        .unwrap();
        dbg!(project);
    }
}
