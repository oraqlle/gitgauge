<script lang="ts">
  import {
    getUserTotalCommits,
    getUserTotalLinesOfCode,
    getUserLinesPerCommit,
    getUserTotalAdditions,
    getUserTotalDeletions,
    calculateScalingFactor,
    getAverageCommits,
    getSD,
    type Contributor
  } from '../../metrics';

  let { users, selectedBranch } = $props();

  // Calculate metrics for each user
  let commit_mean = getAverageCommits(users);
  let sd = getSD(users);
  
  let peopleWithMetrics = users.map((user: Contributor) => {
    const numCommits = getUserTotalCommits(user);
    const scalingFactor = calculateScalingFactor(numCommits, commit_mean, sd);
    
    return {
      username: user.author.login,
      image: null,
      numCommits,
      totalLinesOfCode: getUserTotalLinesOfCode(user),
      linesPerCommit: getUserLinesPerCommit(user),
      totalAdditions: getUserTotalAdditions(user),
      totalDeletions: getUserTotalDeletions(user),
      scalingFactor: scalingFactor.toFixed(1)
    };
  });
</script>

<div class="cards-row">
  {#each peopleWithMetrics as person}
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

<style>
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
</style> 