<script lang="ts">
	import { onMount } from 'svelte';

	let prsResponse: { data: { id: number; title: string; state: string }[] };

	onMount(async function () {
		const response = await fetch('/api/prs');
		prsResponse = await response.json();
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<table class="table">
		<thead>
			<tr>
				<th>Id</th>
				<th>Title</th>
				<th>State</th>
			</tr>
		</thead>
		<tbody>
			{#if prsResponse !== undefined}
				{#each prsResponse.data as pr}
					<tr>
						<td>{pr.id}</td>
						<td>{pr.title}</td>
						<td>{pr.state}</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}
</style>
