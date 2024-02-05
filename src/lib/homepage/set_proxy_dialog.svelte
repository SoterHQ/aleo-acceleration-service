<script lang="ts">
  import { get_proxy, set_proxy, test_proxy } from "$lib/commands/config";
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
      proxy = proxy_url.hostname;
      proxy_port = proxy_url.port;
    } catch (e) {
      console.log(e);
    }
  }

  let message = null;

  async function submit() {
    let proxyurl = "";
    if (proxy && proxy_port) {
      proxyurl = `http://${proxy}:${proxy_port}`;
    }
    set_proxy(proxyurl);
    await onsubmit();
    open = false;
  }

  async function test_proxy_setting() {
    if (!proxy || !proxy_port) {
      message = "Proxy url or port is empty";
      return;
    }
    let proxyurl = `http://${proxy}:${proxy_port}`;
    try {
      await test_proxy(proxyurl);
      message = "connect success";
    } catch (e) {
      message = e;
    }
  }
</script>

<ContentDialog bind:open>
  <h2 data-tauri-drag-region class="text-xl mb-4">set http proxy</h2>
  <div class="flex justify-between">
    <p>Edit proxy server</p>
    <Button variant="standard" on:click={test_proxy_setting}>Test</Button>
  </div>

  <div>
    <form on:submit={submit} class="flex">
      <div class="mr-4">
        <p class="my-2">IP address</p>
        <TextBox type="text" bind:value={proxy} on:input />
      </div>
      <div>
        <p class="my-2">Proxy port</p>
        <TextBox type="number" bind:value={proxy_port} />
      </div>
      <button type="submit" style="display: none;" />
    </form>
  </div>
  {#if message}
    <p class="mt-2">{message}</p>
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
