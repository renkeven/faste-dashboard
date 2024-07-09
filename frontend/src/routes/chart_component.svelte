<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { type InputData } from "$lib/interfaces.ts";
    import "chartjs-adapter-dayjs-4/dist/chartjs-adapter-dayjs-4.esm";

    let chartCanvas: HTMLCanvasElement;
    let Chart: any;
    let zoomPlugin: any;

    const chartStore = writable<any | null>(null);

    export let data: InputData;

    const cleanedData = [
        {
            label: "Data",
            data: data["data"].map((point) => ({ x: point.x, y: point.y })),
            borderColor: "rgb(75, 192, 192)",
            backgroundColor: "rgb(75, 192, 192)",
            pointStyle: "circle",
            pointRadius: 5,
            pointHoverRadius: 8,
            fill: false
        },
        {
            label: "Upper Bound",
            data: data["data"].map((point) => ({ x: point.x, y: point.yMax })),
            borderColor: "rgba(75, 192, 192, 0.2)",
            backgroundColor: "rgba(75, 192, 192, 0.2)",
            pointRadius: 0,
            pointHitRadius: 0,
            borderDash: [10, 10],
            fill: "+1"
        },
        {
            label: "Lower Bound",
            data: data["data"].map((point) => ({ x: point.x, y: point.yMin })),
            borderColor: "rgba(75, 192, 192, 0.2)",
            backgroundColor: "rgba(75, 192, 192, 0.2)",
            pointRadius: 0,
            pointHitRadius: 0,
            borderDash: [10, 10],
            fill: false
        }
    ];

    const zoomOptions = {
        limits: {
            x: { min: "original", max: "original", minRange: 1 }
        },
        pan: {
            enabled: true,
            modifierKey: "ctrl"
        },
        zoom: {
            drag: {
                enabled: true
            },
            mode: "x"
        }
    };

    const tooltipOptions = {
        position: "nearest",
        callbacks: {
            label: function (context: any) {
                const dataPoint = data["data"][context.dataIndex];
                if (context.datasetIndex === 0) {
                    console.log(data);
                    return [
                        `Y: ${dataPoint.y.toFixed(2)}`,
                        `Min: ${dataPoint.yMin.toFixed(2)}`,
                        `Max: ${dataPoint.yMax.toFixed(2)}`
                    ];
                }
                return null;
            }
        },
        filter: function (tooltipItem: any) {
            return tooltipItem.datasetIndex === 0;
        }
    };

    const resetZoom = () => {
        chartStore.update((chart) => {
            if (chart) {
                chart.resetZoom();
            }
            return chart;
        });
    };

    onMount(async () => {
        // Dynamically import chart.js and zoom plugin
        const chartModule = await import("chart.js/auto");
        Chart = chartModule.default;
        const zoomModule = await import("chartjs-plugin-zoom");
        zoomPlugin = zoomModule.default;
        Chart.register(zoomPlugin);

        const ctx = chartCanvas.getContext("2d");
        if (ctx) {
            let newChart = new Chart(ctx, {
                type: "line",
                data: {
                    datasets: cleanedData
                },
                options: {
                    responsive: true,
                    elements: {
                        line: {
                            tension: 0.3
                        }
                    },
                    plugins: {
                        zoom: zoomOptions,
                        title: {
                            display: true,
                            text: data["name"]
                        },
                        legend: {
                            display: false
                        },
                        tooltip: tooltipOptions
                    },
                    transitions: {
                        zoom: {
                            animation: {
                                duration: 100
                            }
                        }
                    },
                    scales: {
                        x: {
                            type: "timeseries",
                            time: {
                                unit: "year",
                                round: true,
                                displayFormats: {
                                    year: "YYYY"
                                },
                                tooltipFormat: "YYYY"
                            },
                            position: "bottom",
                            title: {
                                display: true,
                                text: "Year"
                            }
                        },
                        y: {
                            type: "linear",
                            position: "left",
                            title: {
                                display: true,
                                text: "Value"
                            }
                        }
                    }
                }
            });
            chartStore.set(newChart);
            // Add event listener
            chartCanvas.addEventListener("dblclick", resetZoom);
        }
    });

    // Clean up event listener on component destruction
    onMount(() => {
        return () => {
            if (chartCanvas) {
                chartCanvas.removeEventListener("dblclick", resetZoom);
            }
        };
    });
</script>

<div class="w-1/2 h-1/2">
    <canvas bind:this={chartCanvas}></canvas>
</div>
