<script lang="ts">
  import { onMount } from 'svelte';
  import * as echarts from 'echarts';
  import type { User } from '../../../data/dummyData';
  import { getAverageCommits, getSD, getRefPoints, calculateScalingFactor } from '../../metrics';

  export let users: User[] = [];
  export let selectedBranch: string = 'all';

  let chartContainer: HTMLElement;
  let chart: echarts.ECharts;

  // Filter users based on selected branch
  $: filteredUsers = selectedBranch === 'all' 
    ? users 
    : users.map(user => ({
        ...user,
        commits: user.commits.filter(commit => commit.branch === selectedBranch)
      })).filter(user => user.commits.length > 0);

  function getUserCommits(users: User[]) {
    if (users.length === 0) return [];
    let userTotalCommits: any[] = [];
    users.forEach(user => { 
      userTotalCommits.push({
        username: user.username,
        image: user.image,
        numCommits: user.commits.length
      })
    });
    
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
        },
        {
          name: 'hoverPoints',
          type: 'scatter',
          data: filteredPeople.map(p => [p.numCommits, 1, p.username]),
          symbolSize: 32,
          z: 10,
          itemStyle: {
            color: 'transparent',
          },
          emphasis: {
            focus: 'series',
            itemStyle: {
              color: 'transparent',
              borderColor: '#fff',
              borderWidth: 2,
              shadowBlur: 10,
              shadowColor: 'rgba(255, 255, 255, 0.7)'
            }
          }
        }
      ],
      tooltip: {
        trigger: 'item',
        formatter: function (params: any) {
          if (params.seriesName === 'hoverPoints') {
            const username = params.data[2];
            const person = filteredPeople.find(p => p.username === username);
            if (!person) return username;
            return `
              <div style="text-align: left;">
                <strong>${username}</strong><br/>
                Total Commits: ${params.data[0]}
              </div>
            `;
          }
          return '';
        }
      },
      graphic: []
    };

    chart.setOption(option, true);
    updateGraphics();
  }

  onMount(() => {
    let cleanup: (() => void) | undefined;

    chart = echarts.init(chartContainer);
    
    window.addEventListener('resize', () => {
      chart.resize();
      updateGraphics();
    });

    cleanup = () => {
      window.removeEventListener('resize', updateGraphics);
      chart.dispose();
    };

    return () => {
      if (cleanup) cleanup();
    };
  });
</script>

<div bind:this={chartContainer} class="chart-container" style="width: 100%; height: 200px;"></div>

<style>
  .chart-container {
    margin-top: 10rem;
  }
</style> 