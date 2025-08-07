<script>
    import Heading from "$lib/components/overview-page/Heading.svelte";
    import CommitGraph from "$lib/components/overview-page/CommitGraph.svelte";
    import { current_repo } from "$lib/stores/repo";
    import ButtonPrimaryMedium from "$lib/components/global/button-primary-medium.svelte";
</script>

<div class="page">
    <Heading
        repo_path={$current_repo.repo_path.split("/").pop() || $current_repo.repo_path}
        repo_type={$current_repo.repo_type}
    />
    <CommitGraph />
    <div class="bottom-container">
        <ButtonPrimaryMedium
            icon="table-import"
            label="Upload Marking Sheet"
        />

        <ButtonPrimaryMedium
            icon="file-download"
            label="Download Marking Sheet"
        />
    </div>
</div>

<style>
    .bottom-container {
        flex-direction: row;
        display: flex;
        align-items: center;
        justify-content: center; 
        padding-top: 2rem;
        padding-bottom: 6rem;
        gap: 1rem;
    }
</style>
