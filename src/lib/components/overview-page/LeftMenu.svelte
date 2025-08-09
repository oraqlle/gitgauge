<script lang="ts">
    let {
		content
    }: {
		content?: any;
    } = $props();
</script>

<div class="topbar">
	{#if content}
		<!-- Logo / Home Link -->
		<a href="/" class="logo-section">
			<img src="/submark.png" alt="logo" class="logo-img" color=""/>
		</a>
		{@render content()}
	{:else}
		<!-- Logo / Home Link -->
		<a href="/" class="logo-section">
			<img src="/submark.png" alt="logo" class="logo-img" color=""/>
		</a>
	{/if}
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
	}
</style>
