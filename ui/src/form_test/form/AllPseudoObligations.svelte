<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PseudoObligationDetail from './PseudoObligationDetail.svelte';
import type { FormSignal } from './types';

export let author: AgentPubKey;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
    if (author === undefined) {
      throw new Error(`The author input is required for the AllPseudoObligations element`);
    }


  await fetchPseudoObligations();
  client.on('signal', signal => {
    if (signal.zome_name !== 'form') return;
    const payload = signal.payload as FormSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'PseudoObligation') return;
    if (author.toString() !== client.myPubKey.toString()) return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function findPseudoObligations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'find_next_obligations',
      payload: author,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

async function fetchPseudoObligations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'get_all_pseudo_obligations',
      payload: author,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
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
<span>Error fetching the pseudo obligations: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No pseudo obligations found for this author.</span>
{:else}
<button on:click={findPseudoObligations}>Find Meta Obligations</button>

<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <PseudoObligationDetail pseudoObligationHash={hash}  on:pseudo-obligation-deleted={() => fetchPseudoObligations()}></PseudoObligationDetail>
    </div>
  {/each}
</div>
{/if}

