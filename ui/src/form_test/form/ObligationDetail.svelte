<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Obligation } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditObligation from './EditObligation.svelte'; 

const dispatch = createEventDispatcher();

export let obligationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let obligation: Obligation | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, obligation;

onMount(async () => {
  if (obligationHash === undefined) {
    throw new Error(`The obligationHash input is required for the ObligationDetail element`);
  }
  await fetchObligation();
});

async function fetchObligation() {
  loading = true;
  error = undefined;
  record = undefined;
  obligation = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'get_obligation',
      payload: obligationHash,
    });
    if (record) {
      obligation = decode((record.entry as any).Present.entry) as Obligation;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteObligation() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'delete_obligation',
      payload: obligationHash,
    });
    dispatch('obligation-deleted', { obligationHash: obligationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the obligation: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the obligation: {error.data.data}</span>
{:else if editing}
<EditObligation
  originalObligationHash={ obligationHash}
  currentRecord={record}
  on:obligation-updated={async () => {
    editing = false;
    await fetchObligation()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditObligation>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteObligation()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Amount:</strong></span>
    <span style="white-space: pre-line">{ obligation.amount }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Attachment:</strong></span>
    <span style="white-space: pre-line">{ obligation.attachment }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Datetime:</strong></span>
    <span style="white-space: pre-line">{ new Date(obligation.datetime / 1000).toLocaleString() }</span>
  </div>

</div>
{/if}

