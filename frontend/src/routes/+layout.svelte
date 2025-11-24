<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";
	import { ChessKnight, Clock3, House, Music } from "@lucide/svelte";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="drawer lg:drawer-open">
	<input id="drawer1" type="checkbox" class="drawer-toggle" />
	<div class="drawer-content">
		<nav class="navbar w-full bg-base-300">
			<label
				for="drawer1"
				aria-label="open sidebar"
				class="btn btn-square btn-ghost"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					stroke-linejoin="round"
					stroke-linecap="round"
					stroke-width="2"
					fill="none"
					stroke="currentColor"
					class="my-1.5 inline-block size-4"
					><path
						d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"
					></path><path d="M9 4v16"></path><path
						d="M14 10l2 2l-2 2"
					></path></svg
				>
			</label>
			<div class="px-4">Gacha Portal</div>
		</nav>
		{@render children()}
	</div>
	<div class="drawer-side is-drawer-close:overflow-visible">
		<label
			for="drawer1"
			aria-label="close sidebar"
			class="drawer-overlay"
		></label>
		<div
			class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-16 is-drawer-open:w-64"
		>
			<ul class="menu w-full grow">
				<li>
					<button
						class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
						data-tip="Home"
					>
						<House />
						<span
							class="is-drawer-close:hidden"
							>Home</span
						>
					</button>
				</li>
				<li>
					<button
						class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
						data-tip="Timers"
					>
						<Clock3 />
						<span
							class="is-drawer-close:hidden"
							>Timers</span
						>
					</button>
				</li>
				<li>
					<button
						class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
						data-tip="Uma Musume"
					>
						<ChessKnight />
						<span
							class="is-drawer-close:hidden"
							>Uma Musume</span
						>
					</button>
				</li>
				<li>
					<button
						class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
						data-tip="学園アイドルマスター"
					>
						<Music />
						<span
							class="is-drawer-close:hidden"
							>学園アイドルマスター</span
						>
					</button>
				</li>
			</ul>
		</div>
	</div>
</div>
