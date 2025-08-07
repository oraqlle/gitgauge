<script lang="ts">
    import { page } from "$app/state";

    //import layout components
    import Banner from '$lib/components/overview-page/Banner.svelte';
    import UserMenu from '$lib/components/overview-page/UserMenu.svelte';
    import Sidebar from '$lib/components/global/sidebar.svelte';

    let profile_image_url = '/mock_profile_img.png';
    let user_name = 'Baaset Moslih';
     
    let repo_url = $derived(page.state.repo_url);
    let repo_path = $derived(page.state.repo_path);
    let repo_type = $derived(page.state.repo_type);

</script>

<header class="header">
    <Banner repo_url={repo_url} repo_path={repo_path}/>
    <UserMenu username={user_name} profile_image_url={profile_image_url} />
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
