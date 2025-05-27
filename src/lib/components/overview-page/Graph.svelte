<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import * as echarts from 'echarts';
    import {
        get_average_commits,
        get_sd,
        get_ref_points,
        type Contributor,
    } from "../../metrics";

    let { contributors }: { contributors: Contributor[] } = $props();
    let chartContainer: HTMLElement;
    let chart: echarts.ECharts;
    let filteredPeople: any[] = [];
    let minCommits: number = 0;
    let maxCommits: number = 1;
    let xMin: number = 0;
    let xMax: number = 1;
    let commit_mean: number = 0;
    let sd: number = 0;
    let ref_point_values: number[] = [];
    let refPoints: { label: string, value: number }[] = [];
    let resizeHandler: () => void;

    $effect(() => {
        filteredPeople = getUserCommits(contributors);
    });
    $effect(() => {
        minCommits = filteredPeople.length > 0 ? Math.min(...filteredPeople.map((p: any) => p.numCommits)) : 0;
    });
    $effect(() => {
        maxCommits = filteredPeople.length > 0 ? Math.max(...filteredPeople.map((p: any) => p.numCommits)) : 1;
    });
    $effect(() => {
        xMin = minCommits === maxCommits ? minCommits - 1 : minCommits - 1;
    });
    $effect(() => {
        xMax = minCommits === maxCommits ? maxCommits + 1 : maxCommits + 1;
    });
    $effect(() => {
        commit_mean = get_average_commits(contributors);
    });
    $effect(() => {
        sd = get_sd(contributors);
    });
    $effect(() => {
        ref_point_values = get_ref_points(commit_mean, sd);
    });
    $effect(() => {
        refPoints = sd === 0
            ? [{ label: 'mean', value: ref_point_values[2] }]
            : [
                { label: '-2σ', value: ref_point_values[0] },
                { label: '-σ', value: ref_point_values[1] },
                { label: 'mean', value: ref_point_values[2] },
                { label: '+σ', value: ref_point_values[3] },
                { label: '+2σ', value: ref_point_values[4] }
            ];
    });
    $effect(() => {
        if (chart) setChartOptions();
    });

    function getUserCommits(users: Contributor[]) {
        if (users.length === 0) return [];
        let userTotalCommits: any[] = [];
        users.forEach(user => {
            userTotalCommits.push({
                username: user.author.login,
                image: user.author.avatar_url,
                numCommits: user.total_commits
            });
        });
        const sortedCommits = userTotalCommits.sort((a, b) => a.numCommits - b.numCommits);
        const groups = new Map<number, any[]>();
        sortedCommits.forEach(user => {
            if (!groups.has(user.numCommits)) {
                groups.set(user.numCommits, []);
            }
            groups.get(user.numCommits)!.push(user);
        });
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

    function updateGraphics() {
        if (!chart) return;
        const gridTop = chart.convertToPixel({gridIndex: 0}, [0, 6])[1];
        const xAxisY = chart.convertToPixel({gridIndex: 0}, [0, 0])[1];

        const fullHeight = xAxisY - gridTop;
        const tintHeight = fullHeight * 0.9;

        const marginLeft = 40; // px
        const marginRight = 40; // px
        const containerWidth = chartContainer.clientWidth;
        const drawableWidth = containerWidth - marginLeft - marginRight;

        function xScale(value: number) {
            return marginLeft + ((value - xMin) / (xMax - xMin)) * drawableWidth;
    }

        // Clamp function to ensure tints stay inside drawable area
        function clampTint(x: number, width: number) {
            const clampedX = Math.max(x, marginLeft);
            const maxWidth = Math.min(width - (clampedX - x), containerWidth - marginRight - clampedX);
            return { x: clampedX, width: maxWidth };
        }

        // Calculate pixel positions of ref points (commit counts)
        const xMinus2Sigma = xScale(ref_point_values[0]);
        const xMinusSigma = xScale(ref_point_values[1]);
        const xPlusSigma = xScale(ref_point_values[3]);
        const xPlus2Sigma = xScale(ref_point_values[4]);

        // Clamp tints within bounds
        const leftTint = clampTint(xMinus2Sigma, xMinusSigma - xMinus2Sigma);
        const middleTint = clampTint(xMinusSigma, xPlusSigma - xMinusSigma);
        const rightTint = clampTint(xPlusSigma, xPlus2Sigma - xPlusSigma);

        // White tint between -σ and +σ
        const tintBetween1Sigma = {
            type: 'rect',
            shape: {
                x: middleTint.x,
                y: xAxisY-tintHeight,
                width: middleTint.width,
                height: tintHeight
            },
            style: {
                fill: 'rgba(255, 255, 255, 0.20)'
            },
            silent: true,
            z: 1
        };

        const tintBetween2SigmaLeft = {
            type: 'rect',
            shape: {
                x: leftTint.x,
                y: xAxisY-tintHeight,
                width: leftTint.width,
                height: tintHeight
            },
            style: {
                fill: 'rgba(255, 255, 255, 0.1)'
            },
            silent: true,
            z: 1
        };

        const tintBetween2SigmaRight = {
            type: 'rect',
            shape: {
                x: rightTint.x,
                y: xAxisY-tintHeight,
                width: rightTint.width,
                height: tintHeight
            },
            style: {
                fill: 'rgba(255, 255, 255, 0.1)'
            },
            silent: true,
            z: 1
        };

        const refLineGraphics = refPoints.map((ref) => {
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
                            fontSize: 14,
                            fill: '#fff',
                            font: 'bold 16px "DM Sans", sans-serif',
                            textAlign: 'center',
                            textVerticalAlign: 'bottom'
                        },
                        x: x,
                        y: gridTop - 8,
                        z:2
                    }
                ]
            };
        });
        const userGraphics = filteredPeople.map((person: any) => {
            const [baseX, y] = chart.convertToPixel({gridIndex: 0}, [person.numCommits, 1]);
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
                        x: x - 20,
                        y: y - 20,
                        z: 3,
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
        chart.setOption({ graphic: [
            tintBetween2SigmaLeft,
            tintBetween1Sigma,
            tintBetween2SigmaRight, 
            ...refLineGraphics, 
            ...userGraphics
        ] });
    }

    function setChartOptions() {
        const option = {
            backgroundColor: 'transparent',  //#222',
            grid: {
                top: '50%',
                bottom: 100,
                left: 40,
                right: 40,
                containLabel: false
            },
            xAxis: {
                type: 'value',
                min: xMin,
                max: xMax,
                name: 'Total Commits',
                nameTextStyle: {
                    fontSize: 20,
                    fontWeight: 'bold',
                    fontFamily: 'DM Sans, sans-serif',
                },
                nameLocation: 'middle',
                nameGap: 60,
                axisLine: {
                    lineStyle: {
                        color: '#fff',
                        width: 2
                    }
                },
                axisLabel: {
                    color: '#fff',
                    fontSize: 15,
                    margin: 30
                },
                splitLine: { show: false },
                axisTick: {
                    length: 20,
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
                max: 2.5,
            },
            series: [
                {
                    type: 'scatter',
                    data: filteredPeople.map((p: any) => [p.numCommits, 1]),
                    symbolSize: 0,
                    z: 3
                },
                {
                    name: 'hoverPoints',
                    type: 'scatter',
                    data: filteredPeople.map((p: any) => [p.numCommits, 1, p.username]),
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
                        const person = filteredPeople.find((p: any) => p.username === username);
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
        chart = echarts.init(chartContainer);
        setChartOptions();
        resizeHandler = () => {
            chart.resize();
            updateGraphics();
        };
        window.addEventListener('resize', resizeHandler);
    });
    onDestroy(() => {
        window.removeEventListener('resize', resizeHandler);
        chart.dispose();
    });
</script>

<div bind:this={chartContainer} class="chart-container"></div>

<style>
    .chart-container {
        width: 100%;
        height: 500px;
        font-family: 'DM Sans', sans-serif;
        padding-bottom: 2rem;
    }

</style>

