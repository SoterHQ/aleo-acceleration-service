<script lang="ts">
  import { clear_logs, get_logs } from "$lib/commands/log";
  import { Button } from "fluent-svelte";
  import { onDestroy, onMount } from "svelte";

  let logs = [];
  let timerId: number;

  onMount(async () => {
    timerId = setInterval(async () => {
      logs = await get_logs();
    }, 100);
  });

  onDestroy(() => {
    clearInterval(timerId);
  });
</script>

<div class="text-left text-sm leading-loose overflow-auto">
  <div class="flex justify-end">
    <Button variant="hyperlink" on:click={clear_logs}>Clear</Button>
  </div>
  <div class="whitespace-pre-wrap">
    {#each logs as log_msg}
      <p>{log_msg}</p>
    {/each}
  </div>
</div>
