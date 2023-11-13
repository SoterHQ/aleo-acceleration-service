<script lang="ts">
  import { get_proxy, set_proxy } from "$lib/commands/config";
  import { ContentDialog, Button, TextBox } from "fluent-svelte";
  import { onMount } from "svelte";

  export let open: boolean = false;
  export let onsubmit: () => Promise<void> | void = () => {};

  let proxy = "";
  let proxy_port = "";

  onMount(async () => {
    await read_proxy();
  });

  $: open || read_proxy();

  async function read_proxy() {
    try {
      let proxy_url = new URL(await get_proxy());
      proxy = proxy_url.host;
      proxy_port = proxy_url.port;
    } catch (e) {
      console.log(e);
    }
  }

  let errmsg = null;

  async function submit() {
    let proxyurl = `http://${proxy}:${proxy_port}`;
    set_proxy(proxyurl);
    await onsubmit();
    open = false;
  }
</script>

<ContentDialog bind:open>
  <h2 data-tauri-drag-region class="text-xl mb-4">set http proxy</h2>
  <p>edit proxy server</p>

  <div>
    <form on:submit={submit} class="flex">
      <div class="mr-4">
        <p class="my-2">ip address</p>
        <TextBox type="text" bind:value={proxy} on:input />
      </div>
      <div>
        <p class="my-2">proxy_port</p>
        <TextBox type="number" bind:value={proxy_port} />
      </div>
      <button type="submit" style="display: none;" />
    </form>
  </div>
  {#if errmsg}
    <p>{errmsg}</p>
  {/if}
  <svelte:fragment slot="footer">
    <Button variant="standard" on:click={submit}>Save</Button>
    <Button
      variant="standard"
      on:click={() => {
        open = false;
      }}>Cancel</Button
    >
  </svelte:fragment>
</ContentDialog>
