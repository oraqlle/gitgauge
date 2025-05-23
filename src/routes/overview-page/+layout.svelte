<script lang="ts">
    import { page }     from '$app/stores';
    import { derived }  from 'svelte/store';
    import Icon         from '@iconify/svelte';

    //import layout components
    import Banner from '$lib/components/overview-page/banner.svelte';
    import UserMenu from '$lib/components/overview-page/user-menu.svelte';
    import Sidebar from '$lib/components/global/sidebar.svelte';

    import { toggleSidebar, sidebarOpen } from '$lib/stores/sidebar';
    import { onDestroy } from 'svelte';

    

    let institutionName = 'Monash'
    let unitCode = 'FIT3170'
    let repoName = '2025W1-Commitment';
    $: repoPath = `/${institutionName}/${unitCode}/${repoName}`;

    let open = false;
    const unsubscribe = sidebarOpen.subscribe(value => open = value);
    onDestroy(unsubscribe);

    //dummy input variables
    //dummy values
    let profileImageURL = '/mock_profile_img.png';
    let userName = 'Baaset Moslih';


</script>
<!-- universal page elements -->
<Banner {repoPath}/>
<UserMenu {userName} {profileImageURL}/>
<Sidebar />

<!-- page-specific content -->
<main class="main">
    <slot />
    <div class="body">
        Hello World!
    </div>
</main>
  