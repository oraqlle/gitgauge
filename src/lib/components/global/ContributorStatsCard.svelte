<script lang="ts">
    import ContributorCard from "../global/ContributorCard.svelte";
    export let username: string;
    export let image: string;
    export let numCommits;
    export let totalLinesOfCode;
    export let linesPerCommit;
    export let totalAdditions;
    export let totalDeletions;
    export let scalingFactor
</script>

<!--
@component
This is a contributor card component that displays a user's profile and
contributor statistics.

- Usage:
  ```svelte
    <ContributorStatsCard {...contributor}/>
  ```
-->

<ContributorCard
    username={username}
    image={image}
    scalingFactor={scalingFactor}
    >
    <div class="contents">
        <div>{numCommits} commits</div>
        <div>{totalLinesOfCode} lines of code</div>
        <div>{linesPerCommit} lines/commit</div>
        <div class="body-accent addition">
            {totalAdditions}++ additions
        </div>
        <div class="body-accent deletion">
            {totalDeletions}-- deletions
        </div>
    </div>
</ContributorCard>

<style>
    .contents {
        width: 100%;
        display: flex;
        flex-wrap: wrap;
        gap: 0.3125rem 0.625rem;
        width: 100%;
    }

    .contents > div {
        color : var(--label-secondary);
    }

    .addition {
        color: var(--wonderland--00d498) !important;
    }
    .deletion {
        color: var(--wonderland--ff748b) !important;
    }

</style>