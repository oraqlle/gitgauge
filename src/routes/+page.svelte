<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  interface RepoOption {
    value: string,
    label: string,
    icon: string
  }

  let sidebarOpen = false;
  let profileImageURL = "/mock_profile_img.png";
  let userName = "Baaset Moslih"

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  let bookmarked_repo = [
    {repo_name: "fit3170-A1", repo_url: "https://github.com/user/fit3170-A1.git"},
    {repo_name: "this-is-a-repo", repo_url: "https://gitlab.com/abc0012/this-is-a-repo.git"},
    {repo_name: "project", repo_url: "https://github.com/example-org/project.git"},
    {repo_name: "another-project", repo_url: "https://gitlab.com/example-org/another-project.git"},
    {repo_name: "assignment", repo_url: "https://gitlab.com/xyz0001/assignment.git"}
  ];

  
</script>

<header>
  <div class="container">
    <div class="header-content">
      <div class="logo-section">
        <a href="/" class="cursor-pointer">
          <img class="logo-img" src="/secondary_logo.png" alt="Your Company">
        </a>
      </div>

      <div class="user-section">
        <h1 class="user-name body-accent">{userName}</h1>
        <img
          src="{profileImageURL}"
          alt="Profile"
          class="profile-img"
        />

        <button class="hamburger-btn" on:click={toggleSidebar}>
          <img src="/hamburger_menu.png" alt="hamburger_menu"/>
        </button>
      </div>
    </div>
  </div>
</header>

<main>
  

  
</main>

<!-- Sidebar -->
<div class={`sidebar ${sidebarOpen ? 'open' : 'closed'}`}>
  <div class="sidebar-header">
    <div class="sidebar-title">
      <img src="/setting_icon.png" alt="setting icon">
      <h1 class="title-1 sidebar-title-text">settings</h1>
    </div>
    <button class="close-button" on:click={toggleSidebar}>
      <img src="/closing_button.png" alt="closing button">
    </button>
  </div>

  <div class="bookmark-list">
    <div class="bookmark-header">
      <img src="/bookmark.png" alt="bookmark icon">
      <h1 class="large-body-text-bold bookmark-text">bookmarks</h1>
    </div>
    
    {#each bookmarked_repo as repo (repo.repo_url)}
      <button class="bookmark-item">
        <h1 class="large-body-text repo-name">{repo.repo_name}</h1>
        <h1 class="body repo-url">{repo.repo_url}</h1>
      </button>
    {/each}
  </div>
</div>



<style>
.container {
  margin-left: auto;
  margin-right: auto;
  padding-left: 26px;
  padding-right: 26px;
  padding-top: 26px;
  padding-bottom: 26px;
}

.header-content {
  position: relative;
  display: flex;
  height: 22px;
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

.user-name {
  color: white;
}

.profile-img {
  height: 22px;
  width: 22px;
  margin-left: 13px;
  margin-right: 13px;
  border-radius: 9999px;
  object-fit: cover;
}

.hamburger-btn {
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
}

.sidebar {
  position: fixed;
  top: 0;
  right: 0;
  width: 359px;
  height: 100%;
  padding-right: 26px;
  padding-left: 25px;
  padding-top: 26px;
  background-color: rgba(34, 34, 34, 0.65);
  backdrop-filter: blur(12px);
  border-width: 1px;
  border-style: solid;
  border-color: #383838;
  border-right: none;
  border-radius: 0.5rem;
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
  height: 29px;
  margin-bottom: 26px;
}

.sidebar-title {
  display: flex;
  height: 29px;
}

.sidebar-title-text {
  margin: auto;
  margin-left: 6px;
}

.close-button {
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
}

.bookmark-list {
  display: grid;
  grid-template-columns: 1fr;
  gap: 13px;
}

.bookmark-header {
  display: flex;
  align-items: center;
  height: 22px;
}

.bookmark-text {
  padding-left: 6px;
}

.bookmark-item {
  display: flex;
  flex-direction: column;
  text-align: left;
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
}

.repo-name, .repo-url {
  margin-top: 0px;
  margin-bottom: 0px
}

</style>