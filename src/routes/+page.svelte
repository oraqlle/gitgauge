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
  <div class="mx-auto px-[26px] py-[26px]">
    <div class="relative flex h-[22px] items-center justify-between">
      <div class="flex">
        <a href="/" class="cursor-pointer">
          <img class="h-[20px] w-auto" src="/secondary_logo.png" alt="Your Company">
        </a>
      </div>

      <div class="flex items-center justify-between py-2">
        <!-- Profile Image -->
        <h1 class="mt-[3px] body-accent text-white">{userName}</h1>
        <img
          src={profileImageURL}
          alt="Profile"
          class="h-[22px] w-[22px] mx-[13px] rounded-full object-cover"
        />
      
        <!-- Hamburger Button -->
        <button
          class="cursor-pointer"
          on:click={toggleSidebar}
        >
          <!-- Hamburger Icon -->
          <img src="/hamburger_menu.png" alt="hamburger_menu"/>
        </button>
      </div>
    </div>
  </div>
</header>

<main>
  

  
</main>

<!-- Sidebar -->
<div class={`fixed top-0 right-0 w-[359px] h-full pr-[26px] pl-[25px] pt-[26px] bg-[#222222]/65 backdrop-blur-md rounded-lg border-[1px] border-r-0 border-[#383838] shadow-lg z-50 transform transition-transform duration-500 ease-in-out ${sidebarOpen ? 'translate-x-0' : 'translate-x-full'}`}>
  <div class="flex items-start justify-between h-[29px] mb-[26px]">
    <div class="flex h-[29px]">
      <img src="/setting_icon.png" alt="setting icon">
      <h1 class="title-1 ml-[6px] text-white">settings</h1>
    </div>
    <button class="cursor-pointer" on:click={toggleSidebar}>
      <img src="/closing_button.png" alt="closing button">
    </button>
  </div>

  <div class="grid grid-cols-1 gap-[13px]">
    <div class="flex items-center h-[22px]">
      <img src="/bookmark.png" alt="bookmark icon">
      <h1 class="large-body-text-bold h-[22px] ml-[6px] text-white">bookmarks</h1>
    </div>
    
    {#each bookmarked_repo as repo (repo.repo_url)}
      <button class="flex flex-col cursor-pointer text-left">
        <!-- <button class="cursor-pointer"> -->
          <h1 class="large-body-text text-[#8f8f8f]">{repo.repo_name}</h1>
          <h1 class="body text-[#8f8f8f]">{repo.repo_url}</h1>
        <!-- </button> -->
      </button>
    {/each}
  </div>

</div>
