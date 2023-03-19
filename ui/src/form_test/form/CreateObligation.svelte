<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import {
    type AppAgentClient,
    type Record,
    type EntryHash,
    type AgentPubKey,
    type ActionHash,
    type DnaHash,
    encodeHashToBase64,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { Obligation } from "./types";
  import "@material/mwc-button";
  import "@material/mwc-snackbar";
  import type { Snackbar } from "@material/mwc-snackbar";
  import "@material/mwc-textfield";

  import "@material/mwc-slider";
  import "@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js";

  import {
    ProfileDetail,
    ProfilesClient,
    ProfilesContext,
    ProfilesStore,
    SearchAgent,
  } from "@holochain-open-dev/profiles";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  export let debtor!: AgentPubKey;
  export let creator!: AgentPubKey;

  let amount: number = 0.0;
  let datetime: number = Date.now();

  let errorSnackbar: Snackbar;

  $: amount, debtor, datetime, creator;
  $: isObligationValid = true && true  && true;

  if (!customElements.get("profiles-context")) {
    customElements.define("profiles-context", ProfilesContext);
  }

  if (!customElements.get("search-agent")) {
    customElements.define("search-agent", SearchAgent);
  }

  if (!customElements.get("profile-detail")) {
    customElements.define("profile-detail", ProfileDetail);
  }

  let store = undefined;

  onMount(() => {
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateObligation element`
      );
    }

    store = new ProfilesStore(new ProfilesClient(client, "form_test"));
  });

  async function createObligation() {
    const obligationEntry: Obligation = {
      amount: amount!,
      debtor: debtor!,
      datetime: datetime!,
      creator: creator!,
      approved: false,
    };

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: "form_test",
        zome_name: "form",
        fn_name: "create_obligation",
        payload: obligationEntry,
      });
      dispatch("obligation-created", {
        obligationHash: record.signed_action.hashed.hash,
      });
    } catch (e) {
      errorSnackbar.labelText = `Error creating the obligation: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />
<div style="display: flex; flex-direction: column">
  <span style="margin-bottom: 32px; font-size: 18px">Create Obligation</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Amount"
      value={amount}
      on:input={(e) => {
        amount = parseFloat(e.target.value);
      }}
      required
    />
  </div>

  <div style="margin-bottom: 16px">
    <profiles-context {store}>
      <div>
        <search-agent
          field-label="Debtor"
          on:agent-selected={(data) => {
            debtor = data.detail.agentPubKey;
          }}
        />

        <span>
          Selected agent public key: {encodeHashToBase64(debtor)}
        </span>
        <br />
        <profile-detail />
      </div>
    </profiles-context>
  </div>


  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker
      label="Datetime"
      value={new Date(datetime / 1000).toISOString()}
      on:change={(e) => {
        datetime = new Date(e.target.value).valueOf() * 1000;
      }}
      required
    />
  </div>

  <mwc-button
    raised
    label="Create Obligation"
    disabled={!isObligationValid}
    on:click={() => createObligation()}
  />
</div>
