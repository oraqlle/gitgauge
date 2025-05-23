<script>
    import { page } from '$app/stores';
    import { derived } from 'svelte/store';
    import Icon from '@iconify/svelte';
  
    let sidebarOpen = false;
    let isBookmarked = false;

    //dummy inputs
    let profileImageURL = '/mock_profile_img.png';
    let userName = 'Baaset Moslih';

    let institutionName = 'Monash'
    let unitCode = 'FIT3170'
    let repoName = '2025W1-Commitment';
    $: repoPathway = `/${institutionName}/${unitCode}/${repoName}`;

    let repoTypeURL = '/github.png';

    function toggleSidebar() {
      sidebarOpen = !sidebarOpen;
    }

    function toggleBookmark() {
        isBookmarked = !isBookmarked
    }

  </script>
  
<header>
    <div class="container">
        <div class="header-content">
            <div class="logo-section">
                <a href="/" class="cursor-pointer">
                <img class="logo-img" src="/submark.png" alt="Your Company" />
                </a>
            </div>
            <div class="title">
                <div class="repo-pathway">
                    {repoPathway}
                </div>
                <button type="button" class="bookmark-btn" on:click={toggleBookmark} aria-pressed={isBookmarked}>
                <Icon icon={isBookmarked ? 'tabler:star-filled' : 'tabler:star'}
                    class="icon-medium"
                    style="color: white"
                  />
              </button>
            </div>

            <div class="user-section">
                <h6 class="white body-accent">{userName}</h6>
                <img src="{profileImageURL}" alt="Profile" class="profile-img" />
                <button type="button" class="hamburger-btn" on:click={toggleSidebar}>
                    <Icon icon="tabler:menu-2" class="icon-medium" style="color: white" />
                </button>
            </div>
        </div>
    </div>
</header>  

<main class="main">
    <!-- TODO: insert your statistics graph & cards here -->
    <section class="overview-stats">
        <!-- e.g. chart component, filters, date picker… -->
    </section>
</main>

<div class={`sidebar ${sidebarOpen ? 'open' : 'closed'}`}>
    <div class="sidebar-header">
            <div class="sidebar-title">
                <Icon icon="tabler:chart-line" class="icon-large" />
                <h1 class="title sidebar-title-text white">settings</h1>
            </div>
        <button class="close-button" on:click={toggleSidebar}>
            <Icon icon="tabler:x" class="icon-medium" style="color: white" />
        </button>
    </div>
</div>

<style>
/* MAIN PAGE CONTENT */
.main {
  height: calc(100vh - 4.1875rem);
  display: flex;
  justify-content: center;
  align-items: center;
}

/* HEADER */
.container {
  padding: 0px;
  margin-left: 2rem;
  margin-right: 2rem;
  margin-top: 2rem;
  margin-bottom: 0.8125rem;
}

.header-content {
  position: relative;
  display: flex;
  height: 1.375rem;
  align-items: center;
  justify-content: space-between;
}

.logo-section {
  display: flex;
}

.logo-img {
  height: 20px;
  width: auto;
}

.user-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 8px;
  padding-bottom: 8px;
}

.white {
  color: var(--white);
}

.label-secondary {
  color: var(--label-secondary);
}

.profile-img {
  height: 1.375rem;
  width: 1.375rem;
  margin-left: 0.8125rem;
  margin-right: 0.8125rem;
  object-fit: cover;
}

.hamburger-btn {
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
  height: 1.375rem;
  width: 1.375rem;
  display: flex;
  justify-content: center;
  align-items: center;
}

/* REPO PATHWAY HEADER */
.repo-pathway {
  margin-left: 1rem; 
  white-space: nowrap;
  color: var(--label-secondary);
  font-size: 1.15rem;
}

.bookmark-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  margin-left: 0.5rem;
  cursor: pointer;
}

.bookmark-icon {
  width: 1.25rem;
  height: 1.25rem;
  display: block;
  color: var(--white);

}

/* SIDEBAR */
.sidebar {
  position: fixed;
  top: 0;
  right: 0;
  width: 18.4375rem;
  height: 100%;
  padding: 2rem;
  border-radius: 8px 0px 0px 8px;
  border-top: solid var(--Label-Tertiary, #747474);
  border-bottom: solid var(--Label-Tertiary, #747474);
  border-left: solid var(--Label-Tertiary, #747474);
  border-width: 0.0625rem;
  background: var(--Background-Tint, rgba(34, 34, 34, 0.70));
  backdrop-filter: blur(16px);
  box-shadow: 0 10px 15px rgba(0, 0, 0, 0.3);
  z-index: 50;
  transform: translateX(100%);
  transition: transform 0.5s ease-in-out;
}

.sidebar.open {
  transform: translateX(0);
}

.sidebar.closed {
  transform: translateX(100%);
}

.sidebar-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  height: 1.8125rem;
  margin-bottom: 1.5rem;
}

.sidebar-title {
  display: flex;
  height: 29px;
}

.sidebar-title-text {
  margin: auto;
  margin-left: 0.375rem;
  height: 1.8125rem
}

.close-button {
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
}

/* GRAPH */
  .overview-stats {
    padding: 2rem;
    background: var(--background-secondary);
    border-radius: 8px;
  }
</style>