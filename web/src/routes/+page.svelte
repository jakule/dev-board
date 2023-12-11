<script lang="ts">
	import { onMount } from 'svelte';

	let prsResponse: { data: { id: number; title: string;
		score: number, opened_at: Date, should_close_at: Date }[] };

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
				<th>Score</th>
				<th>Opened At</th>
				<th>Should Close at</th>
			</tr>
		</thead>
		<tbody>
			{#if prsResponse !== undefined}
				{#each prsResponse.data as pr}
					<tr>
						<td><a href='https://github.com/gravitational/teleport/pull/{pr.id}'>{pr.id}</a></td>
						<td>{pr.title}</td>
						<td>{pr.score}</td>
						<td>{pr.opened_at}</td>
						<td>{pr.should_close_at}</td>
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
