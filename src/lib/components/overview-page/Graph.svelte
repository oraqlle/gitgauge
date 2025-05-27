<script lang="ts">
    import { Chart } from "svelte-echarts";
    import { init, use, util as eUtils } from "echarts/core";
    import { ScatterChart } from "echarts/charts";
    import {
        get_average_commits,
        get_sd,
        get_ref_points,
        type Contributor,
    } from "../../metrics";
    import { GridComponent, TitleComponent } from "echarts/components";
    import { CanvasRenderer } from "echarts/renderers";
    import { info } from "@tauri-apps/plugin-log";

    let { contributors }: { contributors: Contributor[] } = $props();

    type User = Readonly<{
        contributor: Contributor;
        offsetIndex: number;
    }>;

    function get_user_commits(users: Contributor[]) {
        if (users.length === 0) return [];

        let user_total_commits: any[] = [];
        users.forEach((user) => {
            user_total_commits.push({
                username: user.author.login,
                image: user.author.avatar_url,
            });
        });

        // Sort by number of commits
        const sorted_commits = user_total_commits.sort(
            (a, b) => a.total_commits - b.total_commits,
        );

        // Group by numCommits and apply horizontal offset
        const groups = new Map<number, any[]>();
        sorted_commits.forEach((user) => {
            if (!groups.has(user.total_commits)) {
                groups.set(user.total_commits, []);
            }
            groups.get(user.total_commits)!.push(user);
        });

        // Apply horizontal offset to overlapping points
        const result: User[] = [];
        groups.forEach((users, _) => {
            if (users.length === 1) {
                result.push(users[0]);
            } else {
                users.forEach((user, index) => {
                    result.push({
                        ...user,
                        offsetIndex: index - (users.length - 1) / 2,
                    });
                });
            }
        });

        return result;
    }

    // Reactive declarations for derived values
    let commit_mean = $state(get_average_commits(contributors));
    let sd = $state(get_sd(contributors));
    let ref_point_values = $derived(get_ref_points(commit_mean, sd));
    //let contributor_data = $state(get_user_commits(contributors));

    // Reference points for vertical lines
    let ref_points = $derived(
        sd === 0
            ? [{ label: "mean", value: ref_point_values[2] }]
            : [
                  { label: "-2σ", value: ref_point_values[0] },
                  { label: "-σ", value: ref_point_values[1] },
                  { label: "mean", value: ref_point_values[2] },
                  { label: "+σ", value: ref_point_values[3] },
                  { label: "+2σ", value: ref_point_values[4] },
              ],
    );

    type min_max = Readonly<{
        min: number;
        max: number;
    }>;

    // Calculate min and max number of commits for filteredPeople
    let { min: min_commits_x, max: max_commits_x } = contributors.reduce(
        (a: min_max, p: Contributor) => {
            return {
                min: Math.min(a.min, p.total_commits),
                max: Math.max(a.max, p.total_commits),
            };
        },
        { min: Infinity, max: 1 },
    );

    use([ScatterChart, GridComponent, CanvasRenderer, TitleComponent]);

    let commit_data = $derived(
        contributors.map((p: Contributor) => [p.total_commits, 1])
    );

    let contributor_data = $derived(
        contributors.map((p: Contributor) => [p.total_commits, 1, p.author.login])
    );

    // const ref_line_graphics = $derived( ref_points.map((ref) => { const x = chart.convertToPixel({ gridIndex: 0 }, [ref.value, 0])[0];
    //            return {
    //                type: "group",
    //                children: [
    //                    {
    //                        type: "line",
    //                        shape: {
    //                            x1: x,
    //                            y1: grid_top,
    //                            x2: x,
    //                            y2: x_axis_Y,
    //                        },
    //                        style: {
    //                            stroke: "#fff",
    //                            lineDash: [4, 4],
    //                            lineWidth: 1,
    //                            opacity: 0.5,
    //                        },
    //                        silent: true,
    //                    },
    //                    {
    //                        type: "text",
    //                        style: {
    //                            text: ref.label,
    //                            fontSize: 12,
    //                            fill: "#fff",
    //                            font: "bold 16px sans-serif",
    //                            textAlign: "center",
    //                            textVerticalAlign: "bottom",
    //                        },
    //                        x: x,
    //                        y: grid_top - 8,
    //                    },
    //                ],
    //            };
    //        }),
    //    );
    //
    //    // Use y=2 as the top of the y-axis for gridTop
    //    const grid_top = chart.convertToPixel({ gridIndex: 0 }, [0, 2])[1];
    //    const x_axis_Y = chart.convertToPixel({ gridIndex: 0 }, [0, 0])[1];
    //
    //    // Create graphics for reference lines
    //
    //    // Create graphics for user images with fixed pixel offset
    //    const user_graphics = contributor_data.map((person: User) => {
    //        // Convert the base position to pixels
    //        const [baseX, y] = chart.convertToPixel({ gridIndex: 0 }, [
    //            person.contributor.total_commits,
    //            1,
    //        ]);
    //        // Apply fixed 16px offset if there's an offsetIndex
    //        const x = baseX + (person.offsetIndex ? person.offsetIndex * 16 : 0);
    //
    //    });

    let options = $derived({
        backgroundColor: "#222",
        grid: {
            top: "10%",
            bottom: "25%",
            left: 40,
            right: 40,
            containLabel: false,
        },
        xAxis: {
            type: "value",
            min: min_commits_x,
            max: max_commits_x,
            name: "Total Commits",
            nameGap: 40,
            nameLocation: "middle",
            position: "bottom",
            axisLine: {
                lineStyle: {
                    color: "#fff",
                    width: 2,
                },
            },
            axisLabel: {
                color: "#fff",
                fontSize: 16,
                margin: 16,
            },
            splitLine: { show: false },
            axisTick: {
                lineStyle: {
                    color: "#fff",
                    width: 2,
                },
            },
        },
        yAxis: {
            show: false,
            min: 0,
            max: 2,
        },
        series: [
            {
                type: "scatter",
                data: commit_data,
                symbolSize: 3,
                z: 3,
            },
            {
                name: "hover_points",
                type: "scatter",
                data: contributor_data,
                symbolSize: 32,
                z: 0,
                itemStyle: {
                    color: "transparent",
                },
                emphasis: {
                    focus: "series",
                    itemStyle: {
                        color: "transparent",
                        borderColor: "#fff",
                        borderWidth: 2,
                        shadowBlur: 10,
                        shadowColor: "rgba(255, 255, 255, 0.7)",
                    },
                },
            },
        ],
        tooltip: {
            trigger: "item",
            formatter: function (params: any) {
                if (params.seriesName === "hover_points") {
                    const username = params.data[2];
                    const person = contributors.find(
                        (p) => p.author.login === username,
                    );
                    if (!person) return username;
                    return `
              <div style="text-align: left;">
                <strong>${username}</strong><br/>
                Total Commits: ${params.data[0]}
              </div>
            `;
                }

                return "";
            },
        },
        graphics: eUtils.map(contributor_data, (p: User) => {
            return {
                type: "group",
                children: [
                    {
                        type: "image",
                        style: {
                            image: p.contributor.author.avatar_url,
                            width: 40,
                            height: 40,
                        },
                        x: x - 20, // Center the image
                        y: y - 20, // Center the image
                        silent: false,
                        clipPath: {
                            type: "circle",
                            shape: {
                                cx: 20,
                                cy: 20,
                                r: 20,
                            },
                        },
                    },
                ],
            };
        }),
    });
</script>

<div class="app">
    <Chart {init} {options} />
</div>

<style>
    .app {
        width: 90vw;
        height: 90vh;
    }
</style>
