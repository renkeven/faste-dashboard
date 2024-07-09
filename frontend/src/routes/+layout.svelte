<script lang="ts">
	import '../style.css';
	import Navbar from './navbar.svelte';

	export const ssr = false;
</script>

<div class="px-4">
	<Navbar />
	<slot />
</div>
