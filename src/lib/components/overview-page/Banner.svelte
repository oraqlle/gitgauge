<script lang="ts">
    import Icon from '@iconify/svelte';
    import { bookmarks } from '$lib/stores/bookmarks';
    import type { Repo } from '$lib/repo';
    import { get_repo_type } from '$lib/repo';

    let {
        repo_url,
        repo_path,
    }: {
        repo_url: string;
        repo_path: string;
    } = $props();

    let bookmarked = $state(bookmarks.contains(repo_url));

    function toggle_bookmark() {
      bookmarked = !bookmarked;
      const bookmark: Repo = {
        "repo_path": repo_url.split('/').pop()?.replace('.git', '') || repo_url,
        "repo_url": repo_url,
        "repo_type": get_repo_type(repo_url)
      }
      bookmarks.toggle(bookmark);
    }
</script>

<div class="topbar">
	<!-- Logo / Home Link -->
	<a href="/" class="logo-section">
		<img src="/submark.png" alt="logo" class="logo-img" color=""/>
	</a>

  <!-- repo pathway display -->
  <div class="repo-pathway">
      {repo_path}
  </div>

	<!-- bookmark toggle -->
	<button
		type="button"
		class="bookmark-btn"
		onclick={toggle_bookmark}
		aria-pressed={bookmarked}
	>
		<Icon
			icon={bookmarked ? 'tabler:star-filled' : 'tabler:star'}
			class="icon-medium"
		/>
	</button>
</div>



<style>
	/* topbar wrapper fixed at top-left */
	.topbar {
		display: flex;
		align-items: center;
		height: inherit;
	}

	.logo-section {
		display: flex;
		align-items: center;
		margin-right: 0.8125rem;
	}

	.logo-img {
		height: 0.9375rem;
		/* margin-top: 2px; */

	}

	.repo-pathway {
		font-family: 'DM Mono', monospace;
		font-size: 1rem;
		color: var(--label-primary);
		white-space: nowrap;
		margin-right: 0.5rem;
	}

	.bookmark-btn {
		background: none;
		border: none;
		padding: 0px;
		cursor: pointer;
		display: flex;
		color: var(--label-primary)
	}
</style>
