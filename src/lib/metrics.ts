//import type { User } from '../data/dummyData';
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";

export type Author = Readonly<{
    login: string,
    avatar_url: string 
}>;

export type Contributor = Readonly<{
    author: Author,
    total_commits: number,
    additions: number,
    deletions: number
}>;


// Load branches for a repository
export async function loadBranches(owner: string, repo: string): Promise<string[]> {
    info(`Loading branches for ${owner}/${repo}...`);
    try {
        const realBranches = await invoke<string[]>('get_branch_names', { owner, repo });
        return ['All', ...realBranches];
    } catch (err) {
        console.error('Failed to load branches: ', err);
        return ['All'];
    }
}

export async function loadCommitData(owner: string, repo: string, branch?: string): Promise<Contributor[]> {
    info(`Loading contributor data for ${owner}/${repo}...`);

    try {
        const commitData = await invoke<Contributor[]>('get_contributor_info', { owner, repo });
        // info(`${commitData}`);
        return commitData;
    } catch (err) {
        info(`Failed to get contributor data`)
        console.error('Failed to load contributor data: ', err);
        return [];
    }
}

// 1. Total Commits for a user
export function getUserTotalCommits(user: Contributor): number {
    return user.total_commits;
}

// 2. Total Lines of Code (additions + deletions) for a user
export function getUserTotalLinesOfCode(user: Contributor): number {
    return user.additions + user.deletions;
}

// 3. Lines per Commit for a user
export function getUserLinesPerCommit(user: Contributor): number {
    const totalCommits = getUserTotalCommits(user);
    const totalLines = getUserTotalLinesOfCode(user);
    return totalCommits === 0 ? 0 : Math.round(totalLines / totalCommits);
}

// 4. Commits per Day for a user
// export function getUserCommitsPerDay(user: User): number {
//     const allDates = user.commits.map(commit => commit.date);
//     const uniqueDates = new Set(allDates);
//     const totalCommits = getUserTotalCommits(user);
//     return uniqueDates.size === 0 ? 0 : +(totalCommits / uniqueDates.size).toFixed(2);
// }

// 5. Total Additions for a user
export function getUserTotalAdditions(user: Contributor): number {
    return user.additions;
}

// 6. Total Deletions for a user
export function getUserTotalDeletions(user: Contributor): number {
    return user.deletions;
}

// Calculate average commits
export function getAverageCommits(users: Contributor[]): number {
    if (users.length === 0) return 0;
    const commit_mean: number = users.reduce((acc, curr) => {
        return acc + curr.total_commits;
    }, 0) / users.length;

    return commit_mean;
}

// Calculate standard deviation
export function getSD(users: Contributor[]): number {
    if (users.length === 0) return 0;
    let commits: number[] = [];

    // Get the list of total commits for each user
    users.forEach(user => {
        commits.push(user.total_commits);
    });

    // Creating the mean with Array.reduce
    const n: number = users.length;
    const mean = getAverageCommits(users);

    const variance: number = commits.reduce((acc: number, val: number) => acc + Math.pow(val - mean, 2), 0) / n;
    
    return Math.sqrt(variance);
}

// Calculate reference points
export function getRefPoints(mean: number, sd: number): number[] {
    if (sd === 0) return [mean, mean, mean, mean, mean];
    return [
        (mean - (2 * sd)),
        (mean - sd),
        mean,
        (mean + sd),
        (mean + (2 * sd))
    ];
}

// Calculate scaling factor
export function calculateScalingFactor(numCommits: number, mean: number, sd: number): number {
    if (sd === 0) return 1.0;
    const zScore = (numCommits - mean) / sd;
    const EPSILON = 1e-6;
    if (Math.abs(zScore) < EPSILON) {
        return 1.0;
    } else if (Math.abs(zScore) <= 1) {
        return 1.0;
    } else if (zScore < -1 && zScore >= -2) {
        return 0.9;
    } else if (zScore > 1 && zScore <= 2) {
        return 1.1;
    } else {
        return zScore < 0 ? 0.8 : 1.2;
    }
} 
