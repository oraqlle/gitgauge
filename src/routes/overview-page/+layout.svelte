<script lang="ts">

    //import layout components
    import Banner from '$lib/components/overview-page/Banner.svelte';
    import UserMenu from '$lib/components/overview-page/UserMenu.svelte';
    import Sidebar from '$lib/components/global/sidebar.svelte';
    import ContributorGrid from '$lib/components/overview-page/ContributorGrid.svelte';
    import Heading from '$lib/components/overview-page/Heading.svelte';

    // dummy data for demo
    import { current_repo } from '$lib/stores/repo';

    let profileImageURL = '/mock_profile_img.png';
    let userName = 'Baaset Moslih';


</script>

<header class="header">
    <Banner repo_url={$current_repo.repo_url} repo_path={$current_repo.repo_path}/>
    <UserMenu {userName} {profileImageURL} />
</header>

<main class="body">
    <slot />
    
</main>

<Sidebar />

<style>

.header {
    padding-left: 2rem;
    padding-right: 2rem;
    padding-top: 2rem;
    margin-bottom: 0.8125rem;
    height: 1.375rem;
    display: flex;
    justify-content: space-between;
}

.body {
    height: calc(100vh - 1.375rem)
}
</style>
