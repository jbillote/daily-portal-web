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
	<div>Week of:</div>
	<div>
		<button
			popovertarget="cally-popover1"
			class="input input-border w-fit border-none shadow-none"
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
		<div>
			<table
				class="m-auto border-separate border-spacing-2 border-none"
			>
				<thead>
					<tr>
						<th>Date</th>
						<th>Starting Fans</th>
						<th>Ending Fans</th>
					</tr>
				</thead>
				<tbody>
					{#each { length: 7 } as _, offset}
						<tr>
							<td
								>{date
									.plus({
										days: offset,
									})
									.toLocaleString()}</td
							>
							<td
								><input
									type="text"
									placeholder="Start"
									class="input"
								/></td
							>
							<td
								><input
									type="text"
									placeholder="End"
									class="input"
								/></td
							>
						</tr>
					{/each}
				</tbody>
			</table>
			<div>Total: 0</div>
		</div>
	</div>
</div>
