<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Icon from "@iconify/svelte";
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
    let selected: RepoOption | null = null;
  
    const options: RepoOption[] = [
      { label: "GitHub", icon: "brand-github" },
      { label: "GitLab", icon: "brand-gitlab" },
      { label: "Local", icon: "folder-code" }
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
          <h6 class="white body-accent">{userName}</h6>
          <img
            src="{profileImageURL}"
            alt="Profile"
            class="profile-img"
          />
  
          <button type='button' class="hamburger-btn" on:click={toggleSidebar}>
            <Icon
              icon={"tabler:menu-2"}
              class="icon-medium"
              style="color: white"
            />
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
              <Icon
                icon={`tabler:${selected.icon}`}
                class="icon-medium"
                style="color: white"
              />
              <h6 class="display-body white dropdown-text" >{selected.label}</h6>
            </div>
          {:else}
            <h6 class="display-body white">Select an option</h6>
          {/if}
          <img src="/dropdown_icon.png" alt="dropdown icon">
        </button>
      
        {#if dropdownOpen}
          <div class="dropdown-content">
            {#each options as option}
              <button class="dropdown-option" on:click={() => selectOption(option)}>
                <Icon
                  icon={`tabler:${option.icon}`}
                  class="icon-medium"
                  style="color: white"
                />
                <h6 class="display-body white dropdown-text">{option.label}</h6>
              </button>
            {/each}
          </div>
        {/if}
      </div>
      
      <!-- Repo link -->
      <div class="repo-link">
        <input
          class="repo-textbox display-body"
          type="text"
          placeholder="enter a git repo..."
          bind:value={inputRepo}
          on:keypress={handleKeyPress}
        />
        <button class="repo-button" on:click={handleSubmit}>
          <Icon
              icon={"tabler:circle-arrow-right"}
              class="icon-medium"
              style="color: white"
            />
        </button>
      </div>
      
  
      <!-- Repo link list -->
      <div class="repo-bookmark-list">
        {#each bookmarked_repo as bookmark (bookmark.repo_url)}
          <button class="repo-list-btn" type="button">
            <h6 class="display-body repo-list-text white">{bookmark.repo_url}</h6>
          </button>
        {/each}
      </div>
    </div>
  </main>

  <style>
    .container {
      margin: 0;
      padding: 1rem 2rem;
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      min-height: auto;
    }
  
    .title {
      font-size: 2rem;
      font-weight: bold;
      margin-bottom: 2rem;
      color: #f6f6f6;
    }
  
    .header-row {
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 2rem;
    }
  
    .branch-select {
      background-color: #333;
      color: #f6f6f6;
      border: 1px solid #444;
      border-radius: 6px;
      padding: 8px 12px;
      font-size: 14px;
      cursor: pointer;
      outline: none;
      transition: border-color 0.2s;
    }
  
    .branch-select:hover {
      border-color: #666;
    }
  
    .branch-select:focus {
      border-color: #888;
    }
  
    /* Sidebar styles */
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
  
    .white {
      color: var(--white);
    }
  
    .label-secondary {
      color: var(--label-secondary);
    }
  </style>
