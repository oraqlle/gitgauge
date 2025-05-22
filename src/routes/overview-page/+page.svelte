<script>
    import { page } from '$app/stores';
    import { derived } from 'svelte/store';
  
    const showRepoPage = derived(page, $page => $page.url.searchParams.get('repo') === 'true');
  </script>
  
  {#if $showRepoPage}
    <div class="repo-container">
      <h1>Repository Details</h1>
      <p>This is the repo-specific view.</p>
    </div>
  {:else}
    <div class="landing-container">
      <h1>Welcome to GitGauge</h1>
      <a href="/?repo=true">Go to Repo</a>
    </div>
  {/if}
  