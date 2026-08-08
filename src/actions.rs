use std::collections::BTreeMap;

use clap::{Args, Subcommand};
use eyre::{OptionExt, bail};
use forgejo_api::{
    Forgejo, ForgejoError,
    structs::{
        ActionArtifact, CreateOrUpdateSecretOption, CreateVariableOption, ListActionArtifactsQuery,
        ListActionRunsQuery, ListActionRunsQueryStatus, ListActionTasksQuery,
        ListActionTasksQueryStatus, RepoGetActionJobLogsQuery, UpdateVariableOption,
    },
};
use hyper::StatusCode;

use crate::repo::{RepoArg, RepoInfo, RepoName};

#[derive(Args, Clone, Debug)]
pub struct ActionsCommand {
    /// The local git remote that points to the repo to operate on
    #[clap(long, short = 'R', global = true)]
    remote: Option<String>,

    /// The repo to operate on
    #[clap(long, short, global = true)]
    repo: Option<RepoArg>,

    #[clap(subcommand)]
    command: ActionsSubcommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsSubcommand {
    /// List the tasks on a repo
    Tasks {
        /// The page to show. One page always includes up to 20 tasks.
        #[clap(long, short, default_value = "1")]
        page: u32,

        /// Only show tasks with this status. Can be given multiple times.
        #[clap(long, value_enum)]
        status: Vec<StatusFilter>,
    },

    /// List and manage workflow runs
    #[clap(alias = "runs")]
    Run {
        #[clap(subcommand)]
        command: ActionsRunSubcommand,
    },

    /// List and manage workflow run artifacts
    #[clap(alias = "artifacts")]
    Artifact {
        #[clap(subcommand)]
        command: ActionsArtifactSubcommand,
    },

    /// List and manage variables
    Variables {
        #[clap(subcommand)]
        command: ActionsVariablesSubcommmand,
    },

    Secrets {
        #[clap(subcommand)]
        command: ActionsSecretsSubcommmand,
    },

    /// Dispatch a workflow
    Dispatch {
        /// Name of the workflow to dispatch
        name: String,

        /// Git revision to dispatch the workflow on
        r#ref: String,

        #[clap(long, short = 'I', value_parser = parse_dispatch_kvs)]
        inputs: Vec<(String, String)>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsVariablesSubcommmand {
    /// List variables
    List {
        /// Also print owner_id and repo_id
        #[clap(long, short)]
        verbose: bool,
    },

    /// Create a new variable
    Create {
        /// The name of the new variable
        name: String,

        /// The data to save into the variable. Omit to invoke editor.
        data: Option<String>,

        /// Override existing variables
        #[clap(long, short)]
        force: bool,
    },

    Delete {
        /// The variable to delete
        name: String,
        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
        /// Preview without executing
        #[clap(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsSecretsSubcommmand {
    /// List secrets
    List,

    /// Create a new secret
    Create {
        /// The name of the new secret
        name: String,

        /// The data to save into the secret.
        data: String,
    },

    Delete {
        /// The secret to delete
        name: String,
        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
        /// Preview without executing
        #[clap(long)]
        dry_run: bool,
    },
}

/// A task/run status filter, mapped onto the forgejo-api query enums.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum StatusFilter {
    Unknown,
    Waiting,
    Running,
    Success,
    Failure,
    Cancelled,
    Skipped,
    Blocked,
}

impl From<StatusFilter> for ListActionTasksQueryStatus {
    fn from(status: StatusFilter) -> Self {
        match status {
            StatusFilter::Unknown => Self::Unknown,
            StatusFilter::Waiting => Self::Waiting,
            StatusFilter::Running => Self::Running,
            StatusFilter::Success => Self::Success,
            StatusFilter::Failure => Self::Failure,
            StatusFilter::Cancelled => Self::Cancelled,
            StatusFilter::Skipped => Self::Skipped,
            StatusFilter::Blocked => Self::Blocked,
        }
    }
}

impl From<StatusFilter> for ListActionRunsQueryStatus {
    fn from(status: StatusFilter) -> Self {
        match status {
            StatusFilter::Unknown => Self::Unknown,
            StatusFilter::Waiting => Self::Waiting,
            StatusFilter::Running => Self::Running,
            StatusFilter::Success => Self::Success,
            StatusFilter::Failure => Self::Failure,
            StatusFilter::Cancelled => Self::Cancelled,
            StatusFilter::Skipped => Self::Skipped,
            StatusFilter::Blocked => Self::Blocked,
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsRunSubcommand {
    /// List workflow runs
    List {
        /// The page to show. One page always includes up to 20 runs.
        #[clap(long, short, default_value = "1")]
        page: u32,

        /// Only show runs on this git reference, e.g. `refs/heads/main`
        #[clap(long)]
        r#ref: Option<String>,

        /// Only show runs of this workflow file, e.g. `ci.yml`
        #[clap(long)]
        workflow_id: Option<String>,

        /// Only show runs with this status. Can be given multiple times.
        #[clap(long, value_enum)]
        status: Vec<StatusFilter>,
    },

    /// View a workflow run
    View {
        /// The id of the run to view
        id: i64,
    },

    /// List the jobs of a workflow run
    Jobs {
        /// The id of the run to list jobs for
        id: i64,
    },

    /// Print the logs of a workflow run
    ///
    /// With `--job`, prints the plaintext logs of that job to stdout. Without
    /// it, writes a ZIP archive containing the logs of every job in the run
    /// to stdout.
    Logs {
        /// The id of the run to fetch logs for
        id: i64,

        /// Print the plaintext logs of this job (see `run jobs` for job ids)
        #[clap(long, short)]
        job: Option<i64>,
    },

    /// Cancel a pending or running workflow run
    Cancel {
        /// The id of the run to cancel
        id: i64,
    },

    /// Delete a completed workflow run
    Delete {
        /// The id of the run to delete
        id: i64,
        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
        /// Preview without executing
        #[clap(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionsArtifactSubcommand {
    /// List artifacts
    List {
        /// Only list artifacts of this workflow run
        #[clap(long)]
        run: Option<i64>,
    },

    /// Download an artifact's ZIP archive
    Download {
        /// The artifact to download, by id or name
        artifact: String,

        /// Where to save the artifact. Defaults to `<name>.zip`
        #[clap(long, short)]
        output: Option<std::path::PathBuf>,
    },

    /// Delete an artifact
    Delete {
        /// The artifact to delete, by id or name
        artifact: String,
        /// Skip confirmation prompt
        #[clap(long, short = 'f')]
        force: bool,
        /// Preview without executing
        #[clap(long)]
        dry_run: bool,
    },
}

impl ActionsCommand {
    pub async fn run(self, keys: &mut crate::KeyInfo, host_name: Option<&str>) -> eyre::Result<()> {
        let repo =
            RepoInfo::get_current(host_name, self.repo.as_ref(), self.remote.as_deref(), keys)?;

        let api = keys.get_api(repo.host_url()).await?;
        let repo = repo
            .name()
            .ok_or_eyre("can't figure what repo to access, try specifying with `--repo`")?;
        match self.command {
            ActionsSubcommand::Tasks { page, status } => {
                view_tasks(repo, &api, page, status).await?
            }

            ActionsSubcommand::Run { command } => match command {
                ActionsRunSubcommand::List {
                    page,
                    r#ref,
                    workflow_id,
                    status,
                } => list_runs(repo, &api, page, r#ref, workflow_id, status).await?,
                ActionsRunSubcommand::View { id } => view_run(repo, &api, id).await?,
                ActionsRunSubcommand::Jobs { id } => list_run_jobs(repo, &api, id).await?,
                ActionsRunSubcommand::Logs { id, job } => run_logs(repo, &api, id, job).await?,
                ActionsRunSubcommand::Cancel { id } => cancel_run(repo, &api, id).await?,
                ActionsRunSubcommand::Delete { id, force, dry_run } => {
                    delete_run(repo, &api, id, force, dry_run).await?
                }
            },

            ActionsSubcommand::Artifact { command } => match command {
                ActionsArtifactSubcommand::List { run } => list_artifacts(repo, &api, run).await?,
                ActionsArtifactSubcommand::Download { artifact, output } => {
                    download_artifact(repo, &api, artifact, output).await?
                }
                ActionsArtifactSubcommand::Delete {
                    artifact,
                    force,
                    dry_run,
                } => delete_artifact(repo, &api, artifact, force, dry_run).await?,
            },

            ActionsSubcommand::Variables { command } => match command {
                ActionsVariablesSubcommmand::List { verbose } => {
                    list_variables(repo, &api, verbose).await?
                }
                ActionsVariablesSubcommmand::Create { name, data, force } => {
                    create_variable(repo, &api, name, data, force).await?
                }
                ActionsVariablesSubcommmand::Delete {
                    name,
                    force,
                    dry_run,
                } => delete_variable(repo, &api, name, force, dry_run).await?,
            },

            ActionsSubcommand::Secrets { command } => match command {
                ActionsSecretsSubcommmand::List => list_secrets(repo, &api).await?,
                ActionsSecretsSubcommmand::Create { name, data } => {
                    create_secret(repo, &api, name, data).await?
                }
                ActionsSecretsSubcommmand::Delete {
                    name,
                    force,
                    dry_run,
                } => delete_secret(repo, &api, name, force, dry_run).await?,
            },

            ActionsSubcommand::Dispatch {
                name,
                r#ref,
                inputs,
            } => dispatch(repo, &api, name, r#ref, inputs.into_iter().collect()).await?,
        }

        Ok(())
    }
}

async fn view_tasks(
    repo: &RepoName,
    api: &Forgejo,
    page: u32,
    status: Vec<StatusFilter>,
) -> eyre::Result<()> {
    let query = ListActionTasksQuery {
        status: if status.is_empty() {
            None
        } else {
            Some(status.into_iter().map(Into::into).collect())
        },
    };

    // We don't iterate this to collect all tasks (not just the ones on the first page) like the
    // issue search subcommand will do, because it's unlikely someone wants to see *all* tasks.
    let res = api
        .list_action_tasks(repo.owner(), repo.name(), query)
        .page(page)
        .page_size(20)
        .await?;

    let tasks = res.workflow_runs.unwrap_or_default();

    crate::output::print_list(
        &tasks,
        &["#", "STATUS", "NAME", "TITLE", "SHA", "EVENT", "TIME"],
        |task| {
            let run_number = task
                .run_number
                .map(|n| n.to_string())
                .unwrap_or_else(|| "?".to_string());

            let status = colored_task_status(task.status.as_deref());

            let name = task.name.as_deref().unwrap_or("").to_string();
            let title = task.display_title.as_deref().unwrap_or("").to_string();

            let sha = task.head_sha.as_deref().unwrap_or("");
            let sha = if sha.len() > 10 { &sha[0..10] } else { sha };
            let sha = sha.to_string();

            let event = task.event.as_deref().unwrap_or("").to_string();

            let time = if let (Some(end), Some(start)) = (task.updated_at, task.run_started_at) {
                format!("{}", end - start)
            } else {
                String::new()
            };

            vec![run_number, status, name, title, sha, event, time]
        },
    );

    Ok(())
}

fn colored_task_status(status: Option<&str>) -> String {
    let crate::SpecialRender {
        fancy,
        reset,
        bright_green,
        light_grey,
        bright_red,
        yellow,
        ..
    } = *crate::special_render();

    match status {
        x if !fancy => x.unwrap_or("?").to_string(),
        // See: https://codeberg.org/forgejo/forgejo/src/commit/5380f23daba969057d9afc53c3dc746eca95188c/models/actions/status.go#L26
        Some("success") => format!("{bright_green}success{reset}"),
        Some("cancelled") => format!("{light_grey}cancelled{reset}"),
        Some("failure") => format!("{bright_red}failure{reset}"),
        Some("waiting") => format!("{light_grey}waiting{reset}"),
        Some("running") => format!("{yellow}running{reset}"),
        Some("skipped") => format!("{light_grey}skipped{reset}"),
        Some("blocked") => format!("{bright_red}blocked{reset}"),
        Some(x) => x.to_string(),
        None => "?".to_string(),
    }
}

async fn list_runs(
    repo: &RepoName,
    api: &Forgejo,
    page: u32,
    r#ref: Option<String>,
    workflow_id: Option<String>,
    status: Vec<StatusFilter>,
) -> eyre::Result<()> {
    let query = ListActionRunsQuery {
        event: None,
        status: if status.is_empty() {
            None
        } else {
            Some(status.into_iter().map(Into::into).collect())
        },
        run_number: None,
        head_sha: None,
        r#ref,
        workflow_id,
    };

    crate::verbose_log!("Listing runs on {}/{}", repo.owner(), repo.name());
    let res = api
        .list_action_runs(repo.owner(), repo.name(), query)
        .page(page)
        .page_size(20)
        .await?;

    let runs = res.workflow_runs.unwrap_or_default();

    crate::output::print_list(
        &runs,
        &["ID", "WORKFLOW", "STATUS", "REF", "STARTED"],
        |run| {
            vec![
                run.id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "?".to_string()),
                run.workflow_id.as_deref().unwrap_or("?").to_string(),
                colored_task_status(run.status.as_deref()),
                run.prettyref.as_deref().unwrap_or("").to_string(),
                run.started
                    .as_ref()
                    .map(crate::output::relative_time)
                    .unwrap_or_default(),
            ]
        },
    );

    Ok(())
}

async fn view_run(repo: &RepoName, api: &Forgejo, id: i64) -> eyre::Result<()> {
    crate::verbose_log!("Fetching run {id} on {}/{}", repo.owner(), repo.name());
    let run = api.get_action_run(repo.owner(), repo.name(), id).await?;

    crate::output::print_or_json(&run, || {
        let crate::SpecialRender { bold, reset, .. } = *crate::special_render();

        let title = run.title.as_deref().unwrap_or("(untitled)");
        println!("{bold}{title}{reset}");
        println!("workflow: {}", run.workflow_id.as_deref().unwrap_or("?"));
        println!("status: {}", colored_task_status(run.status.as_deref()));
        println!(
            "trigger: {} by {}",
            run.trigger_event
                .as_deref()
                .or(run.event.as_deref())
                .unwrap_or("?"),
            run.trigger_user
                .as_ref()
                .and_then(|user| user.login.as_deref())
                .unwrap_or("?"),
        );
        println!("ref: {}", run.prettyref.as_deref().unwrap_or("?"));
        if let Some(sha) = run.commit_sha.as_deref() {
            let sha = if sha.len() > 10 { &sha[0..10] } else { sha };
            println!("commit: {sha}");
        }
        if let Some(created) = run.created {
            println!("created: {created}");
        }
        if let Some(started) = run.started {
            println!("started: {started}");
        }
        if let Some(stopped) = run.stopped {
            println!("stopped: {stopped}");
        }
        if let Some(url) = &run.html_url {
            println!("url: {url}");
        }
        Ok(())
    })?;

    Ok(())
}

async fn list_run_jobs(repo: &RepoName, api: &Forgejo, id: i64) -> eyre::Result<()> {
    crate::verbose_log!(
        "Listing jobs of run {id} on {}/{}",
        repo.owner(),
        repo.name()
    );
    let jobs = api
        .list_action_run_jobs(repo.owner(), repo.name(), id)
        .await?;

    crate::output::print_list(&jobs, &["ID", "NAME", "STATUS"], |job| {
        vec![
            job.id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "?".to_string()),
            job.name.as_deref().unwrap_or("?").to_string(),
            colored_task_status(job.status.as_deref()),
        ]
    });

    Ok(())
}

async fn run_logs(repo: &RepoName, api: &Forgejo, id: i64, job: Option<i64>) -> eyre::Result<()> {
    match job {
        Some(job_id) => {
            crate::verbose_log!(
                "Fetching logs of job {job_id} on {}/{}",
                repo.owner(),
                repo.name()
            );
            let logs = api
                .repo_get_action_job_logs(
                    repo.owner(),
                    repo.name(),
                    job_id,
                    RepoGetActionJobLogsQuery::default(),
                )
                .await?;
            print!("{logs}");
        }
        None => {
            crate::verbose_log!(
                "Fetching log archive of run {id} on {}/{}",
                repo.owner(),
                repo.name()
            );
            let archive = api
                .repo_get_action_run_logs(repo.owner(), repo.name(), id)
                .await?;
            use std::io::Write;
            std::io::stdout().write_all(&archive)?;
        }
    }

    Ok(())
}

async fn cancel_run(repo: &RepoName, api: &Forgejo, id: i64) -> eyre::Result<()> {
    crate::verbose_log!("Cancelling run {id} on {}/{}", repo.owner(), repo.name());
    api.cancel_action_run(repo.owner(), repo.name(), id).await?;
    crate::output::success(&format!("Cancelled run {id}"));

    Ok(())
}

async fn delete_run(
    repo: &RepoName,
    api: &Forgejo,
    id: i64,
    force: bool,
    dry_run: bool,
) -> eyre::Result<()> {
    if dry_run {
        crate::output::dry_run(&format!(
            "delete run {id} on {}/{}",
            repo.owner(),
            repo.name()
        ));
        return Ok(());
    }

    if !force
        && !crate::yes_mode()
        && !crate::prompt_bool(&format!("Delete run {id}?"), false).await?
    {
        crate::output::info("Not deleted");
        return Ok(());
    }

    crate::verbose_log!("Deleting run {id} on {}/{}", repo.owner(), repo.name());
    api.delete_action_run(repo.owner(), repo.name(), id).await?;
    crate::output::success(&format!("Deleted run {id}"));

    Ok(())
}

async fn list_artifacts(repo: &RepoName, api: &Forgejo, run: Option<i64>) -> eyre::Result<()> {
    let artifacts = match run {
        Some(run_id) => {
            crate::verbose_log!(
                "Listing artifacts of run {run_id} on {}/{}",
                repo.owner(),
                repo.name()
            );
            api.list_action_run_artifacts(
                repo.owner(),
                repo.name(),
                run_id,
                forgejo_api::structs::ListActionRunArtifactsQuery::default(),
            )
            .await?
        }
        None => {
            crate::verbose_log!("Listing artifacts on {}/{}", repo.owner(), repo.name());
            api.list_action_artifacts(
                repo.owner(),
                repo.name(),
                ListActionArtifactsQuery::default(),
            )
            .await?
        }
    };

    crate::output::print_list(&artifacts, &["ID", "NAME", "SIZE", "EXPIRES"], |artifact| {
        vec![
            artifact
                .id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "?".to_string()),
            artifact.name.as_deref().unwrap_or("?").to_string(),
            artifact
                .size_in_bytes
                .map(format_size)
                .unwrap_or_else(|| "?".to_string()),
            if artifact.expired == Some(true) {
                "expired".to_string()
            } else {
                artifact
                    .expires_at
                    .map(|t| t.date().to_string())
                    .unwrap_or_else(|| "?".to_string())
            },
        ]
    });

    Ok(())
}

/// Find an artifact by numeric id first, falling back to a server-side name search.
async fn find_artifact(repo: &RepoName, api: &Forgejo, arg: &str) -> eyre::Result<ActionArtifact> {
    if let Ok(id) = arg.parse::<i64>() {
        crate::verbose_log!(
            "Looking up artifact by id {id} on {}/{}",
            repo.owner(),
            repo.name()
        );
        match api.get_action_artifact(repo.owner(), repo.name(), id).await {
            Ok(artifact) => return Ok(artifact),
            Err(ForgejoError::ApiError(forgejo_api::ApiError {
                kind: forgejo_api::ApiErrorKind::NotFound { .. },
                ..
            })) => {
                crate::verbose_log!("No artifact with id {id}, falling back to name search");
            }
            Err(e) => return Err(e.into()),
        }
    }

    crate::verbose_log!(
        "Searching for artifact named '{arg}' on {}/{}",
        repo.owner(),
        repo.name()
    );
    let artifacts = api
        .list_action_artifacts(
            repo.owner(),
            repo.name(),
            ListActionArtifactsQuery {
                name: Some(arg.to_string()),
            },
        )
        .await?;

    // Forgejo keeps a same-named artifact per workflow run, so a bare name
    // can match several; acting on an arbitrary one would silently target a
    // stale build. Prefer the newest and say so.
    let mut matches: Vec<ActionArtifact> = artifacts
        .into_iter()
        .filter(|artifact| artifact.name.as_deref() == Some(arg))
        .collect();
    if matches.len() > 1 {
        matches.sort_by_key(|a| a.id);
        let newest = matches.pop().expect("len checked above");
        crate::output::info(&format!(
            "{} artifacts named '{arg}' found, using newest (id {}); pass an id to be explicit",
            matches.len() + 1,
            newest
                .id
                .map_or_else(|| "?".to_string(), |id| id.to_string()),
        ));
        return Ok(newest);
    }
    matches
        .into_iter()
        .next()
        .ok_or_else(|| eyre::eyre!("could not find artifact {arg}"))
}

async fn download_artifact(
    repo: &RepoName,
    api: &Forgejo,
    artifact: String,
    output: Option<std::path::PathBuf>,
) -> eyre::Result<()> {
    use tokio::io::AsyncWriteExt;

    let found = find_artifact(repo, api, &artifact).await?;
    let id = found.id.ok_or_eyre("artifact does not have id")?;
    let name = found.name.as_deref().unwrap_or(&artifact);

    crate::verbose_log!(
        "Downloading artifact {id} on {}/{}",
        repo.owner(),
        repo.name()
    );
    let file = api
        .download_action_artifact(repo.owner(), repo.name(), id)
        .await?;

    let default_output = std::path::PathBuf::from(format!("{name}.zip"));
    let real_output = output.as_deref().unwrap_or(&default_output);
    tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(real_output)
        .await?
        .write_all(file.as_ref())
        .await?;

    crate::output::success(&format!("Downloaded {name} into {}", real_output.display()));

    Ok(())
}

async fn delete_artifact(
    repo: &RepoName,
    api: &Forgejo,
    artifact: String,
    force: bool,
    dry_run: bool,
) -> eyre::Result<()> {
    if dry_run {
        crate::output::dry_run(&format!(
            "delete artifact {artifact} on {}/{}",
            repo.owner(),
            repo.name()
        ));
        return Ok(());
    }

    if !force
        && !crate::yes_mode()
        && !crate::prompt_bool(&format!("Delete artifact '{artifact}'?"), false).await?
    {
        crate::output::info("Not deleted");
        return Ok(());
    }

    let found = find_artifact(repo, api, &artifact).await?;
    let id = found.id.ok_or_eyre("artifact does not have id")?;

    crate::verbose_log!("Deleting artifact {id} on {}/{}", repo.owner(), repo.name());
    api.delete_action_artifact(repo.owner(), repo.name(), id)
        .await?;
    crate::output::success(&format!("Deleted artifact {artifact}"));

    Ok(())
}

/// Format a byte count as a human-readable size, e.g. `1.5 MiB`.
fn format_size(bytes: i64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    if bytes < 0 {
        return "?".to_string();
    }

    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

async fn list_variables(repo: &RepoName, api: &Forgejo, verbose: bool) -> eyre::Result<()> {
    let variables = api
        .get_repo_variables_list(repo.owner(), repo.name())
        .all()
        .await?;

    if verbose {
        crate::output::print_list(
            &variables,
            &["NAME", "VALUE", "OWNER_ID", "REPO_ID"],
            |var| {
                vec![
                    var.name.as_deref().unwrap_or("?").to_string(),
                    var.data.as_deref().unwrap_or("").to_string(),
                    var.owner_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "?".to_string()),
                    var.repo_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "?".to_string()),
                ]
            },
        );
    } else {
        crate::output::print_list(&variables, &["NAME", "VALUE"], |var| {
            vec![
                var.name.as_deref().unwrap_or("?").to_string(),
                var.data.as_deref().unwrap_or("").to_string(),
            ]
        });
    }

    Ok(())
}

async fn create_variable(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    data: Option<String>,
    force: bool,
) -> eyre::Result<()> {
    let mut data = if let Some(data) = data {
        data
    } else {
        let mut data = String::new();
        crate::editor(&mut data, Some("variable_content.txt")).await?;
        data
    };

    match api
        .create_repo_variable(
            repo.owner(),
            repo.name(),
            &name,
            CreateVariableOption {
                // If we don't have force enabled, we will not need the data again to (potentially)
                // make another request. To avoid a clone in this case, we take the string here,
                // replacing it with an empty one.
                value: if force {
                    data.clone()
                } else {
                    std::mem::take(&mut data)
                },
            },
        )
        .await
    {
        Err(ForgejoError::ApiError(forgejo_api::ApiError {
            kind: forgejo_api::ApiErrorKind::Other(StatusCode::CONFLICT),
            ..
        })) => {
            if !force {
                bail!("variable already exists, pass --force to replace it.");
            }

            crate::output::info("Variable already exists, updating");
            api.update_repo_variable(
                repo.owner(),
                repo.name(),
                &name,
                UpdateVariableOption {
                    name: None,
                    value: data,
                },
            )
            .await?;
            crate::output::success(&format!("Updated variable {name}"));
        }
        Err(e) => return Err(e.into()),
        Ok(()) => {
            crate::output::success(&format!("Created variable {name}"));
        }
    }

    Ok(())
}

async fn delete_variable(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    force: bool,
    dry_run: bool,
) -> eyre::Result<()> {
    if dry_run {
        crate::output::dry_run(&format!(
            "delete variable {name} on {}/{}",
            repo.owner(),
            repo.name()
        ));
        return Ok(());
    }

    if !force
        && !crate::yes_mode()
        && !crate::prompt_bool(&format!("Delete variable '{name}'?"), false).await?
    {
        crate::output::info("Not deleted");
        return Ok(());
    }

    crate::verbose_log!(
        "Deleting variable {name} on {}/{}",
        repo.owner(),
        repo.name()
    );
    api.delete_repo_variable(repo.owner(), repo.name(), &name)
        .await?;
    crate::output::success(&format!("Deleted variable {name}"));

    Ok(())
}

async fn list_secrets(repo: &RepoName, api: &Forgejo) -> eyre::Result<()> {
    let secrets = api
        .repo_list_actions_secrets(repo.owner(), repo.name())
        .all()
        .await?;

    crate::output::print_list(&secrets, &["NAME", "CREATED"], |secret| {
        vec![
            secret.name.as_deref().unwrap_or("?").to_string(),
            secret
                .created_at
                .as_ref()
                .map(crate::output::relative_time)
                .unwrap_or_else(|| "?".to_string()),
        ]
    });

    Ok(())
}

async fn create_secret(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    data: String,
) -> eyre::Result<()> {
    api.update_repo_secret(
        repo.owner(),
        repo.name(),
        &name,
        CreateOrUpdateSecretOption { data },
    )
    .await?;
    crate::output::success(&format!("Created secret {name}"));

    Ok(())
}

async fn delete_secret(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    force: bool,
    dry_run: bool,
) -> eyre::Result<()> {
    if dry_run {
        crate::output::dry_run(&format!(
            "delete secret {name} on {}/{}",
            repo.owner(),
            repo.name()
        ));
        return Ok(());
    }

    if !force
        && !crate::yes_mode()
        && !crate::prompt_bool(&format!("Delete secret '{name}'?"), false).await?
    {
        crate::output::info("Not deleted");
        return Ok(());
    }

    crate::verbose_log!("Deleting secret {name} on {}/{}", repo.owner(), repo.name());
    api.delete_repo_secret(repo.owner(), repo.name(), &name)
        .await?;
    crate::output::success(&format!("Deleted secret {name}"));

    Ok(())
}

async fn dispatch(
    repo: &RepoName,
    api: &Forgejo,
    name: String,
    r#ref: String,
    inputs: BTreeMap<String, String>,
) -> eyre::Result<()> {
    let n_inputs = inputs.len();
    api.dispatch_workflow(
        repo.owner(),
        repo.name(),
        &name,
        forgejo_api::structs::DispatchWorkflowOption {
            inputs: Some(inputs),
            return_run_info: Some(false),
            r#ref: r#ref.clone(),
        },
    )
    .await?;

    crate::output::success(&format!(
        "Dispatched workflow {name} in {ref} with {n_inputs} input(s)"
    ));

    Ok(())
}

fn parse_dispatch_kvs(s: &str) -> eyre::Result<(String, String)> {
    let eq_idx = s
        .find('=')
        .ok_or_eyre("Input argument does not contain a '=' character!")?;

    Ok((s[..eq_idx].to_string(), s[eq_idx + 1..].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dispatch_kvs_simple() {
        let (k, v) = parse_dispatch_kvs("key=value").unwrap();
        assert_eq!(k, "key");
        assert_eq!(v, "value");
    }

    #[test]
    fn parse_dispatch_kvs_value_containing_equals() {
        let (k, v) = parse_dispatch_kvs("key=a=b").unwrap();
        assert_eq!(k, "key");
        assert_eq!(v, "a=b");
    }

    #[test]
    fn parse_dispatch_kvs_empty_value() {
        let (k, v) = parse_dispatch_kvs("key=").unwrap();
        assert_eq!(k, "key");
        assert_eq!(v, "");
    }

    #[test]
    fn parse_dispatch_kvs_no_equals_is_error() {
        assert!(parse_dispatch_kvs("no-equals").is_err());
    }

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn format_size_kibibytes() {
        assert_eq!(format_size(1024), "1.0 KiB");
        assert_eq!(format_size(1536), "1.5 KiB");
    }

    #[test]
    fn format_size_mebibytes() {
        assert_eq!(format_size(5 * 1024 * 1024), "5.0 MiB");
    }

    #[test]
    fn format_size_gibibytes() {
        assert_eq!(format_size(2 * 1024 * 1024 * 1024), "2.0 GiB");
    }

    #[test]
    fn format_size_negative_is_unknown() {
        assert_eq!(format_size(-1), "?");
    }
}
