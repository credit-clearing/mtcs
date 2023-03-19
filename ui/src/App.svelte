<script lang="ts">
  import { onMount, setContext } from "svelte";
  import type { ActionHash, AppAgentClient } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import "@material/mwc-circular-progress";

  import { clientContext } from "./contexts";

  import AllObligations from "./form_test/form/AllObligations.svelte";
  import CreateObligation from "./form_test/form/CreateObligation.svelte";

  import {
    ProfilesStore,
    ProfilesClient,
    CreateProfile,
    ProfilePrompt,
    profilesStoreContext,
    MyProfile,
    ProfilesContext
  } from '@holochain-open-dev/profiles';

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;

  $: client, loading, store;

  if (!customElements.get('profiles-context')){
    customElements.define('profiles-context', ProfilesContext)
  }

  if (!customElements.get('my-profile')){
    customElements.define('my-profile', MyProfile)
  }

  if (!customElements.get('profile-prompt')){
    customElements.define('profile-prompt', ProfilePrompt)
  }

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "mtcs");
    store = new ProfilesStore(new ProfilesClient(client, 'form_test'));
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });

  let count = 0;
  function addOne() {
    count += 1;
  }
</script>

<main>
  {#if loading}
    <div
      style="display: flex; flex: 1; align-items: center; justify-content: center"
    >
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <profiles-context store={store}>
      <profile-prompt>
        <my-profile />
        <div id="content" style="display: flex; flex-direction: column; flex: 1;">
          <button on:click={addOne}>Clicked {count} times</button>
          <AllObligations />

          <CreateObligation debtor={client.myPubKey} creator={client.myPubKey} />
        </div>
      </profile-prompt>
    </profiles-context>
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
