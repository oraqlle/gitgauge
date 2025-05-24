<script lang="ts">
    export let name: string;
    export let avatarUrl: string;
    export let role: string = '';
  </script>
  
  <div class="tile">
    <img src={avatarUrl} alt={name} class="avatar" />
    <h3 class="name">{name}</h3>
    {#if role}
      <p class="role">{role}</p>
    {/if}
  </div>
  
  <style>
    .tile {
      background: var(--background-tile, #222);
      border-radius: 8px;
      padding: 1rem;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
      text-align: center;
      color: var(--white, #fff);
      /* width: 100%; */
      width: 450px;
      height: 240px;
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  
    .avatar {
      width: 100px;
      height: 100px;
      border-radius: 50%;
      object-fit: cover;
      margin-bottom: 0.75rem;
    }
  
    .name {
      margin: 0;
      font-size: 1.2rem;
      font-weight: 600;
    }
  
    .role {
      margin: 0.25rem 0 0;
      font-size: 0.9rem;
      color: var(--label-tertiary, #aaa);
    }
  </style>
  