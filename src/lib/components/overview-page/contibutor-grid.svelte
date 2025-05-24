<script lang="ts">
    import ContributorTile from '$lib/components/global/contributor-tile.svelte';
  
    export let contributors = [
      { name: "Alice", avatar_url: "/git_profile.png", profile: "#", role: "RTE" },
      { name: "Bob", avatar_url: "/git_profile.png", profile: "#", role: "SA" },
      { name: "Charlie", avatar_url: "/git_profile.png", profile: "#", role: "PM" },
      { name: "Dana", avatar_url: "/git_profile.png", profile: "#", role: "Developer" },
      { name: "Eve", avatar_url: "/git_profile.png", profile: "#", role: "Developer" },
      { name: "Frank", avatar_url: "/git_profile.png", profile: "#", role: "" },
    ];
  </script>
  
  <div class="grid">
    {#each contributors as c (c.name)}
      <a href={c.profile} target="_blank" rel="noopener noreferrer" class="tile-link">
        <ContributorTile name={c.name} avatarUrl={c.avatar_url} role={c.role} />
      </a>
    {/each}
  </div>
  
  <style>
    .grid {
      display: flex;
      flex-wrap: wrap;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 2rem; /* Small gap between tiles */
      justify-content: center;
      padding: 2rem;
      box-sizing: border-box;
      max-width: 100%;
    }
  
    @media (min-width: 960px) {
      .grid {
        grid-template-columns: repeat(3, 280px); /* 3 tiles per row */
        justify-content: center;
        gap: 1rem;
      }
    }
  
    .tile {
      background: #222;
      padding: 1rem;
      border-radius: 8px;
      text-align: center;
      color: white;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
    }
  
    .tile img {
      border-radius: 50%;
      width: 100px;
      height: 100px;
      object-fit: cover;
      margin-bottom: 0.5rem;
    }
  
    .tile h3 {
      margin: 0;
      font-size: 1rem;
    }
  </style>
  