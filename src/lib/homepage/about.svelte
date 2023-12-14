<script lang="ts">
  import { get_build_info, type BuildInfo } from "$lib/commands/app";
  import { shell } from "@tauri-apps/api";
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";

  let TauriVersion;
  let version;
  let buildInfo: BuildInfo | undefined;
  onMount(async () => {
    TauriVersion = await getTauriVersion();
    version = await getVersion();
    buildInfo = await get_build_info();
  });
</script>

<div class="column">
  <h1 class="text-2xl mb-4">Aleo wallet Acceleration Service</h1>
  <p>version: {version}</p>
  {#if buildInfo}
    <p>build time: {buildInfo.time}</p>
    <p>commit: {buildInfo.commit}</p>
  {/if}
  <p>tauri version: {TauriVersion}</p>
  <p>
    <!-- svelte-ignore a11y-missing-attribute -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    sourcecode:
    <a
      on:click={() =>
        shell.open("https://github.com/SoterHQ/aleo-acceleration-service")}
      >https://github.com/SoterHQ/aleo-acceleration-service</a
    >
  </p>
</div>
