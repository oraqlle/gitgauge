<script lang="ts">
    export let contributors = [
      { name: "Alice", avatar_url: "https://i.pravatar.cc/100?img=1", profile: "#" },
      { name: "Bob", avatar_url: "https://i.pravatar.cc/100?img=2", profile: "#" },
      { name: "Charlie", avatar_url: "https://i.pravatar.cc/100?img=3", profile: "#" },
      { name: "Dana", avatar_url: "https://i.pravatar.cc/100?img=4", profile: "#" },
      { name: "Eve", avatar_url: "https://i.pravatar.cc/100?img=5", profile: "#" },
      { name: "Frank", avatar_url: "https://i.pravatar.cc/100?img=6", profile: "#" },
    ];
  </script>
  
  <div class="grid">
    {#each contributors as c (c.name)}
      <div class="tile">
        <a href={c.profile} target="_blank" rel="noopener noreferrer">
          <img src={c.avatar_url} alt={`Avatar of ${c.name}`} loading="lazy" />
          <h3>{c.name}</h3>
        </a>
      </div>
    {/each}
  </div>
  
  <style>
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
      gap: 1rem;
      justify-content: center;
      max-width: 1000px;
      margin: 2rem auto;
    }
    .tile {
      background: #222;
      padding: 1rem;
      border-radius: 8px;
      text-align: center;
      color: white;
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
  