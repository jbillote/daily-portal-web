<script lang="ts">
	import "cally";
	import { ChevronLeft, ChevronRight } from "@lucide/svelte";

	let date = $state("Pick a date");
</script>

<div class="text-center">
	<div>Uma Musume (EN)</div>

	<div>
		<button
			popovertarget="cally-popover1"
			class="input input-border"
			id="cally1"
			style="anchor-name:--cally1"
		>
			{date}
		</button>
		<div
			popover
			id="cally-popover1"
			class="dropdown bg-base-100 rounded-box shadow-lg"
			style="position-anchor:--cally1"
		>
			<calendar-date
				class="cally"
				on:change={(e: Event) =>
					(date = (e.target as HTMLInputElement)
						.value)}
			>
				<div slot="previous">
					<ChevronLeft aria-label="Previous" />
				</div>
				<div slot="next">
					<ChevronRight aria-label="Next" />
				</div>
				<calendar-month></calendar-month>
			</calendar-date>
		</div>
	</div>
</div>
