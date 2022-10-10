use jujutsu_lib::index::IndexEntry;
    short_commit_description, short_commit_hash, write_commit_summary, Args, CommandError,
    CommandHelper, WorkspaceCommandHelper,
        if !ui.settings().allow_native_backend() {
            return Err(CommandError::UserError(
                "The native backend is disallowed by default. Did you mean to pass `--git`?
Set `ui.allow-init-native` to allow initializing a repo with the native backend."
                    .to_string(),
            ));
        }
        formatter.add_label("removed")?;
        formatter.add_label("added")?;
                    formatter.add_label("removed")?;
                    formatter.add_label("added")?;
    formatter.add_label("diff")?;
                formatter.add_label("header")?;
                formatter.add_label("header")?;
                formatter.add_label("header")?;
        formatter.add_label("hunk_header")?;
                    formatter.add_label("context")?;
                    formatter.add_label("removed")?;
                    formatter.add_label("added")?;
    formatter.add_label("diff")?;
        formatter.add_label("file_header")?;
    formatter.add_label("diff")?;
                formatter.add_label("modified")?;
                formatter.add_label("added")?;
                formatter.add_label("removed")?;
    let mut formatter = ui.stdout_formatter();
    let formatter = formatter.as_mut();
        formatter.write_str("Parent commit: ")?;
        write_commit_summary(
            formatter,
            repo.as_repo_ref(),
            &workspace_id,
            &wc_commit.parents()[0],
            ui.settings(),
        )?;
        formatter.write_str("\n")?;
        formatter.write_str("Working copy : ")?;
        write_commit_summary(
            formatter,
            repo.as_repo_ref(),
            &workspace_id,
            wc_commit,
            ui.settings(),
        )?;
        formatter.write_str("\n")?;
        formatter.write_str("No working copy\n")?;
        formatter.add_label("conflict")?;
        writeln!(formatter, "These branches have conflicts:")?;
        formatter.remove_label()?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "{}", branch_name)?;
            formatter.remove_label()?;
            writeln!(formatter)?;
            formatter,
        formatter.add_label("conflict")?;
        writeln!(formatter, "These remote branches have conflicts:")?;
        formatter.remove_label()?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "{}@{}", branch_name, remote_name)?;
            formatter.remove_label()?;
            writeln!(formatter)?;
            formatter,
            formatter.write_str("The working copy is clean\n")?;
            formatter.write_str("Working copy changes:\n")?;
                formatter,
            formatter.add_label("conflict")?;
            writeln!(formatter, "There are unresolved conflicts at these paths:")?;
            formatter.remove_label()?;
                writeln!(formatter, "{}", &workspace_command.format_file_path(&path))?;
    formatter.add_label("log")?;
                let mut formatter = ui.new_formatter(&mut buffer);
                    formatter.add_label("working_copy")?;
                let mut formatter = ui.new_formatter(&mut buffer);
    formatter.add_label("log")?;
                let mut formatter = ui.new_formatter(&mut buffer);
                let mut formatter = ui.new_formatter(&mut buffer);
    write_commit_summary(
        ui.stdout_formatter().as_mut(),
        ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
(parent) commit. The remainder will be in the second commit. If you
don't make any changes, then the operation will be aborted.
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            ui.settings(),
        |formatter: &mut dyn Formatter, target: Option<&RefTarget>| -> Result<(), CommandError> {
                    write!(formatter, ": ")?;
                    write_commit_summary(
                        formatter,
                        repo.as_repo_ref(),
                        &workspace_id,
                        &commit,
                        ui.settings(),
                    )?;
                    writeln!(formatter)?;
                    write!(formatter, " ")?;
                    formatter.add_label("conflict")?;
                    write!(formatter, "(conflicted)")?;
                    formatter.remove_label()?;
                    writeln!(formatter, ":")?;
                        write!(formatter, "  - ")?;
                        write_commit_summary(
                            formatter,
                            repo.as_repo_ref(),
                            &workspace_id,
                            &commit,
                            ui.settings(),
                        )?;
                        writeln!(formatter)?;
                        write!(formatter, "  + ")?;
                        write_commit_summary(
                            formatter,
                            repo.as_repo_ref(),
                            &workspace_id,
                            &commit,
                            ui.settings(),
                        )?;
                        writeln!(formatter)?;
                    writeln!(formatter, " (deleted)")?;
    let mut formatter = ui.stdout_formatter();
    let formatter = formatter.as_mut();
        formatter.add_label("branch")?;
        write!(formatter, "{}", name)?;
        formatter.remove_label()?;
        print_branch_target(formatter, branch_target.local_target.as_ref())?;
            write!(formatter, "  ")?;
            formatter.add_label("branch")?;
            write!(formatter, "@{}", remote)?;
            formatter.remove_label()?;
                    write!(formatter, " (ahead by {} commits)", remote_ahead_count)?;
                    write!(formatter, " (behind by {} commits)", local_ahead_count)?;
                        formatter,
            print_branch_target(formatter, Some(remote_target))?;
            formatter.add_label("id")?;
            formatter.add_label("user")?;
            formatter.add_label("time")?;
            formatter.add_label("description")?;
                formatter.add_label("tags")?;
            let mut formatter = ui.new_formatter(&mut buffer);
            formatter.add_label("op-log")?;
                formatter.add_label("head")?;
        write_commit_summary(
            ui.stdout_formatter().as_mut(),
            repo.as_repo_ref(),
            workspace_id,
            &commit,
            ui.settings(),
        )?;