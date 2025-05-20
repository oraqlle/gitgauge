<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  let inputRepo = '';

  interface RepoBookmark {
    repo_name: string,
    repo_url: string
  }

  let sidebarOpen = false;
  let profileImageURL = "/mock_profile_img.png";
  let userName = "Baaset Moslih";

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  let bookmarked_repo: RepoBookmark[] = [
    {repo_name: "fit3170-A1", repo_url: "https://github.com/user/fit3170-A1.git"},
    {repo_name: "this-is-a-repo", repo_url: "https://gitlab.com/abc0012/this-is-a-repo.git"},
    {repo_name: "project", repo_url: "https://github.com/example-org/project.git"},
    {repo_name: "another-project", repo_url: "https://gitlab.com/example-org/another-project.git"},
    {repo_name: "assignment", repo_url: "https://gitlab.com/xyz0001/assignment.git"},
    {repo_name: "assignment", repo_url: "https://gitlab.com/xyz0001/assignment.git2"},
    {repo_name: "assignment", repo_url: "https://gitlab.com/xyz0001/assignment.git3"},
    {repo_name: "assignment", repo_url: "https://gitlab.com/xyz0001/assignment.git4"},
  ];

  // 
  interface RepoOption {
    label: string,
    icon: string
  }
  let dropdownOpen = false;
  let selected: RepoOption | null = { label: "GitHub", icon: "/github.png" };

  const options: RepoOption[] = [
    { label: "GitHub", icon: "/github.png" },
    { label: "GitLab", icon: "/gitlab.png" },
    { label: "Local", icon: "/folder_code.png" }
  ];

  function selectOption(option: RepoOption) {
    selected = option;
    dropdownOpen = false;
  }

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }

  function handleSubmit() {
    if (inputRepo.trim()) {
      goto(`/overview-page?repo=${encodeURIComponent(inputRepo)}`);
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      handleSubmit();
    }
  }


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

<main class="main">
  <div class="repo-start">
    <!-- Repo dropdown -->
    <div class="dropdown">
      <button type="button" class={`dropdown-btn ${dropdownOpen ? 'show' : 'hide'}`} on:click={toggleDropdown}>
        {#if selected}
          <div class="dropdown-show">
            <img class="dropdown-img" src={selected.icon} alt={selected.label} />
            <h1 class="large-body-text dropdown-text" >{selected.label}</h1>
          </div>
        {:else}
          Select an option
        {/if}
        <img src="/dropdown_icon.png" alt="dropdown icon">
      </button>
    
      {#if dropdownOpen}
        <div class="dropdown-content">
          {#each options as option}
            <button class="dropdown-option" on:click={() => selectOption(option)}>
              <img class="dropdown-img" src={option.icon} alt={option.label} />
              <h1 class="large-body-text dropdown-text">{option.label}</h1>
            </button>
          {/each}
        </div>
      {/if}
    </div>
    
    <!-- Repo link -->
    <div class="repo-link">
      <input
        class="repo-textbox large-body-text"
        type="text"
        placeholder="enter a git repo..."
        bind:value={inputRepo}
        on:keypress={handleKeyPress}
      />
      <button class="repo-button" on:click={handleSubmit}>
        <img class="input-icon" src="/repo_confirm.png" alt="repo confirm icon" />
      </button>
    </div>
    

    <!-- Repo link list -->
    <div class="repo-bookmark-list">
      {#each bookmarked_repo as bookmark (bookmark.repo_url)}
        <button class="repo-list-btn" type="button">
          <h1 class="large-body-text repo-list-text">{bookmark.repo_url}</h1>
        </button>
      {/each}
    </div>
  </div>
  
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
/* MAIN PAGE CONTENT */
.main {
  height: calc(100vh - 76px);
  display: flex;
  justify-content: center;
  align-items: center;
}

/* HEADER */
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

/* SIDEBAR */
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


/* REPO SECTION */
.repo-start {
  /* width: ; */
  display: grid;
  grid-template-columns: 161px 441px; /* 2 columns */
  grid-template-rows: repeat(2, auto);  /* 2 rows */
  column-gap: 19.5px;
  row-gap: 10px;
}

/* REPO DROPDOWN */
.dropdown {
  position: relative;
  width: 161px;
  height: 42px;
  border-top-left-radius: 12px;
  border-top-right-radius: 12px;
}

.dropdown-btn {
  width: 100%;
  height: inherit;
  padding: 10px 12px 10px 16px;
  background: #222;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.dropdown-btn.hide {
  border-radius: 12px;
}

.dropdown-btn.show {
  border-top-left-radius: 12px;
  border-top-right-radius: 12px;
}

.dropdown-btn.show::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 1px;
  background-color: #fff;
}

.dropdown-show {
  display: flex; 
  align-items: center;
}

.dropdown-content {
  width: inherit;
  background-color: #222;
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 12px;
}

.dropdown-option {
  width: inherit;
  height: 42px;
  padding: 10px 12px 10px 16px;
  background: #222;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: start;
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 12px;
}

.dropdown-text {
  color: white;
}

.dropdown-img {
  width: 19.5px;
  height: 19.5px;
  margin-right: 8px;
}

/* REPO TEXTBOX */
.repo-link {
  height: 24px;
  width: 399px;
  display: flex;
  justify-content: start;
  align-items: center;
  background-color: #222;
  padding: 9px 18px 9px 24px;
  border-radius: 12px;
}

.repo-textbox {
  flex: 1;
  margin-right: 12px;
  background-color: #222;
  border: none;
  height: 24px;
  padding: 0px;
  width: 350px;
  color: white;
}

.repo-textbox::placeholder {
  font-family: "DM Sans", sans-serif;
  font-size: 17px;
  font-weight: 400;
  color: #8f8f8f;
}

.repo-textbox:focus {
  outline: none;
}

.repo-button {
  background-color: inherit;
  border: none;
  padding: 0px;
  cursor: pointer;
  display: flex;
  align-items: center;
}

/* Repo link list */
.repo-bookmark-list {
  grid-column: 2;
  grid-row: 2;
  padding-left: 24px;
  margin-top: 22px;
  width: 693px;
  display: grid;
  grid-template-columns: 441px;
  row-gap: 13px;

  /* let the list overflow and can be scrolll */
  max-height: 174px;         /* adjust height to fit your layout */
  overflow-y: auto;          /* enables vertical scrolling */
  overflow-x: hidden;
  padding-right: 8px;
  /* padding-bottom: 84px;  */
  scroll-padding-bottom: 174px;

  scrollbar-width: none; 
  -ms-overflow-style: none;

  -webkit-mask-image: linear-gradient(to bottom, black 0%, rgba(0,0,0,0.2) 80%, transparent 100%);
  mask-image: linear-gradient(to bottom, black 0%, rgba(0,0,0,0.2) 80%, transparent 100%);
  mask-size: 100% 100%;
  mask-repeat: no-repeat;
}

.repo-list-btn {
  height: 22px;
  width: 693px;
  background-color: #181818;
  border: none;
  margin: none;
  padding: 0px;
  text-align: left;
  cursor: pointer;
}

.repo-list-text {
  margin: 0px;
}
</style>