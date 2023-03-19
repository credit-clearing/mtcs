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
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  export let debtor!: AgentPubKey;

  export let creator!: AgentPubKey;

  let amount: number = 0.0;
  let attachment: string = "";
  let datetime: number = Date.now();

  let errorSnackbar: Snackbar;

  $: amount, debtor, attachment, datetime, creator;
  $: isObligationValid = true && true && attachment !== "" && true;

  onMount(() => {
    if (debtor === undefined) {
      throw new Error(
        `The debtor input is required for the CreateObligation element`
      );
    }
    if (creator === undefined) {
      throw new Error(
        `The creator input is required for the CreateObligation element`
      );
    }
  });

  async function createObligation() {
    const obligationEntry: Obligation = {
      amount: amount!,
      debtor: debtor!,
      attachment: attachment!,
      datetime: datetime!,
      creator: creator!,
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
        amount = e.target.value;
      }}
      required
    />
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Debtor"
      value={encodeHashToBase64(debtor)}
      on:input={(e) => {
        debtor = e.target.value;
      }}
      required
    />
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Attachment"
      value={attachment}
      on:input={(e) => {
        attachment = e.target.value;
      }}
      required
    />
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
