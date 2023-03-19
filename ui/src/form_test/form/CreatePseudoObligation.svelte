<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { PseudoObligation } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-slider';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let creditor!: AgentPubKey;

export let trail!: Array<ActionHash>;


let amount: number = 0.0;

let errorSnackbar: Snackbar;

$: amount, creditor, trail;
$: isPseudoObligationValid = true && true;

onMount(() => {
  if (creditor === undefined) {
    throw new Error(`The creditor input is required for the CreatePseudoObligation element`);
  }
  if (trail === undefined) {
    throw new Error(`The trail input is required for the CreatePseudoObligation element`);
  }
});

async function createPseudoObligation() {  
  const pseudoObligationEntry: PseudoObligation = { 
    amount: amount!,
    creditor: creditor!,
    trail: trail,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'form_test',
      zome_name: 'form',
      fn_name: 'create_pseudo_obligation',
      payload: pseudoObligationEntry,
    });
    dispatch('pseudo-obligation-created', { pseudoObligationHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the pseudo obligation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create PseudoObligation</span>
  

  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Amount</span>
    
      <mwc-slider value={ amount } on:input={e => { amount = e.detail.value; } }></mwc-slider>
    </div>          
  </div>
            

  <mwc-button 
    raised
    label="Create PseudoObligation"
    disabled={!isPseudoObligationValid}
    on:click={() => createPseudoObligation()}
  ></mwc-button>
</div>
