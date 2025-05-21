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
  
  <script>
  import { onMount } from 'svelte';
  import * as echarts from 'echarts';
  import { get } from 'svelte/store';
  import { users, type User } from '../data/dummyData';
  import { Size } from '@tauri-apps/api/dpi';

  let chartContainer: HTMLElement;
  let chart: echarts.ECharts;
  let isJittered: boolean = false; // State for jitter effect

  // Add branch selection state
  let selectedBranch = 'all';
  
  // Extract unique branches from dummy data and add 'all' option
  const branches = ['all', ...new Set(users.flatMap(user => 
    user.commits.map(commit => commit.branch)
  ))];

  // Filter users based on selected branch
  $: filteredUsers = selectedBranch === 'all' 
    ? users 
    : users.map(user => ({
        ...user,
        commits: user.commits.filter(commit => commit.branch === selectedBranch)
      })).filter(user => user.commits.length > 0);

  function getAverageCommits(users: User[]): number {
    if (users.length === 0) return 0;
    const commit_mean: number = users.reduce((acc, curr) => {
        return acc + curr.commits.length;
    }, 0) / users.length;

    return commit_mean;
  }

  function getSD(users: User[]): number {
    if (users.length === 0) return 0;
    let commits: number[] = [];

    // Get the list of total commits for each user
    users.forEach(user => {
      commits.push(user.commits.length);
    })

    // Creating the mean with Array.reduce
    const n: number = users.length;
    const mean = getAverageCommits(users);

    const variance: number = commits.reduce((acc: number, val: number) => acc + Math.pow(val - mean, 2), 0) / n;
    
    return Math.sqrt(variance);
  }

  // Calculate SD & Mean
  function getRefPoints(mean: number, sd: number): number[] {
    if (sd === 0) return [mean, mean, mean, mean, mean];
    return [
      (mean - (2 * sd)),
      (mean - sd),
      mean,
      (mean + sd),
      (mean + (2 * sd))
    ];
  }

  function getRandomHexColor(): string {
    const randomColor = Math.floor(Math.random() * 0xffffff);
    return `#${randomColor.toString(16).padStart(6, '0')}`;
}

  function getUserCommits(users: User[]) {
    if (users.length === 0) return [];
    let userTotalCommits: any[] = [];
    users.forEach(user => { 
      userTotalCommits.push({
        username: user.username,
        image: user.image,
        numCommits: user.commits.length
      })
    })
    // Sort by number of commits
    const sortedCommits = userTotalCommits.sort((a, b) => a.numCommits - b.numCommits);
    
    // Group by numCommits and apply horizontal offset
    const groups = new Map<number, any[]>();
    sortedCommits.forEach(user => {
      if (!groups.has(user.numCommits)) {
        groups.set(user.numCommits, []);
      }
      groups.get(user.numCommits)!.push(user);
    });

    // Apply horizontal offset to overlapping points
    const result: any[] = [];
    groups.forEach((users, commits) => {
      if (users.length === 1) {
        result.push(users[0]);
      } else {
        users.forEach((user, index) => {
          result.push({
            ...user,
            offsetIndex: index - (users.length - 1) / 2
          });
        });
      }
    });

    return result;
  }

  // Reactive declarations for derived values
  $: commit_mean = getAverageCommits(filteredUsers);
  $: sd = getSD(filteredUsers);
  $: refPointValues = getRefPoints(commit_mean, sd);
  $: filteredPeople = getUserCommits(filteredUsers);

  // Reference points for vertical lines
  $: refPoints = (sd === 0)
    ? [{ label: 'mean', value: refPointValues[2] }]
    : [
        { label: '-2σ', value: refPointValues[0] },
        { label: '-σ', value: refPointValues[1] },
        { label: 'mean', value: refPointValues[2] },
        { label: '+σ', value: refPointValues[3] },
        { label: '+2σ', value: refPointValues[4] }
      ];

  // Dummy data for people (aggregate x-values)
  const people = getUserCommits(users);
  
  // [
  //   { name: 'A', color: '#6fcf97', x: 15 },
  //   { name: 'B', color: '#e0e0e0', x: 20 },
  //   { name: 'C', color: '#bb6bd9', x: 28 },
  //   { name: 'D', color: '#6fcf97', x: 38 },
  //   { name: 'E', color: '#f2994a', x: 40 },
  //   { name: 'F', color: '#2d9cdb', x: 52 },
  //   { name: 'G', color: '#f2994a', x: 54 },
  //   { name: 'H', color: '#6fcf97', x: 60 },
  //   { name: 'I', color: '#27ae60', x: 70 },
  //   { name: 'J', color: '#bb6bd9', x: 80 },
  //   { name: 'K', color: '#b7e4c7', x: 85 }
  // ];

  function updateGraphics() {
    if (!chart) return;
    
    // Use y=2 as the top of the y-axis for gridTop
    const gridTop = chart.convertToPixel({gridIndex: 0}, [0, 2])[1];
    const xAxisY = chart.convertToPixel({gridIndex: 0}, [0, 0])[1];
    
    // Create graphics for reference lines
    const refLineGraphics = refPoints.map(ref => {
      const x = chart.convertToPixel({gridIndex: 0}, [ref.value, 0])[0];
      return {
        type: 'group',
        children: [
          {
            type: 'line',
            shape: {
              x1: x,
              y1: gridTop,
              x2: x,
              y2: xAxisY
            },
            style: {
              stroke: '#fff',
              lineDash: [4, 4],
              lineWidth: 1,
              opacity: 0.5
            },
            silent: true
          },
          {
            type: 'text',
            style: {
              text: ref.label,
              fontSize: 12,
              fill: '#fff',
              font: 'bold 16px sans-serif',
              textAlign: 'center',
              textVerticalAlign: 'bottom'
            },
            x: x,
            y: gridTop - 8 
          }
        ]
      };
    });

    // Create graphics for user images with fixed pixel offset
    const userGraphics = filteredPeople.map((person) => {
      // Convert the base position to pixels
      const [baseX, y] = chart.convertToPixel({gridIndex: 0}, [person.numCommits, 1]);
      // Apply fixed 16px offset if there's an offsetIndex
      const x = baseX + (person.offsetIndex ? person.offsetIndex * 16 : 0);
      
      return {
        type: 'group',
        children: [
          {
            type: 'image',
            style: {
              image: person.image,
              width: 40,
              height: 40
            },
            x: x - 20, // Center the image
            y: y - 20, // Center the image
            silent: false,
            clipPath: {
              type: 'circle',
              shape: {
                cx: 20,
                cy: 20,
                r: 20
              }
            }
          }
        ]
      };
    });

    // Combine all graphics
    chart.setOption({ 
      graphic: [...refLineGraphics, ...userGraphics]
    });

    console.log(filteredPeople);
  }

  // Calculate min and max number of commits for filteredPeople
  $: minCommits = filteredPeople.length > 0 ? Math.min(...filteredPeople.map(p => p.numCommits)) : 0;
  $: maxCommits = filteredPeople.length > 0 ? Math.max(...filteredPeople.map(p => p.numCommits)) : 1;
  $: xMin = minCommits === maxCommits ? minCommits - 1 : minCommits - 1;
  $: xMax = minCommits === maxCommits ? maxCommits + 1 : maxCommits + 1;

  // Update the chart config to use xMin and xMax
  $: if (chart && selectedBranch) {
    const option = {
      backgroundColor: '#222',
      grid: {
        top: '10%',
        bottom: '25%',
        left: 40,
        right: 40,
        containLabel: false
      },
      xAxis: {
        type: 'value',
        min: xMin,
        max: xMax,
        name: 'Total Commits',
        nameLocation: 'middle',
        nameGap: 40,
        axisLine: {
          lineStyle: {
            color: '#fff',
            width: 2
          }
        },
        axisLabel: {
          color: '#fff',
          fontSize: 16,
          margin: 16
        },
        splitLine: { show: false },
        axisTick: {
          lineStyle: {
            color: '#fff',
            width: 2
          }
        },
        position: 'bottom'
      },
      yAxis: {
        show: false,
        min: 0,
        max: 2,
      },
      series: [
        {
          type: 'scatter',
          data: filteredPeople.map(p => [p.numCommits, 1]),
          symbolSize: 0,
          z: 3
        }
      ],
      graphic: []
    };

    chart.setOption(option, true); // true for not merging with previous options
    updateGraphics();
  }

  onMount(() => {
    chart = echarts.init(chartContainer);
    
    window.addEventListener('resize', () => {
      chart.resize();
      updateGraphics();
    });

    return () => {
      window.removeEventListener('resize', updateGraphics);
      chart.dispose();
    };
  });

  // 1. Total Commits for a user
  function getUserTotalCommits(user: User): number {
    return user.commits.length;
  }

  // 2. Total Lines of Code (additions + deletions) for a user
  function getUserTotalLinesOfCode(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
      commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
        fileAcc + file.added + file.deleted, 0
      ), 0
    );
  }

  // 3. Lines per Commit for a user
  function getUserLinesPerCommit(user: User): number {
    const totalCommits = getUserTotalCommits(user);
    const totalLines = getUserTotalLinesOfCode(user);
    return totalCommits === 0 ? 0 : Math.round(totalLines / totalCommits);
  }

  // 4. Commits per Day for a user
  function getUserCommitsPerDay(user: User): number {
    const allDates = user.commits.map(commit => commit.date);
    const uniqueDates = new Set(allDates);
    const totalCommits = getUserTotalCommits(user);
    return uniqueDates.size === 0 ? 0 : +(totalCommits / uniqueDates.size).toFixed(2);
  }

  // 5. Total Additions for a user
  function getUserTotalAdditions(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
      commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
        fileAcc + file.added, 0
      ), 0
    );
  }

  // 6. Total Deletions for a user
  function getUserTotalDeletions(user: User): number {
    return user.commits.reduce((commitAcc, commit) =>
      commitAcc + commit.filesChanged.reduce((fileAcc, file) =>
        fileAcc + file.deleted, 0
      ), 0
    );
  }

  // Add this function after the other utility functions
  function calculateScalingFactor(numCommits: number): number {
    if (sd === 0) return 1.0;
    const zScore = (numCommits - commit_mean) / sd;
    const EPSILON = 1e-6;
    if (Math.abs(zScore) < EPSILON) {
      return 1.0;
    } else if (Math.abs(zScore) <= 1) {
      return 1.0;
    } else if (zScore < -1 && zScore >= -2) {
      return 0.9;
    } else if (zScore > 1 && zScore <= 2) {
      return 1.1;
    } else {
      return zScore < 0 ? 0.8 : 1.2;
    }
  }

  // Update the peopleWithMetrics to use filteredUsers
  $: peopleWithMetrics = filteredPeople.map(person => {
    const user = filteredUsers.find(u => u.username === person.username);
    const scalingFactor = calculateScalingFactor(person.numCommits);
    return {
      ...person,
      totalLinesOfCode: user ? getUserTotalLinesOfCode(user) : 0,
      linesPerCommit: user ? getUserLinesPerCommit(user) : 0,
      commitsPerDay: user ? getUserCommitsPerDay(user) : 0,
      totalAdditions: user ? getUserTotalAdditions(user) : 0,
      totalDeletions: user ? getUserTotalDeletions(user) : 0,
      scalingFactor: scalingFactor.toFixed(1)
    };
  });
</script>

<main class="container">
  <div class="header-row">
    <h1 class="title">Overview Page</h1>
    <select 
      bind:value={selectedBranch} 
      class="branch-select"
    >
      {#each branches as branch}
        <option value={branch}>{branch === 'all' ? 'All Branches' : branch}</option>
      {/each}
    </select>
  </div>
  <div bind:this={chartContainer} class="chart-container" style="width: 100%; height: 200px;"></div>
  <div class="cards-row">
    {#each peopleWithMetrics as person, i}
      <div class="profile-card">
        <div class="profile-header-row">
          <img class="profile-avatar" src={person.image} alt={person.username} />
          <div class="profile-header-main">
            <div class="profile-header-info">
              <div class="profile-title">{person.username}</div>
              <div class="profile-scaling">scaling: {person.scalingFactor}</div>
            </div>
            <div class="profile-metrics-main">
              <div class="profile-metrics-row">
                <span>{person.numCommits} commits</span>
                <span class="metrics-separator">&nbsp;&nbsp;</span>
                <span>{person.totalLinesOfCode} lines of code</span>
              </div>
              <div class="profile-metrics-row">
                <span>{person.linesPerCommit} lines/commit</span>
                <span class="metrics-separator">&nbsp;&nbsp;</span>
                <span>{person.commitsPerDay} commits/day</span>
              </div>
              <div class="profile-metrics-row">
                <span class="metrics-additions">{person.totalAdditions}++ additions</span>
                <span class="metrics-separator">&nbsp;&nbsp;</span>
                <span class="metrics-deletions">{person.totalDeletions}-- deletions</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    {/each}
  </div>
</main>

<!-- Sidebar -->
<div class={`sidebar ${sidebarOpen ? 'open' : 'closed'}`}>
  <div class="sidebar-header">
    <div class="sidebar-title">
      <Icon
        icon={"tabler:chart-line"}
        class="icon-large"
        style="color: white"
      />
      <h1 class="title sidebar-title-text white">settings</h1>
    </div>
    <button class="close-button" on:click={toggleSidebar}>
      <Icon
        icon={"tabler:x"}
        class="icon-medium"
        style="color: white"
      />
    </button>
  </div>

  <div class="bookmark-list">
    <div class="bookmark-header">
      <Icon
        icon={"tabler:star-filled"}
        class="icon-medium"
        style="color: white"
      />
      <h2 class="heading-1 bookmark-text white">bookmarks</h2>
    </div>
    
    {#each bookmarked_repo as repo (repo.repo_url)}
      <button class="bookmark-item">
        <h6 class="heading-2 repo-name label-secondary">{repo.repo_name}</h6>
        <h6 class="caption repo-url label-secondary">{repo.repo_url}</h6>
      </button>
    {/each}
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
  grid-template-columns: 13rem 35.5rem; /* 2 columns */
  grid-template-rows: repeat(2, auto);  /* 2 rows */
  column-gap: 1rem;
  row-gap: 10px;
}

/* REPO DROPDOWN */
.dropdown {
  position: relative;
  width: 13rem;
  height: 2.625em;
  border-top-left-radius: 12px;
  border-top-right-radius: 12px;
}

.dropdown-btn {
  width: 100%;
  height: inherit;
  padding: 0.625rem 0.75rem 0.625rem 1rem;
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
  padding: 0.625rem 0.75rem 0.625rem 1rem;
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
  margin-left: 0.5rem;
}

/* REPO TEXTBOX */
.repo-link {
  height: 1.5rem;
  width: 33rem;
  display: flex;
  justify-content: start;
  align-items: center;
  background-color: #222;
  padding: 0.5625rem 1.125rem 0.5625rem 1.5rem;
  border-radius: 12px;
}

.repo-textbox {
  flex: 1;
  margin-right: 0.5rem;
  background-color: #222;
  border: none;
  height: 24px;
  padding: 0px;
  width: 350px;
  color: white;
}

.repo-textbox::placeholder {
  font-size: 1.063rem;
  font-family: DM Sans;
  font-weight: 400;
  word-wrap: break-word;
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
  padding-left: 1.5rem;
  padding-right: 1.5rem;
  margin: 0px;
  /* width: 693px; */
  display: grid;
  grid-template-columns: 32.5rem;
  grid-template-rows: repeat(5);
  row-gap: 1rem;

  /* let the list overflow and can be scrolll */
  max-height: 10.875rem;         /* adjust height to fit your layout */
  overflow-y: auto;          /* enables vertical scrolling */
  overflow-x: hidden;
  /* padding-bottom: 84px;  */
  scroll-padding-bottom: 10.875rem;

  scrollbar-width: none; 
  -ms-overflow-style: none;

  -webkit-mask-image: linear-gradient(to bottom, black 0%, rgba(0,0,0,0.2) 80%, transparent 100%);
  mask-image: linear-gradient(to bottom, black 0%, rgba(0,0,0,0.2) 80%, transparent 100%);
  mask-size: 100% 100%;
  mask-repeat: no-repeat;
}

.repo-list-btn {
  height: 22px;
  width: inherit;
  background-color: #181818;
  border: none;
  margin: none;
  padding: 0px;
  text-align: left;
  cursor: pointer;
}

.repo-list-text {
  height: inherit;
  margin: 0px;
}
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #f6f6f6;
    background-color: #222;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  .container {
    margin: 0;
    padding: 1rem 2rem;  /* Reset to normal padding */
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
  .chart-container {
    margin-top: 10rem;  /* Add margin to push the chart down */
  }
  .cards-row {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    gap: 2rem;
    justify-content: center;
    margin-top: 3rem;
  }

  .profile-card {
    display: flex;
    flex-direction: column;
    background: var(--Fill-Tint-00, rgba(31, 31, 31, 0.90));
    border-radius: 12px;
    padding: 20px 28px;
    min-width: 320px;
    min-height: 70px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  }

  .profile-header-row {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    gap: 16px;
    margin-bottom: 0;
  }

  .profile-header-main {
    display: flex;
    flex-direction: column;
    flex: 1;
    width: 100%;
  }

  .profile-header-info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin-bottom: 1px;
  }

  .profile-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    flex-shrink: 0;
    object-fit: cover;
    background: #ccc;
  }

  .profile-title {
    color: #fff;
    font-size: 18px;
    font-family: DM Sans, Inter, Arial, sans-serif;
    font-weight: 600;
    margin-bottom: 2px;
    text-align: left;
  }

  .profile-scaling {
    color: #A3A3A3;
    font-size: 14px;
    font-family: DM Sans, Inter, Arial, sans-serif;
    font-weight: 400;
    text-align: left;
    margin-bottom: 8px;
  }

  .profile-metrics-main {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .profile-metrics-row {
    display: flex;
    flex-direction: row;
    align-items: center;
    font-size: 15px;
    color: #ccc;
    margin-bottom: 2px;
    font-family: DM Sans, Inter, Arial, sans-serif;
  }

  .metrics-separator {
    width: 16px;
    display: inline-block;
  }

  .metrics-additions {
    color: #4ade80;
    font-weight: bold;
  }

  .metrics-deletions {
    color: #fb7185;
    font-weight: bold;
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
</style>