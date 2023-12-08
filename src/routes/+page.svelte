<script lang="ts">
  import Log from "../lib/homepage/Log.svelte";
  import { ListItem } from "fluent-svelte";
  import AskPassDialog from "../lib/homepage/ask_pass_dialog.svelte";
  import { onMount } from "svelte";
  import { try_password } from "$lib/commands/password";
  import { run_rpc_server } from "$lib/commands/app";
  import Setting from "$lib/homepage/setting.svelte";
  import About from "$lib/homepage/about.svelte";

  let items = ["Logs", "Settings", "About"];
  let selected = items[0];
  let selectedUi: any = Log;
  let showpassdialog = false;

  function selectItem(item: string) {
    selected = item;
    switch (selected) {
      case items[0]:
        selectedUi = Log;
        break;
      case items[1]:
        selectedUi = Setting;
        break;
      case items[2]:
        selectedUi = About;
        break;
      default:
        break;
    }
  }

  async function on_password_ok() {
    run_rpc_server();
  }

  onMount(async () => {
    if (!(await try_password())) {
      showpassdialog = true;
    } else {
      await on_password_ok();
    }
  });
</script>

<div class="flex h-full">
  <div
    class="px-4 flex-none w-48 h-full overflow-y-hidden hover:overflow-y-auto"
  >
    {#each items as item}
      <ListItem selected={item == selected} on:click={() => selectItem(item)}>
        {item}</ListItem
      >
    {/each}
  </div>

  <AskPassDialog onsubmit={on_password_ok} bind:open={showpassdialog} />
  <div class="pl-4 grow pt-4 h-full pr-8 pb-4">
    <div class="h-full overflow-hidden break-all">
      <div
        class="h-full overflow-y-auto scroll-smooth flex flex-col"
      >
        <svelte:component this={selectedUi} />
      </div>
    </div>
  </div>
</div>
