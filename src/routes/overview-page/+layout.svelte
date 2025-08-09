<script lang="ts">
    import { page } from "$app/state";
    import Banner from "$lib/components/overview-page/Banner.svelte";
    import UserMenu from "$lib/components/overview-page/UserMenu.svelte";
    import Sidebar from "$lib/components/global/Sidebar.svelte";

    let profile_image_url = "/mock_profile_img.png";
    let user_name = "Baaset Moslih";

    let { children } = $props();

    let repo_url = $derived(page.state.repo_url);
    let repo_path = $derived(page.state.repo_path);
</script>


<main class="page">
    <header class="header">
        <Banner {repo_url} {repo_path} />
        <UserMenu username={user_name} {profile_image_url} />
    </header>
    {@render children()}
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
</style>
