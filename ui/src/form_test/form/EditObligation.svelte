<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type {
    AppAgentClient,
    Record,
    EntryHash,
    AgentPubKey,
    DnaHash,
    ActionHash,
  } from "@holochain/client";
  import { decode } from "@msgpack/msgpack";
  import { clientContext } from "../../contexts";
  import type { Obligation } from "./types";
  import "@material/mwc-button";
  import "@material/mwc-snackbar";
  import type { Snackbar } from "@material/mwc-snackbar";

  import "@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js";
  import "@material/mwc-slider";
  import "@material/mwc-textfield";
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  export let originalObligationHash!: ActionHash;

  export let currentRecord!: Record;
  let currentObligation: Obligation = decode(
    (currentRecord.entry as any).Present.entry
  ) as Obligation;

  let amount: number | undefined = currentObligation.amount;
  let datetime: number | undefined = currentObligation.datetime;

  let errorSnackbar: Snackbar;

  $: amount, datetime;
  $: isObligationValid = true && true && true;

  onMount(() => {
    if (currentRecord === undefined) {
      throw new Error(
        `The currentRecord input is required for the EditObligation element`
      );
    }
    if (originalObligationHash === undefined) {
      throw new Error(
        `The originalObligationHash input is required for the EditObligation element`
      );
    }
  });

  async function updateObligation() {
    const obligation: Obligation = {
      amount: amount!,
      datetime: datetime!,
      debtor: currentObligation.debtor,
      creator: currentObligation.creator,
      approved: true
    };

    try {
      const updateRecord: Record = await client.callZome({
        cap_secret: null,
        role_name: "form_test",
        zome_name: "form",
        fn_name: "update_obligation",
        payload: {
          original_obligation_hash: originalObligationHash,
          previous_obligation_hash: currentRecord.signed_action.hashed.hash,
          updated_obligation: obligation,
        },
      });

      dispatch("obligation-updated", {
        actionHash: updateRecord.signed_action.hashed.hash,
      });
    } catch (e) {
      errorSnackbar.labelText = `Error updating the obligation: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Obligation</span>

  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Amount</span>

      <mwc-slider
        value={amount}
        on:input={(e) => {
          amount = e.detail.value;
        }}
      />
    </div>
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

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch("edit-canceled")}
      style="flex: 1; margin-right: 16px"
    />
    <mwc-button
      raised
      label="Save"
      disabled={!isObligationValid}
      on:click={() => updateObligation()}
      style="flex: 1;"
    />
  </div>
</div>
