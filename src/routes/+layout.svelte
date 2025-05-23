<script lang="ts">
    import { page }     from '$app/stores';
    import { derived }  from 'svelte/store';
    import Icon         from '@iconify/svelte';

    //import layout components
    import Banner from '../components/overview-page/banner.svelte';
    import UserMenu from '../components/overview-page//user-menu.svelte';
    // import OverviewGraph    from '$library/overview-graph.svelte';

    //dummy input variables
    let profileImageURL = '/mock_profile_img.png';
    let userName = 'Baaset Moslih';

    let institutionName = 'Monash'
    let unitCode = 'FIT3170'
    let repoName = '2025W1-Commitment';
    $: repoPathway = `/${institutionName}/${unitCode}/${repoName}`;

</script>
<!-- universal page elements -->
<Banner />
<UserMenu />

<!-- page-specific content -->
<main class="main">
    <!-- main stuffffff -->
    <slot />
</main>
  