<script lang="ts">
    import { page } from '$app/stores';
    import { derived } from 'svelte/store';
  
    const repoParam = derived(page, $page => $page.url.searchParams.get('repo') || '');
  </script>
  
  <h1 class="title-1">Welcome to the New Page</h1>
  <p class="large-body-text">You entered: {$repoParam}</p>
  