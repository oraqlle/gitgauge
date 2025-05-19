/**
 * Verifies a URL to ensure it is a valid GitHub URL and extracts owner and repository information.
 *
 * @param {string} urlString The URL string to verify and parse.
 * @returns {{owner: string, repo: string}} An object containing the owner and repository name.
 * @throws {Error} If the URL is invalid, not a GitHub URL, or if owner/repo names are invalid.
 */
export function verifyAndExtractGitHubInfo(urlString) {
    let url;
    try {
        url = new URL(urlString);
    } catch (e) {
        throw new Error(`Invalid URL`);
    }

    const host = url.hostname;
    if (host !== 'github.com' && host !== 'www.github.com') {
        throw new Error('URL is not from GitHub (github.com or www.github.com)');
    }

    // Split pathname by '/' and filter out empty segments (e.g., leading/trailing slashes)
    const pathSegments = url.pathname.split('/').filter(segment => segment.length > 0);

    if (pathSegments.length < 2) {
        throw new Error('GitHub URL must contain at least an owner and a repository name.');
    }

    const owner = pathSegments[0];
    const repo = pathSegments[1];

    // Validate using GitHub naming rules (simplified):
    // - Alphanumeric characters, dashes, and underscores.
    // - Cannot start or end with a hyphen.
    // - No consecutive hyphens.
    const nameRegex = /^[a-zA-Z0-9](?:[a-zA-Z0-9-_]*[a-zA-Z0-9])?$/;
    const hyphenConsecutiveRegex = /--/;
    const startsOrEndsWithHyphenRegex = /^-|-$_/; // Though nameRegex should mostly cover this.

    if (!nameRegex.test(owner) || hyphenConsecutiveRegex.test(owner) || startsOrEndsWithHyphenRegex.test(owner)) {
        throw new Error(`Invalid GitHub owner name: "${owner}".`);
    }

    if (!nameRegex.test(repo) || hyphenConsecutiveRegex.test(repo) || startsOrEndsWithHyphenRegex.test(repo)) {
        throw new Error(`Invalid GitHub repository name: "${repo}".`);
    }

    return { owner, repo };
}

