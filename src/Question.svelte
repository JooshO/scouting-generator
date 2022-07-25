<script lang="ts">
  export let label: string;
  export let options: string[];
  export let type: number;
  export let section: string;
  export let onDelete: (label: string, section: string) => void;
  export let deletable: boolean;

  const typeMap = ["Number", "Number Inc", "Text", "Select", "Checkbox"];

  function onDeleteHelper() {
    if (onDelete != null) {
      onDelete(label, section);
    }
  }
</script>

<div class="box">
  <div class="grid-container">{label}</div>
  <div class="grid-container">{typeMap[type]}</div>
  <div class="grid-container">{options}</div>
  {#if deletable}
    <button class="grid-container" on:click={onDeleteHelper}> X </button>
  {/if}
</div>

<style>
  .box {
    width: inherit;
    border: 1px solid #aaa;
    height: fit-content;
    border-radius: 2px;
    box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1);
    margin: 0.5em 0.5em 0.5em 0.5em;
    display: grid;
    grid-auto-flow: row;
    grid-template-columns: 25% 25% 25% 25%;
    min-height: 3em;
  }

  button {
    margin: 0.5em 0.5em 0.5em 0.5em;
    background-color: #ff0b15;
    color: white;

    width: 2em;
    height: 2em;
  }

  button:hover {
    background-color: #cc0810;
  }

  .grid-container {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
