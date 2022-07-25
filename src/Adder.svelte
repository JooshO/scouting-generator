<script lang="ts">
  export let add: (
    label: string,
    type: number,
    section: string,
    options: string[]
  ) => void;
  export let save: () => void;

  let label: string;
  let type: number;
  let section: string;
  let optionsStr: string = "";

  function addHelper() {
    if (optionsStr != "") {
      add(label, type, section, optionsStr.split(","));
    } else {
      add(label, type, section, [""]);
    }

    optionsStr = "";
  }

  let sections = [
    { text: "Prematch", value: "prematch" },
    { text: "Autonomous", value: "auton" },
    { text: "Teleoperated", value: "teleop" },
    { text: "Endgame", value: "endgame" },
  ];

  let types = [
    { text: "Number", value: 0 },
    { text: "Text", value: 2 },
    { text: "Number Inc", value: 1 },
    { text: "Select", value: 3 },
    { text: "Checkbox", value: 4 },
  ];
</script>

<div class="box">
  <input bind:value={label} placeholder="label" />
  <select bind:value={type}>
    {#each types as type}
      <option value={type.value}>
        {type.text}
      </option>
    {/each}
  </select>
  <select bind:value={section}>
    {#each sections as section}
      <option value={section.value}>
        {section.text}
      </option>
    {/each}
  </select>
  {#if type == 3}
    <input bind:value={optionsStr} placeholder="Options (comma seperated)" />
  {/if}
  <div>
    <button on:click={addHelper}>Add</button>
    <button on:click={save}>Save</button>
  </div>
</div>

<style>
  .box {
    border: 0.2em solid #ff0b15;
    padding: 0.3em;
    display: flex;
    flex-direction: column;
    justify-content: left;
  }

  button {
    background-color: #ff0b15;
    color: white;
  }

  button:hover {
    background-color: #cc0810;
  }
</style>
