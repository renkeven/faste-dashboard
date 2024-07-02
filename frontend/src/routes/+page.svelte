<script lang="ts">
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';

	let data = [20, 100, 50, 12, 20, 130, 45];
    let labels = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
    let ctx;
    let canvas: any;

    onMount(() => {
        ctx = canvas.getContext('2d');
        var chart = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'Unit Sales',
                        data: data
                    }
                ]
            }
        });
    });
</script>

<div class="w-1/2 h-1/2">
    <canvas bind:this={canvas} />
</div>
