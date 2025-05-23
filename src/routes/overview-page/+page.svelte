<!-- src/routes/overview-page/+page.svelte -->
<script lang="ts">
    //import shared layout components
    import Banner from '$lib/components/overview-page/banner.svelte';
    import UserMenu from '$lib/components/overview-page/user-menu.svelte';
    // import OverviewGraph from '../../components/overview-page/OverviewGraph.svelte';

    //page-specific data or stores
    import { page } from '$app/stores';
</script>

<!-- Shared header/navigation -->
<Banner />
<UserMenu />

<!-- Main page content -->
<main class="main">
    <section class="overview-stats">
    <!-- Replace with your actual graph/component -->
    <!-- <OverviewGraph /> -->
    </section>
</main>


<style>
/* Scoped styles for overview page */
.main {
    height: calc(100vh - 4.1875rem);
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 1rem;
}

.overview-stats {
    width: 100%;
    max-width: 800px;
    background: var(--background-secondary);
    border-radius: 8px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}
</style>
