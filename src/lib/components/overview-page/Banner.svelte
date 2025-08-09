<script lang="ts">
    import LeftMenu from "./LeftMenu.svelte";
    import LeftMenuWithRepo from "./LeftMenuWithRepo.svelte";
    import UserMenu from "$lib/components/overview-page/UserMenu.svelte";

    let {
        repo_url,
        repo_path,
        user_name = "Baaset Moslih",
        profile_image_url = "/mock_profile_img.png"
    }: {
        repo_url?: string;
        repo_path?: string;
        user_name?: string;
        profile_image_url?: string;
    } = $props();
</script>

<div class="header">
    {#if repo_url && repo_path}
        <LeftMenuWithRepo {repo_url} {repo_path} />
    {:else}
        <LeftMenu/>
    {/if}
    <UserMenu username={user_name} {profile_image_url} />
</div>

<style>
    .header {
        padding: 2rem 2rem 0.0rem 2rem;
        height: 1.375rem;
        display: flex;
        justify-content: space-between;
    }
</style>