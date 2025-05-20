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

  let chartContainer: HTMLElement;
  let chart: echarts.ECharts;
  let hovermessage: string = '';    //temp to check hover event
  let onclickmessage: string = '';  //temp to check onclick event

  // DUMMY DATA SETUP -- REMOVE LATER 
  type Branch = Readonly<{
    commits: Commit[],
    dateRange: String
  }>

  type File = Readonly<{
    status: String,
    added: number,
    deleted: number,
    changed: number,
    path: String
  }>

  type Commit = Readonly<{
    date: String,
    time: String,
    branch: String,
    filesChanged: File[]
  }>

  type User = Readonly<{
    username: String,
    userEmails: String[],
    commits: Commit[]
  }>;

  // Dummy Data array - of users
  const users: User[] = [
  {
    username: "alice",
    userEmails: ["alice@example.com"],
    commits: [
      {
        date: "2025-05-01",
        time: "01:00 AM",
        branch: "feature/login",
        filesChanged: [
          { status: "modified", added: 10, deleted: 2, changed: 12, path: "src/auth.ts" }
        ]
      },
      {
        date: "2025-05-02",
        time: "02:00 PM",
        branch: "feature/login",
        filesChanged: [
          { status: "added", added: 50, deleted: 0, changed: 50, path: "src/components/LoginForm.tsx" }
        ]
      },
      {
        date: "2025-05-03",
        time: "03:00 AM",
        branch: "feature/dashboard",
        filesChanged: [
          { status: "deleted", added: 0, deleted: 30, changed: 30, path: "src/legacy.ts" }
        ]
      },
      {
        date: "2025-05-04",
        time: "04:00 PM",
        branch: "feature/dashboard",
        filesChanged: [
          { status: "modified", added: 5, deleted: 3, changed: 8, path: "src/dashboard.tsx" }
        ]
      },
      {
        date: "2025-05-05",
        time: "05:00 AM",
        branch: "hotfix/login-crash",
        filesChanged: [
          { status: "added", added: 20, deleted: 0, changed: 20, path: "src/utils/errorHandler.ts" }
        ]
      },
      {
        date: "2025-05-06",
        time: "06:00 PM",
        branch: "hotfix/login-crash",
        filesChanged: [
          { status: "modified", added: 2, deleted: 1, changed: 3, path: "src/auth.ts" }
        ]
      }
    ]
  },
  {
    username: "bob",
    userEmails: ["bob@example.com"],
    commits: [
      {
        date: "2025-05-01",
        time: "01:00 AM",
        branch: "feature/notifications",
        filesChanged: [
          { status: "added", added: 40, deleted: 0, changed: 40, path: "src/notifications.ts" }
        ]
      },
      {
        date: "2025-05-02",
        time: "02:00 PM",
        branch: "feature/notifications",
        filesChanged: [
          { status: "modified", added: 12, deleted: 4, changed: 16, path: "src/notifications.ts" }
        ]
      },
      {
        date: "2025-05-03",
        time: "03:00 AM",
        branch: "feature/settings",
        filesChanged: [
          { status: "added", added: 30, deleted: 0, changed: 30, path: "src/settings.ts" }
        ]
      },
      {
        date: "2025-05-04",
        time: "04:00 PM",
        branch: "feature/settings",
        filesChanged: [
          { status: "modified", added: 7, deleted: 2, changed: 9, path: "src/settings.ts" }
        ]
      },
      {
        date: "2025-05-05",
        time: "05:00 AM",
        branch: "hotfix/email-bug",
        filesChanged: [
          { status: "deleted", added: 0, deleted: 15, changed: 15, path: "src/email.ts" }
        ]
      }
    ]
  },
  {
    username: "carol",
    userEmails: ["carol@example.com"],
    commits: [
      {
        date: "2025-05-01",
        time: "01:00 PM",
        branch: "feature/profile",
        filesChanged: [
          { status: "added", added: 25, deleted: 0, changed: 25, path: "src/profile.ts" }
        ]
      },
      {
        date: "2025-05-02",
        time: "02:00 PM",
        branch: "feature/profile",
        filesChanged: [
          { status: "modified", added: 10, deleted: 5, changed: 15, path: "src/profile.ts" }
        ]
      },
      {
        date: "2025-05-03",
        time: "03:00 PM",
        branch: "feature/preferences",
        filesChanged: [
          { status: "added", added: 18, deleted: 0, changed: 18, path: "src/preferences.ts" }
        ]
      },
      {
        date: "2025-05-04",
        time: "04:00 PM",
        branch: "feature/preferences",
        filesChanged: [
          { status: "modified", added: 6, deleted: 2, changed: 8, path: "src/preferences.ts" }
        ]
      },
      // {
      //   date: "2025-05-05",
      //   time: "05:00 PM",
      //   branch: "hotfix/profile-404",
      //   filesChanged: [
      //     { status: "modified", added: 2, deleted: 2, changed: 4, path: "src/profile.ts" }
      //   ]
      // }
    ]
  },
  {
    username: "dave",
    userEmails: ["dave@example.com"],
    commits: [
      {
        date: "2025-05-01",
        time: "10:00 AM",
        branch: "feature/chat",
        filesChanged: [
          { status: "added", added: 100, deleted: 0, changed: 100, path: "src/chat.ts" }
        ]
      },
      {
        date: "2025-05-02",
        time: "11:00 AM",
        branch: "feature/chat",
        filesChanged: [
          { status: "modified", added: 15, deleted: 5, changed: 20, path: "src/chat.ts" }
        ]
      },
      {
        date: "2025-05-03",
        time: "12:00 PM",
        branch: "feature/chat-ui",
        filesChanged: [
          { status: "added", added: 60, deleted: 0, changed: 60, path: "src/components/ChatUI.tsx" }
        ]
      },
      {
        date: "2025-05-04",
        time: "01:00 PM",
        branch: "feature/chat-ui",
        filesChanged: [
          { status: "modified", added: 5, deleted: 3, changed: 8, path: "src/components/ChatUI.tsx" }
        ]
      },
      {
        date: "2025-05-05",
        time: "02:00 PM",
        branch: "hotfix/chat-scroll",
        filesChanged: [
          { status: "modified", added: 2, deleted: 1, changed: 3, path: "src/chat.ts" }
        ]
      }
    ]
  },
  {
    username: "eve",
    userEmails: ["eve@example.com"],
    commits: [
      {
        date: "2025-05-01",
        time: "09:00 AM",
        branch: "feature/security",
        filesChanged: [
          { status: "added", added: 80, deleted: 0, changed: 80, path: "src/security.ts" }
        ]
      },
      {
        date: "2025-05-02",
        time: "10:00 AM",
        branch: "feature/security",
        filesChanged: [
          { status: "modified", added: 8, deleted: 2, changed: 10, path: "src/security.ts" }
        ]
      },
      {
        date: "2025-05-03",
        time: "11:00 AM",
        branch: "feature/encryption",
        filesChanged: [
          { status: "added", added: 35, deleted: 0, changed: 35, path: "src/encryption.ts" }
        ]
      },
      {
        date: "2025-05-04",
        time: "12:00 PM",
        branch: "feature/encryption",
        filesChanged: [
          { status: "modified", added: 10, deleted: 1, changed: 11, path: "src/encryption.ts" }
        ]
      },
      {
        date: "2025-05-05",
        time: "01:00 PM",
        branch: "hotfix/security-alert",
        filesChanged: [
          { status: "deleted", added: 0, deleted: 5, changed: 5, path: "src/legacySecurity.ts" }
        ]
      },
      {
        date: "2025-05-06",
        time: "02:00 PM",
        branch: "hotfix/security-alert",
        filesChanged: [
          { status: "modified", added: 4, deleted: 0, changed: 4, path: "src/security.ts" }
        ]
      }
    ]
  }
];

  function getAverageCommits(users: User[]): number{
    const commit_mean: number = users.reduce((acc, curr) => {
        return acc + curr.commits.length;
    }, 0) / users.length;

    return commit_mean;
  }


  function getRandomHexColor(): string {
    const randomColor = Math.floor(Math.random() * 0xffffff);
    return `#${randomColor.toString(16).padStart(6, '0')}`;
}

  function getUserCommits(users: User[]){
    let userTotalCommits: any[] = [];
    users.forEach(user => { 
      userTotalCommits.push({
        username: user.username,
        colour: getRandomHexColor(),
        numCommits: user.commits.length
      })
    })
    // Sort by number of commits
    return userTotalCommits.sort((a, b) => a.numCommits - b.numCommits);
  }

  const commit_mean = getAverageCommits(users);

  function getSD(users: User[]): number {
    let commits: number[] = [];

    // Get the list of total commits for each user
    users.forEach(user => {
      commits.push(user.commits.length);
    })

    // Creating the mean with Array.reduce
    const n: number = users.length;

    const variance: number = commits.reduce((acc: number, val: number) => acc + Math.pow(val - commit_mean, 2), 0) / n;
    
    const sd = Math.sqrt(variance);

    return sd;
  }

  const sd = getSD(users);

  // Calculate SD & Mean
  function getRefPoints() {

    const refPoints: number[] = [(commit_mean - (2 * sd)), (commit_mean - sd), commit_mean, (commit_mean + sd), (commit_mean + (2 * sd))]

    return refPoints
}
  
  const refPointValues: number[] = getRefPoints();

  // Reference points for vertical lines
  const refPoints = [
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

  onMount(() => {
    const option = {
      backgroundColor: '#222',
      grid: {
        top: '30%',
        bottom: '30%',
        left: 40,
        right: 40,
        containLabel: false
      },
      xAxis: {
        type: 'value',
        min: Math.ceil(commit_mean - (3 * sd)),
        max: Math.ceil(commit_mean + (3 * sd)),
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
        max: users.length + 1,
      },
      series: [
        // Scatter points for people
        {
          type: 'scatter',
          data: people.map(p => [p.numCommits, 4]),
          symbolSize: 40,
          itemStyle: {
            color: function(params: { dataIndex: number }) {
              return people[params.dataIndex].colour;
            },
            borderColor: function(params: { dataIndex: number }) {
              return people[params.dataIndex].colour;
            },
            borderWidth: 4,
            shadowBlur: 0
          },
          label: {
            show: false
          },
          z: 3
        }
      ],
      graphic: [] // Will be set after chart is initialized
    };

    chart = echarts.init(chartContainer);
    chart.setOption(option);

    //function to add jitter to the y-axis values
    function jitter(data: [number, number][]): [number, number][] {
      // Data is already sorted from getUserCommits
      const stepSize = 0.5; // Adjust this value to control the vertical spread
      
      return data.map(([numCommits, y], index) => {
        const heightOffset = index * stepSize;
        return [numCommits, y + heightOffset];
      });
    }

    //function to remove jitter from the y-axis values
    function unjitter(data: [number, number][]): [number, number][] {
      return data.map(([numCommits, y]) => {
        return [numCommits, 4]; // Return to base height
      });
    }


    //add hover effect        
    chartContainer.addEventListener('mouseenter', () => {
      hovermessage = 'hover in';
      const jitteredData = jitter(people.map(p => [p.numCommits, 4]));
      chart.setOption({
        series: [{ data: jitteredData }]
      });
    });


    // remove hover effect
    chartContainer.addEventListener('mouseleave', () => {
            const jitteredData = unjitter(people.map(p => [p.numCommits, 4]));
      chart.setOption({
        series: [{ data: jitteredData }]
      });
      hovermessage = "hover out";
    });


    //onclick event to show the user name
    chart.on('click', function (params) {
      if (params.componentType === 'scatter') {
        const i = params.dataIndex;
        const person = people[i];
        onclickmessage = `Clicked on ${person.username}`;
      }
    });


    function updateGraphics() {
      const gridTop = chart.convertToPixel({gridIndex: 0}, [0, users.length + 1])[1];
      const xAxisY = chart.convertToPixel({gridIndex: 0}, [0, 0])[1];
      const graphics = refPoints.map(ref => {
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
              y: gridTop - 10
            }
          ]
        };
      });
      chart.setOption({ graphic: graphics });
    }

    chart.on('finished', updateGraphics);
    window.addEventListener('resize', () => {
      chart.resize();
      updateGraphics();
    });


    return () => {
      window.removeEventListener('resize', updateGraphics);
      chart.dispose();
    };
  });
</script>

<main class="container">
  <div bind:this={chartContainer} style="width: 100%; height: 200px;"></div>
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
</style>
{#if hovermessage}
  <div class="hover-message">
    {hovermessage}
  </div>
{/if}
{#if onclickmessage}
  <div class="click-message">
    {onclickmessage}
  </div>
{/if}