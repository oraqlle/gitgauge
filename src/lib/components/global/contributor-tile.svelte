<script lang="ts">
    export let name: string;
    export let avatarUrl: string;

    export let contributions: {
    stats: {
        numCommits: number;
        linesOfCode: number;
        meanLinesPerCommit: number;
        additions: number;
        deletions: number;
        meanCommitsPerDay: number;
    };
    scaling: number;
    analysis: string;
    } = 
    {
        stats: {
            numCommits: 72,
            linesOfCode: 94,
            meanLinesPerCommit: 0.75,
            additions: 82,
            deletions: 43,
            meanCommitsPerDay: 3.2,
        },
        scaling: 1.1,
        analysis: 'lorem ipsum',
    };


</script>
  
  <div class="tile">
    <img src={avatarUrl} alt={name} class="avatar" />
    <div class="profile-preview">
        <div class="heading-1">{name}</div>
        <div class="contributor-stats">
            <div class="scaling">
                Scaling: {contributions.scaling}
            </div>
            <p>
                <span style="white-space: nowrap;">{contributions.stats.numCommits} commits</span> &nbsp;&nbsp;&nbsp;
                <span style="white-space: nowrap;">{contributions.stats.linesOfCode} lines of code</span> &nbsp;&nbsp;&nbsp;
                <span style="white-space: nowrap;">{contributions.stats.meanLinesPerCommit} lines/commit</span> &nbsp;&nbsp;&nbsp;
                <span style="white-space: nowrap;">{contributions.stats.meanCommitsPerDay} commits/day</span>
            </p>  
            <div class="git-changes">
                <div class="additions">
                    {contributions.stats.additions}++ additions
                </div>
                <div class="deletions">
                    {contributions.stats.deletions}-- deletions
                </div>
            </div>
        </div>
    </div>
  </div>
  
  <style>
    .tile {
      background: var(--background-tile, #222);
      border-radius: 8px;
      padding: 2rem;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
      text-align: center;
      color: var(--white, #fff);
      width: 450px;
      height: 240px;
      box-sizing: border-box;
      display: flex;
      flex-direction: row;
      align-items: flex-start;
    }

    .profile-preview {
        padding-left: 1.5rem;
        padding-right: 1.5rem;
        text-align: left;
    }
  
    .avatar {
      width: 60px;
      height: 60px;
      border-radius: 50%;
      object-fit: cover;
      margin-bottom: 0.75rem;
    }
  
    .contributor-stats {
      font-size: 0.9rem;
      color: var(--label-tertiary, #aaa);
    }

    .git-changes {
        font-family: DM Sans, monospace;
        font-weight: strong;
        flex-direction: in;
        text-wrap: nowrap;
    }
    
    .additions {
        color: lime
    }

    .deletions {
        color: coral
    }
  </style>
  