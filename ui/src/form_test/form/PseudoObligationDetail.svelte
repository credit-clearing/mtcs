<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { PseudoObligation } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let pseudoObligationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let pseudoObligation: PseudoObligation | undefined;


  
$:  error, loading, record, pseudoObligation;

onMount(async () => {
  if (pseudoObligationHash === undefined) {
    throw new Error(`The pseudoObligationHash input is required for the PseudoObligationDetail element`);
  }
  await fetchPseudoObligation();
});

async function fetchPseudoObligation() {
  loading = true;
  error = undefined;
  record = undefined;
  pseudoObligation = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'get_pseudo_obligation',
      payload: pseudoObligationHash,
    });
    if (record) {
      pseudoObligation = decode((record.entry as any).Present.entry) as PseudoObligation;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

</script>


{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the pseudo obligation: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Amount:</strong></span>
    <span style="white-space: pre-line">{ pseudoObligation.amount }</span>
  </div>

</div>
{/if}

