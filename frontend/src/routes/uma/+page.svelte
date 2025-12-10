<script lang="ts">
	import "cally";
	import { DateTime } from "luxon";
	import { ChevronLeft, ChevronRight } from "@lucide/svelte";

	// Extra steps to get the start of the week since we want the start of
	// the week to be Sunday and that can't be configured in Luxon
	function startOfWeek(iso: string): DateTime {
		return DateTime.fromISO(iso).endOf("week").minus({ week: 1 });
	}

	function updateTotal() {
		let newTotal = 0;

		for (let day = 0; day < startValues.length; day++) {
			newTotal += Math.max(
				0,
				endValues[day] - startValues[day],
			);
		}

		total = newTotal;
	}

	let date = $state(startOfWeek(DateTime.now().toISO()));
	let total = $state(0);
	let startValues: number[] = $state(Array(7).fill(0));
	let endValues: number[] = $state(Array(7).fill(0));
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
									id={`startDay${offset}`}
									on:change={updateTotal}
									bind:value={
										() =>
											startValues[
												offset
											].toString(),
										(
											v: string,
										) =>
											(startValues[
												offset
											] =
												parseInt(
													v,
												))
									}
								/></td
							>
							<td
								><input
									type="text"
									placeholder="End"
									class="input"
									id={`endDay${offset}`}
									on:change={updateTotal}
									bind:value={
										() =>
											endValues[
												offset
											].toString(),
										(
											v: string,
										) =>
											(endValues[
												offset
											] =
												parseInt(
													v,
												))
									}
								/></td
							>
						</tr>
					{/each}
				</tbody>
			</table>
			<div>Total: {total.toLocaleString()}</div>
			<button class="btn w-fit">Update</button>
		</div>
	</div>
</div>
