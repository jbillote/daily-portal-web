<script lang="ts">
  import { DateTime } from 'luxon';
  import { onMount } from 'svelte';

  let time = $state(DateTime.now());

  // Might not have to use $state to save resources
  // Use $state for now so it gets updated right away, though this shouldn't happen often
  // TODO: Look into potential better ways to get to next reset time
  let hoyoSlopDailyReset = $state(
    DateTime.utc().set({ hour: 9, minute: 0, second: 0, millisecond: 0 })
  );
  let gkmsDailyReset = $state(
    DateTime.utc().set({ hour: 20, minute: 0, second: 0, millisecond: 0 })
  );
  let arknightsDailyReset = $state(
    DateTime.utc().set({ hour: 11, minute: 0, second: 0, millisecond: 0 })
  );
  let fgoJPDailyReset = $state(
    DateTime.utc().set({ hour: 19, minute: 0, second: 0, millisecond: 0 })
  );
  let fgoENDailyReset = $state(
    DateTime.utc().set({ hour: 4, minute: 0, second: 0, millisecond: 0 })
  );

  onMount(() => {
    const interval = setInterval(() => {
      time = DateTime.now();
    }, 1000);

    hoyoSlopDailyReset =
      hoyoSlopDailyReset.diff(time).valueOf() < 0
        ? hoyoSlopDailyReset.plus({ days: 1 })
        : hoyoSlopDailyReset;
    gkmsDailyReset =
      gkmsDailyReset.diff(time).valueOf() < 0 ? gkmsDailyReset.plus({ days: 1 }) : gkmsDailyReset;
    arknightsDailyReset =
      arknightsDailyReset.diff(time).valueOf() < 0
        ? arknightsDailyReset.plus({ days: 1 })
        : arknightsDailyReset;
    fgoJPDailyReset =
      fgoJPDailyReset.diff(time).valueOf() < 0
        ? fgoJPDailyReset.plus({ days: 1 })
        : fgoJPDailyReset;
    fgoENDailyReset =
      fgoENDailyReset.diff(time).valueOf() < 0
        ? fgoENDailyReset.plus({ days: 1 })
        : fgoENDailyReset;

    return () => {
      clearInterval(interval);
    };
  });
</script>

<div class="text-center">
  <div class="text-2xl font-bold">Honkai Star Rail Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{hoyoSlopDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {hoyoSlopDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">Wuthering Waves Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{hoyoSlopDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {hoyoSlopDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">GFL2 Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{hoyoSlopDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {hoyoSlopDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">Arknights Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{arknightsDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {arknightsDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">学園アイドルマスター Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{gkmsDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {gkmsDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">FGO JP Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{fgoJPDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {fgoJPDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
  <div class="text-2xl font-bold">FGO EN Daily Reset</div>
  <div class="tooltip">
    <div class="tooltip-content">
      <span>{fgoENDailyReset.toLocal().toFormat('EEEE MMMM d, yyyy HH:mm:ss')}</span>
    </div>
    <div>Daily reset in {fgoENDailyReset.diff(time).toFormat('hh:mm:ss')}</div>
  </div>
</div>
