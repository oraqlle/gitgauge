import type { User } from '../data/dummyData';
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";

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

// 1. Total Commits for a user
export function getUserTotalCommits(user: User): number {
    return user.commits.length;
}

// 2. Total Lines of Code (additions + deletions) for a user
export function getUserTotalLinesOfCode(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
        commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
            fileAcc + file.added + file.deleted, 0
        ), 0
    );
}

// 3. Lines per Commit for a user
export function getUserLinesPerCommit(user: User): number {
    const totalCommits = getUserTotalCommits(user);
    const totalLines = getUserTotalLinesOfCode(user);
    return totalCommits === 0 ? 0 : Math.round(totalLines / totalCommits);
}

// 4. Commits per Day for a user
export function getUserCommitsPerDay(user: User): number {
    const allDates = user.commits.map(commit => commit.date);
    const uniqueDates = new Set(allDates);
    const totalCommits = getUserTotalCommits(user);
    return uniqueDates.size === 0 ? 0 : +(totalCommits / uniqueDates.size).toFixed(2);
}

// 5. Total Additions for a user
export function getUserTotalAdditions(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
        commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
            fileAcc + file.added, 0
        ), 0
    );
}

// 6. Total Deletions for a user
export function getUserTotalDeletions(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
        commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
            fileAcc + file.deleted, 0
        ), 0
    );
}

// Calculate average commits
export function getAverageCommits(users: User[]): number {
    if (users.length === 0) return 0;
    const commit_mean: number = users.reduce((acc, curr) => {
        return acc + curr.commits.length;
    }, 0) / users.length;

    return commit_mean;
}

// Calculate standard deviation
export function getSD(users: User[]): number {
    if (users.length === 0) return 0;
    let commits: number[] = [];

    // Get the list of total commits for each user
    users.forEach(user => {
        commits.push(user.commits.length);
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