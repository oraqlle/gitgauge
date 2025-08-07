import { writable } from "svelte/store";
import type { Repo } from "$lib/repo";
import { getRepoType as get_repo_type } from "$lib/repo";

export const currentRepo = writable<Repo>({
  "repo_url": "https://github.com/Monash/FIT3170/2025W1-Commitment.git",
  "repo_path": "Monash/FIT3170/2025W1-Commitment",
  "repo_type": "github"
});

export function set_repo_url(repo: string) {
  let name = new URL(repo).pathname.slice(1);
  currentRepo.set({
    "repoUrl": repo,
    "repoPath": name,
    "repoType": get_repo_type(repo)
  });
}
