<script lang="ts">
    import { onMount } from 'svelte';
  
    export let agentPublicKey: string;
  
    let message: string = '';
  
    function copyToClipboard() {
      navigator.clipboard.writeText(agentPublicKey.toString()) ;
      message = 'Your agent key has been copied to your clipboard.';
      setTimeout(() => {
        message = '';
      }, 3000);
    }
  </script>
  
  <div style="display: flex; align-items: center;">
    <p style="margin-right: 8px;">My Public Key:</p>
    <button on:click={copyToClipboard}>Copy to Clipboard</button>
    {#if message !== ''}
      <p>{message}</p>
    {/if}
  </div>