# Contributing Guidelines

This document outlines the rules for contributing to the project. It covers the
required commit message format, how to name branches and how to create PRs.

In general it is *highly recommended* that you use the terminal when interecting with
the repository through Git as this will provide a much more unified experience when
developing and contributing to the project.

1. Clone or fork the repository.
2. Create a new branch based on the `devel` branch.
   Branches must have a 'tag' to indicate the kind of change that is being made.
   The branch name is separated from the tag using a `/` with the name of the branch
   being in lower-kebab-case ie. `my-branch`. Keep branch names simple and indicative
   of changes that will be made on the branch.

    ```sh
    git checkout -b <tag>/<branch> devel
    ```

    Available tags include:

    - `feature` : New major feature
    - `bugfix`  : Bug fixes
    - `conf`    : Changes to internal documentation, configs etc.
    - `docs`    : Documentation changes
    - `ci`      : CI/CD changes
    - `hotfix`  : Fix that is to be fast-tracked into the project as a bug now breaks the project.

3. Make your changes, ensuring to commit frequently
4. When committing, add/stage your changes using `git add .` You can then make a commit
   using `git commit`. This will open the default editor setup for Git allowing you to
   write your commit message. The `git-setup.sh` script will have injected a hook that
   provides a custom template for commit messages. The template is based on the below
   [guidelines](#commit-message-guidelines), which will help you to
5. If others need to contribute to your branch you can push it to the remote.

   ```sh
    git push -u origin <tag>/<branch>
   ```
   > Future pushes can omit the `-u` flag.

6. Once your change is ready, you must submit a 'Pull Request' (PR) on the remote
   repository. A variety of templates will be provided depending on the kind of change
   you a proposing. Select the most applicable one and fill in the template sections.
   Ensure your PR is being made against `devel` and not `main` otherwise it will not be
   approved.

   > You can begin a draft PR before you have finished your changes when using GitHub.
   > Future commits & pushes on the branch you're requesting be 'pulled' will stack into
   > PR automatically. Once you are done you can un-draft your PR.

7. Once your PR is ready, you can request a review from your team's Software Architect
   (SA). Your SA may request changes be made before accepting the PR. 

   > Your changes will be validated using the CI/CD pipelines to ensure compliance with
   > existing code and to ensure your change works as expected.

8. Once the SA has approved the PR the merge will be issued resulting in your
   changes being visible on `devel`.

   > If you are working on a separate branch but require changes merged into `devel`, you
   > will need to rebase your branch on the most recent version of `devel` with:
   > 
   > ```sh
   > git pull origin devel
   > git rebase -i devel <tag>/<other-branch>
   > ```
   >
   > This will interactively apply the changes in `devel` through your branches existing
   > history, therefore rewriting your branch's history as if the changes had always
   > been a part of your branch's history.
   > 
   > You are free to rebase from another branch that is not `devel` or `main` if said
   > branch has new functionality you require for your changes and has not been
   > approved to be merged into `devel` yet.

9. SA's will handle bundling changes made to `devel` into a release PR to `main`.

## Commit Message Guidelines

Ensuring your commit messages are of the expected format and style is of vital importance
to ensure your commits document your changes well. It also makes reviewing commits much
quicker and easier as we can accurately search through the history using identifiable
markers left in messages.

As a minimum, your commit message must have a 'summary' line and a 'description'
paragraph. The summary line is a simple one-line explanation of the changes made in the
commit. It must be a maximum of 72 characters long and *not* terminated by a period. It
must also use an imperative mood/language in the summary and descriptions ie. as if you
are giving orders to the codebase to change its behaviour.

If you have setup the `.githooks` folder correctly then each commit will have a template
you can fill out to guide you through making the commit message.

