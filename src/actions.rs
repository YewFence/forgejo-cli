use std::collections::BTreeMap;

use clap::{Args, Subcommand};
use eyre::{bail, OptionExt};
use forgejo_api::{
    structs::{CreateOrUpdateSecretOption, CreateVariableOption, UpdateVariableOption},
    Forgejo, ForgejoError,
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
            ActionsSubcommand::Tasks { page } => view_tasks(repo, &api, page).await?,

            ActionsSubcommand::Variables { command } => match command {
                ActionsVariablesSubcommmand::List { verbose } => {
                    list_variables(repo, &api, verbose).await?
                }
                ActionsVariablesSubcommmand::Create { name, data, force } => {
                    create_variable(repo, &api, name, data, force).await?
                }
                ActionsVariablesSubcommmand::Delete { name } => {
                    delete_variable(repo, &api, name).await?
                }
            },

            ActionsSubcommand::Secrets { command } => match command {
                ActionsSecretsSubcommmand::List => list_secrets(repo, &api).await?,
                ActionsSecretsSubcommmand::Create { name, data } => {
                    create_secret(repo, &api, name, data).await?
                }
                ActionsSecretsSubcommmand::Delete { name } => {
                    delete_secret(repo, &api, name).await?
                }
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

async fn view_tasks(repo: &RepoName, api: &Forgejo, page: u32) -> eyre::Result<()> {
    // We don't iterate this to collect all tasks (not just the ones on the first page) like the
    // issue search subcommand will do, because it's unlikely someone wants to see *all* tasks.
    let res = api
        .list_action_tasks(repo.owner(), repo.name())
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

async fn delete_variable(repo: &RepoName, api: &Forgejo, name: String) -> eyre::Result<()> {
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

async fn delete_secret(repo: &RepoName, api: &Forgejo, name: String) -> eyre::Result<()> {
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
