<script lang="ts">
    export const ssr = false;

    import { type InputData } from "$lib/interfaces";
    import ChartComponent from "./chart_component.svelte";

    const data: InputData = {
        name: "Example",
        data: [
            { x: "2023", y: 10, yMin: 8, yMax: 12 },
            { x: "2024", y: 15, yMin: 13, yMax: 17 },
            { x: "2025", y: 8, yMin: 6, yMax: 10 },
            { x: "2026", y: 12, yMin: 10, yMax: 14 },
            { x: "2027", y: 20, yMin: 18, yMax: 22 },
            { x: "2028", y: 17, yMin: 15, yMax: 19 },
            { x: "2029", y: 13, yMin: 11, yMax: 15 },
            { x: "2030", y: 18, yMin: 16, yMax: 20 },
            { x: "2031", y: 11, yMin: 9, yMax: 13 },
            { x: "2032", y: 16, yMin: 14, yMax: 18 }
        ]
    };

    // let petData: any = null;
    // let error: string | null = null;

    // async function fetchPetData() {
    // 	try {
    // 		const response = await fetch('http://0.0.0.0:8080/enter');

    // 		if (!response.ok) {
    // 			throw new Error(`HTTP error! status: ${response.status}`);
    // 		}

    // 		petData = await response.json();
    // 	} catch (e) {
    // 		console.error('Error fetching pet data:', e);
    // 		error = 'Failed to fetch pet data';
    // 	}
    // }

    // onMount(() => {
    // 	fetchPetData();
    // });
</script>

<div class="flex flex-col w-full p-2">
    <div class="flex w-full justify-center items-center">
        <ChartComponent {data} />
    </div>
</div>

<!-- {#if error}
	<p>Error: {error}</p>
{:else if petData}
	<h1>Pet Data</h1>
	<pre>{JSON.stringify(petData, null, 2)}</pre>
{:else}
	<p>Loading...</p>
{/if} -->
