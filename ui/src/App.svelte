<script lang="ts">
  import { onMount, setContext } from "svelte";
  import type {
     ActionHash,
     AppAgentClient,
  } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import "@material/mwc-circular-progress";
  import { decode } from "@msgpack/msgpack";

  import { clientContext } from "./contexts";

  import AllObligations from "./form_test/form/AllObligations.svelte";
  import CreateObligation from "./form_test/form/CreateObligation.svelte";
  import MyPubKey from "./components/MyPubKey.svelte";

  import type { FormSignal, Obligation } from "./form_test/form/types";
  import ObligationDetail from "./form_test/form/ObligationDetail.svelte";

  import {
    ProfilesStore,
    ProfilesClient,
    ProfilePrompt,
    MyProfile,
    ProfilesContext,
    ListProfiles,
    SearchAgent,
  } from "@holochain-open-dev/profiles";

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let proposedObligations: Array<Obligation> | undefined;

$: client, loading, proposedObligations;

async function fetchObligation(obligationHash) {
  // const obligation_hash = decode((record.entry as any).Present.entry) as ActionHash;

  try {
    const record = await client.callZome({
      cap_secret: null,
      role_name: "form_test",
      zome_name: "form",
      fn_name: "get_obligation",
      payload: obligationHash,
    });
    // if (record) {
    //   return record as HRecord
    // }

    if (record) {
      return decode((record.entry as any).Present.entry) as Obligation;
    }
  } catch (error) {
    console.error(error);
  }
}

const desirializeData = async (serializedData) => {
  return decode(serializedData);
};

  $: client, loading, store;

  if (!customElements.get("profiles-context")) {
    customElements.define("profiles-context", ProfilesContext);
  }

  if (!customElements.get("my-profile")) {
    customElements.define("my-profile", MyProfile);
  }

  if (!customElements.get("list-profiles")) {
    customElements.define("list-profiles", ListProfiles);
  }

  if (!customElements.get("profile-prompt")) {
    customElements.define("profile-prompt", ProfilePrompt);
  }

  onMount(async () => {
    try {
      
   
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "mtcs");
    store = new ProfilesStore(new ProfilesClient(client, "form_test"));

    client.on("signal", async (signal) => {
      if (signal.zome_name !== "form") return;
      const payload = signal.payload as FormSignal;
      if (payload.type !== "ObligationProposed") return;
      console.log('Obligation Proposal Signal Received: ', signal)

      const actionHash = await desirializeData(payload.action);
      const obligationRecord: Obligation = await fetchObligation(actionHash);

      proposedObligations = [...proposedObligations, obligationRecord];
    });
  } catch (error) {
      console.error(error)
    }
    loading = false;
  });
  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  {#if loading}
    <div
      style="display: flex; flex: 1; align-items: center; justify-content: center"
    >
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <profiles-context {store}>
      <profile-prompt>
        <my-profile />
        <div
          id="content"
          style="display: flex; flex-direction: column; flex: 1;"
        >


          <AllObligations />

          <CreateObligation
            debtor={client.myPubKey}
            creator={client.myPubKey}
          />
          <!-- {#if proposedObligations}

          <div>TEST: {proposedObligations[0].amount}</div>
          {/if} -->

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
