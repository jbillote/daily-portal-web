<script lang="ts">
	import "cally";
	import { DateTime } from "luxon";
	import { ChevronLeft, ChevronRight } from "@lucide/svelte";

	// Extra steps to get the start of the week since we want the start of
	// the week to be Sunday and that can't be configured in Luxon
	function startOfWeek(iso: string): DateTime {
		return DateTime.fromISO(iso).endOf("week").minus({ week: 1 });
	}

	let date = $state(startOfWeek(DateTime.now().toISO()));
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
			{date.toLocaleString()}
		</button>
		<div
			popover
			id="cally-popover1"
			class="dropdown bg-base-100 rounded-box shadow-lg"
			style="position-anchor:--cally1"
		>
			<calendar-date
				class="cally"
				firstDayOfWeek={0}
				on:change={(e: Event) =>
					(date = startOfWeek(
						(e.target as HTMLInputElement)
							.value,
					))}
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
