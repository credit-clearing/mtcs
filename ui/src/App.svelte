<script lang="ts">
  import { onMount, setContext } from "svelte";
  import {
    encodeHashToBase64,
    type ActionHash,
    type AppAgentClient,
    type Record as HRecord,
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

  let client: AppAgentClient | undefined;
  let loading = true;
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

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "mtcs");
  
    console.log(client);
    client.on("signal", async (signal) => {
      console.log('Signal Received: ', signal)
      if (signal.zome_name !== "form") return;
      const payload = signal.payload as FormSignal;
      if (payload.type !== "ObligationProposed") return;

      const actionHash = await desirializeData(payload.action);
      const obligationRecord: Obligation = await fetchObligation(actionHash);

      proposedObligations = [...proposedObligations, obligationRecord];
    });

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
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <MyPubKey agentPublicKey={encodeHashToBase64(client.myPubKey)} />

      <AllObligations />

      <CreateObligation debtor={client.myPubKey} creator={client.myPubKey} />

      {#if proposedObligations}
      <div>PROPOSED OBLIGATION: {proposedObligations[0].debtor}</div>

      {/if}



      <h2>EDIT ME! Add the components of your app here.</h2>

      <span
        >Look in the <code>ui/src/DNA/ZOME</code> folders for UI elements that
        are generated with <code>hc scaffold entry-type</code>,
        <code>hc scaffold collection</code>
        and <code>hc scaffold link-type</code> and add them here as appropriate.</span
      >

      <span
        >For example, if you have scaffolded a "todos" dna, a "todos" zome, a
        "todo_item" entry type, and a collection called "all_todos", you might
        want to add an element here to create and list your todo items, with the
        generated <code>ui/src/todos/todos/AllTodos.svelte</code> and
        <code>ui/src/todos/todos/CreateTodo.svelte</code> elements.</span
      >

      <span>So, to use those elements here:</span>
      <ol>
        <li>
          Import the elements with:
          <pre>
import AllTodos from './todos/todos/AllTodos.svelte';
import CreateTodo from './todos/todos/CreateTodo.svelte';
        </pre>
        </li>
        <li>
          Replace this "EDIT ME!" section with <code
            >&lt;CreateTodo&gt;&lt;/CreateTodo&gt;&lt;AllTodos&gt;&lt;/AllTodos&gt;</code
          >.
        </li>
      </ol>
    </div>
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
