<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import ObligationDetail from './ObligationDetail.svelte';
import type { FormSignal } from './types';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchObligations();
  client.on('signal', signal => {
    if (signal.zome_name !== 'form') return;
    const payload = signal.payload as FormSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Obligation') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchObligations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'get_all_obligations',
      payload: null,
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
<span>Error fetching the obligations: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No obligations found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <ObligationDetail obligationHash={hash}  on:obligation-deleted={() => fetchObligations()}></ObligationDetail>
    </div>
  {/each}
</div>
{/if}

