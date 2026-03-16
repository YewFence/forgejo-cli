use clap::{Args, Subcommand};
use eyre::OptionExt;
use forgejo_api::structs::{
    CreateMilestoneOption, EditMilestoneOption, IssueGetMilestonesListQuery, Milestone,
};
use forgejo_api::Forgejo;
use futures::{future, TryStreamExt};

use crate::{
    keys::KeyInfo,
    repo::{RepoArg, RepoInfo, RepoName},
};

#[derive(Args, Clone, Debug)]
pub struct MilestoneCommand {
    /// The local git remote that points to the repo to operate on
    #[clap(long, short = 'R', global = true)]
    remote: Option<String>,
    /// The name of the repository to operate on
    #[clap(long, short, global = true)]
    repo: Option<RepoArg>,
    #[clap(subcommand)]
    command: MilestoneSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum MilestoneSubcommand {
    /// List milestones on a repo
    List {
        /// Filter by state: open, closed, all. Default: open
        #[clap(long, short, default_value = "open")]
        state: String,
    },
    /// View a milestone's details
    View {
        /// Milestone title or numeric ID
        name: String,
    },
    /// Create a new milestone
    Create {
        /// Title of the milestone
        title: String,
        /// Description of the milestone
        #[clap(long, short)]
        body: Option<Option<String>>,
        /// Due date (RFC 3339, e.g. 2025-06-01T00:00:00Z)
        #[clap(long, short)]
        due: Option<String>,
    },
    /// Edit an existing milestone
    Edit {
        /// Milestone title or numeric ID
        name: String,
        /// New title
        #[clap(long, short)]
        title: Option<String>,
        /// New description
        #[clap(long, short)]
        body: Option<String>,
        /// New due date (RFC 3339, e.g. 2025-06-01T00:00:00Z)
        #[clap(long, short)]
        due: Option<String>,
        /// New state: open or closed
        #[clap(long, short)]
        state: Option<String>,
    },
    /// Delete a milestone
    Delete {
        /// Milestone title or numeric ID
        name: String,
    },
}

impl MilestoneCommand {
    pub async fn run(self, keys: &mut KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo = RepoInfo::get_current(
            host_name,
            self.repo.as_ref(),
            self.remote.as_deref(),
            keys,
        )?;
        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo
            .name()
            .ok_or_eyre("couldn't get repo name, try specifying with --repo")?;
        match self.command {
            MilestoneSubcommand::List { state } => list_milestones(repo, &api, state).await?,
            MilestoneSubcommand::View { name } => view_milestone(repo, &api, &name).await?,
            MilestoneSubcommand::Create { title, body, due } => {
                create_milestone(repo, &api, title, body, due).await?
            }
            MilestoneSubcommand::Edit {
                name,
                title,
                body,
                due,
                state,
            } => edit_milestone(repo, &api, &name, title, body, due, state).await?,
            MilestoneSubcommand::Delete { name } => delete_milestone(repo, &api, &name).await?,
        }
        Ok(())
    }
}

/// Resolve a milestone name or numeric ID to the full Milestone object.
pub async fn find_milestone(
    api: &Forgejo,
    repo: &RepoName,
    name_or_id: &str,
) -> eyre::Result<Milestone> {
    // Try numeric ID first
    if let Ok(id) = name_or_id.parse::<i64>() {
        if let Ok(ms) = api
            .issue_get_milestone(repo.owner(), repo.name(), id)
            .await
        {
            return Ok(ms);
        }
    }

    // Fall back to name search (server-side filter, exact match verified client-side)
    let query = IssueGetMilestonesListQuery {
        state: Some("all".to_string()),
        name: Some(name_or_id.to_string()),
    };
    api.issue_get_milestones_list(repo.owner(), repo.name(), query)
        .stream()
        .try_filter(|ms| {
            future::ready(
                ms.title
                    .as_deref()
                    .is_some_and(|t| t == name_or_id),
            )
        })
        .try_next()
        .await?
        .ok_or_else(|| eyre::eyre!("milestone '{}' not found", name_or_id))
}

fn parse_due_date(s: &str) -> eyre::Result<time::OffsetDateTime> {
    time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
        .map_err(|e| eyre::eyre!("invalid date '{}': {}", s, e))
}

async fn list_milestones(
    repo: &RepoName,
    api: &Forgejo,
    state: String,
) -> eyre::Result<()> {
    let query = IssueGetMilestonesListQuery {
        state: Some(state),
        name: None,
    };
    let milestones = api
        .issue_get_milestones_list(repo.owner(), repo.name(), query)
        .all()
        .await?;

    crate::output::print_list(
        &milestones,
        &["TITLE", "STATE", "ISSUES", "DUE"],
        |ms| {
            let title = ms.title.as_deref().unwrap_or("?").to_string();
            let state = ms
                .state
                .as_ref()
                .map(crate::output::colored_state)
                .unwrap_or_default();
            let open = ms.open_issues.unwrap_or(0);
            let closed = ms.closed_issues.unwrap_or(0);
            let issues = format!("{open} open, {closed} closed");
            let due = ms
                .due_on
                .as_ref()
                .map(|d| {
                    d.format(&time::macros::format_description!("[year]-[month]-[day]"))
                        .unwrap_or_else(|_| "?".to_string())
                })
                .unwrap_or_default();
            vec![title, state, issues, due]
        },
    );
    Ok(())
}

async fn view_milestone(
    repo: &RepoName,
    api: &Forgejo,
    name_or_id: &str,
) -> eyre::Result<()> {
    let ms = find_milestone(api, repo, name_or_id).await?;

    crate::output::print_or_json(&ms, || {
        let crate::SpecialRender {
            bold,
            yellow,
            bright_green,
            bright_red,
            dark_grey,
            reset,
            dash,
            ..
        } = crate::special_render();

        let title = ms.title.as_deref().unwrap_or("?");
        let open = ms.open_issues.unwrap_or(0);
        let closed = ms.closed_issues.unwrap_or(0);
        let total = open + closed;
        let progress = if total > 0 {
            format!("{}%", closed * 100 / total)
        } else {
            "no issues".to_string()
        };

        let state_str = match ms.state.as_ref() {
            Some(forgejo_api::structs::StateType::Open) => {
                format!("{bright_green}open{reset}")
            }
            Some(forgejo_api::structs::StateType::Closed) => {
                format!("{bright_red}closed{reset}")
            }
            None => "?".to_string(),
        };

        println!("{yellow}{title}{reset} {dash} {state_str}");
        println!(
            "{open} open, {closed} closed {dark_grey}({progress}){reset}"
        );

        if let Some(due) = ms.due_on.as_ref() {
            let due_str = due
                .format(&time::macros::format_description!("[year]-[month]-[day]"))
                .unwrap_or_else(|_| "?".to_string());
            println!("Due: {bold}{due_str}{reset}");
        }

        if let Some(desc) = &ms.description {
            if !desc.is_empty() {
                println!();
                println!("{}", crate::markdown(desc));
            }
        }

        Ok(())
    })?;

    Ok(())
}

async fn create_milestone(
    repo: &RepoName,
    api: &Forgejo,
    title: String,
    body: Option<Option<String>>,
    due: Option<String>,
) -> eyre::Result<()> {
    let description = match body {
        Some(Some(body)) => Some(body),
        Some(None) => {
            let mut s = String::new();
            crate::editor(&mut s, Some("md")).await?;
            Some(s)
        }
        None => None,
    };

    let due_on = due.map(|d| parse_due_date(&d)).transpose()?;

    let opt = CreateMilestoneOption {
        title: Some(title.clone()),
        description,
        due_on,
        state: None,
    };
    api.issue_create_milestone(repo.owner(), repo.name(), opt)
        .await?;
    crate::output::success(&format!("Created milestone '{title}'"));
    Ok(())
}

async fn edit_milestone(
    repo: &RepoName,
    api: &Forgejo,
    name_or_id: &str,
    title: Option<String>,
    body: Option<String>,
    due: Option<String>,
    state: Option<String>,
) -> eyre::Result<()> {
    let ms = find_milestone(api, repo, name_or_id).await?;
    let id = ms.id.ok_or_eyre("milestone does not have id")?;

    let due_on = due.map(|d| parse_due_date(&d)).transpose()?;

    let display_title = title
        .as_deref()
        .or(ms.title.as_deref())
        .unwrap_or("?")
        .to_string();

    let opt = EditMilestoneOption {
        title,
        description: body,
        due_on,
        state,
    };
    api.issue_edit_milestone(repo.owner(), repo.name(), id, opt)
        .await?;
    crate::output::success(&format!("Updated milestone '{display_title}'"));
    Ok(())
}

async fn delete_milestone(
    repo: &RepoName,
    api: &Forgejo,
    name_or_id: &str,
) -> eyre::Result<()> {
    let ms = find_milestone(api, repo, name_or_id).await?;
    let id = ms.id.ok_or_eyre("milestone does not have id")?;
    let title = ms.title.as_deref().unwrap_or("?");

    if crate::prompt_bool(&format!("Delete milestone '{title}'?"), false).await? {
        api.issue_delete_milestone(repo.owner(), repo.name(), id)
            .await?;
        crate::output::success(&format!("Deleted milestone '{title}'"));
    } else {
        crate::output::info("Not deleted");
    }
    Ok(())
}
